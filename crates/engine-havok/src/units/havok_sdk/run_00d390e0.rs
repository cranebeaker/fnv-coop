//! `Havok SDK/run_00d390e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d390e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, four words, the first never read: `_unused_1`, `record`, `receiver`,
/// `argument`; descriptions, not confirmed): for each of the three 16-bit slots at `record + 0`,
/// `+2`, `+4` that is not `0xffff`, calls slot `0x14` of `receiver` with `(slot value,
/// argument)` and stores `0xffff` in the slot. Answers `record + 0x20`.
pub fn fn_00d390e0(
    e: &mut Engine,
    _unused_1: u32,
    record: u32,
    receiver: u32,
    argument: u32,
) -> u32 {
    for slot in 0..3u32 {
        let at = record.wrapping_add(slot * 2);
        let value = e.mem.u16(at);
        if value != 0xffff {
            e.vcall(receiver, 0x14, &args![u32::from(value), argument]);
            e.mem.set_u16(at, 0xffff);
        }
    }
    record.wrapping_add(0x20)
}

/// The float at `0104da48` (`0x7f7fffee`), passed on as a bit pattern after the `FLD`/`FSTP` copy.
const LARGEST_SINGLE: u32 = 0x0104_da48;

/// `a * M` for the vector `v` and the matrix rows `m`: `(y * row1 + x * row0) + (z * row2 + row3)`
/// with the lane products and sums in the order of the `MULPS`/`ADDPS` sequence.
fn transform_00d39150(v: [f32; 4], m: [[f32; 4]; 4]) -> [f32; 4] {
    [0usize, 1, 2, 3].map(|lane| {
        let first = v[1] * m[1][lane] + v[0] * m[0][lane];
        let second = v[2] * m[2][lane] + m[3][lane];
        first + second
    })
}

fn read_lanes_00d39150(e: &Engine, at: u32) -> [f32; 4] {
    [0u32, 1, 2, 3].map(|lane| e.mem.f32(at.wrapping_add(lane * 4)))
}

fn write_lanes_00d39150(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (lane, word) in value.into_iter().enumerate() {
        e.mem
            .set_u32(at.wrapping_add(lane as u32 * 4), word.to_bits());
    }
}

// Translated from 00d39150 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, four words, the second never read: `job`, `_unused_2`, `settings`,
/// `result`; descriptions, not confirmed): `job` holds two pointers `p1` and `p2`; `*p1` and
/// `*p2` are objects (`q1`, `q2`) with a 16-lane (4 x 4 float) matrix at `p1 + 8` / `p2 + 8`.
/// Transforms the vectors at `q1 + 0x20`, `q1 + 0x30` and at `q2 + 0x20`, `q2 + 0x30`,
/// `q2 + 0x40` by those matrices (`(y * row1 + x * row0) + (z * row2 + row3)`) into a frame of the
/// function, and calls `00cd6520` (cdecl, nine words) with the first pair's address, the float at
/// `q1 + 0x10`, the second set's address, the float at `q2 + 0x10`, `settings + 8`, the float at
/// `0104da48`, 0, the address of an output area of the frame and 0. That call fills the area; the
/// 16 bytes at area `+0x10` or `+0x30` are copied to `result`: the first when the float at area
/// `+0x3c` is above the one at `+0x1c`, else the second (a NaN counts as not above).
pub fn fn_00d39150(e: &mut Engine, job: Ptr, _unused_2: u32, settings: u32, result: u32) {
    let (first_pointer, second_pointer) = (e.mem.u32(job.addr()), e.mem.u32(job.addr() + 4));
    let second_object = e.mem.u32(second_pointer);
    let first_object = e.mem.u32(first_pointer);
    let first_matrix_address = e.mem.u32(first_pointer + 8);
    let first_matrix = [0u32, 0x10, 0x20, 0x30]
        .map(|offset| read_lanes_00d39150(e, first_matrix_address + offset));
    e.with_stack(0x100, |e, frame| {
        let t = frame.addr();
        for (index, offset) in [0x20u32, 0x30].into_iter().enumerate() {
            let v = read_lanes_00d39150(e, first_object + offset);
            let moved = transform_00d39150(v, first_matrix);
            write_lanes_00d39150(e, t + 0x50 + index as u32 * 0x10, moved);
        }
        let second_matrix_address = e.mem.u32(second_pointer + 8);
        let second_matrix = [0u32, 0x10, 0x20, 0x30]
            .map(|offset| read_lanes_00d39150(e, second_matrix_address + offset));
        for (index, offset) in [0x20u32, 0x30, 0x40].into_iter().enumerate() {
            let v = read_lanes_00d39150(e, second_object + offset);
            let moved = transform_00d39150(v, second_matrix);
            write_lanes_00d39150(e, t + 0xd0 + index as u32 * 0x10, moved);
        }
        let large = x87_quiet_bits(e.mem.u32(LARGEST_SINGLE));
        let second_radius = x87_quiet_bits(e.mem.u32(second_object + 0x10));
        let first_radius = x87_quiet_bits(e.mem.u32(first_object + 0x10));
        e.call(
            0x00cd_6520,
            &args![
                t + 0x50,
                first_radius,
                t + 0xd0,
                second_radius,
                settings.wrapping_add(8),
                large,
                0u32,
                t + 0x70,
                0u32
            ],
        );
        let upper = x87_load32(e.mem.u32(t + 0xac));
        let lower = x87_load32(e.mem.u32(t + 0x8c));
        let source = if upper > lower { t + 0x80 } else { t + 0xa0 };
        let chosen = [0u32, 1, 2, 3].map(|lane| e.mem.u32(source + lane * 4));
        for (lane, word) in chosen.into_iter().enumerate() {
            e.mem.set_u32(result + lane as u32 * 4, word);
        }
    });
}

/// `00cd72a0` (cdecl, two words; Ghidra made it a function of two float pointers).
const FILL_FROM_SHAPE: u32 = 0x00cd_72a0;

// Translated from 00d39030 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words, the first never read: `_unused_0`, `record`, `value`; descriptions, not confirmed; the address is stored at offset `+0xc` of the function words that `00d39840` registers): looks through the three 16-bit slots at `record + 0`, `+2` and `+4` for the first one equal to the low 16 bits of `value`, stores `0xffff` in it and answers its index; answers 3 when none matches.
pub fn fn_00d39030(e: &mut Engine, _unused_0: u32, record: u32, value: u32) -> u32 {
    let wanted = value as u16;
    for index in 0..3u32 {
        let at = record.wrapping_add(index * 2);
        if e.mem.u16(at) == wanted {
            e.mem.set_u16(at, 0xffff);
            return index;
        }
    }
    3
}

// Translated from 00d39060 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words, the first never read: `_unused_0`, `record`, `value`; descriptions, not confirmed; the address is stored at offset `+0x10` of the function words that `00d39840` registers): looks through the three 16-bit slots at `record + 0`, `+2` and `+4` for the first one equal to `0xffff`, stores the low 16 bits of `value` in it and answers its index; answers 3 when none is free.
pub fn fn_00d39060(e: &mut Engine, _unused_0: u32, record: u32, value: u32) -> u32 {
    for index in 0..3u32 {
        let at = record.wrapping_add(index * 2);
        if e.mem.u16(at) == 0xffff {
            e.mem.set_u16(at, value as u16);
            return index;
        }
    }
    3
}

// Translated from 00d39090 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the address is stored at offset `+0x14` of the function words that `00d39840` registers): reads no argument and only returns; how many words its callers push is not confirmed.
pub fn fn_00d39090(_e: &mut Engine) {}

// Translated from 00d390a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words: `input`, `entry`, `record`; descriptions, not confirmed; the address is stored at offset `+0` of the function words that `00d39840` registers). Takes the word that the pointer at `input + 4` points to, clears the byte at `entry + 2`, stores `0xffff` in the 16-bit values at `record + 0`, `+2` and `+4`, runs `00cd72a0` (cdecl, two words) with that word plus 0x20 and `record + 8`, and answers `record + 0x20`.
pub fn fn_00d390a0(e: &mut Engine, input: u32, entry: u32, record: u32) -> u32 {
    let holder = e.mem.u32(input + 4);
    let base = e.mem.u32(holder);
    e.mem.set_u8(entry + 2, 0);
    e.mem.set_u16(record + 2, 0xffff);
    e.mem.set_u16(record, 0xffff);
    e.mem.set_u16(record + 4, 0xffff);
    e.call(FILL_FROM_SHAPE, &args![base + 0x20, record + 8]);
    record + 0x20
}

// Translated from 00d39130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, four words, handed unchanged to [`fn_00d390e0`], whose answer it returns; the address is stored at offset `+4` of the function words that `00d39840` registers).
pub fn fn_00d39130(e: &mut Engine, word_1: u32, word_2: u32, word_3: u32, word_4: u32) -> u32 {
    fn_00d390e0(e, word_1, word_2, word_3, word_4)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_90e0, fn_00d390e0(u32, u32, u32, u32) -> u32),
        entry!(0x00d3_9150, fn_00d39150(Ptr, u32, u32, u32)),
        entry!(0x00d39030, fn_00d39030(u32, u32, u32) -> u32),
        entry!(0x00d39060, fn_00d39060(u32, u32, u32) -> u32),
        entry!(0x00d39090, fn_00d39090()),
        entry!(0x00d390a0, fn_00d390a0(u32, u32, u32) -> u32),
        entry!(0x00d39130, fn_00d39130(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d390e0_notifies_for_each_used_slot_and_clears_it() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x14, 0x0900_00c1);
        let receiver = e.mem.alloc(0x10);
        e.mem.set_u32(receiver, vtable);
        e.register(0x0900_00c1, |_, _| Ret::default());
        let record = e.mem.alloc(0x30);
        e.mem.set_u16(record, 5);
        e.mem.set_u16(record + 2, 0xffff);
        e.mem.set_u16(record + 4, 0x1234);
        e.mem.set_u16(record + 6, 9);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00d3_90e0, &args![0u32, record, receiver, 0x77u32])
            .u32();
        assert_eq!(result, record + 0x20);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0900_00c1, vec![receiver, 5, 0x77]),
                (0x0900_00c1, vec![receiver, 0x1234, 0x77])
            ]
        );
        assert_eq!(e.mem.u16(record), 0xffff);
        assert_eq!(e.mem.u16(record + 2), 0xffff);
        assert_eq!(e.mem.u16(record + 4), 0xffff);
        // The fourth slot is not looked at.
        assert_eq!(e.mem.u16(record + 6), 9);
    }

    /// Two jobs with a translation matrix: vectors `(1, 2, 3, 0)` become `(1 + tx, 2 + ty, 3 + tz,
    /// 1)`. `answer` is what the double of `00cd6520` writes at the output area's `+0x1c` / `+0x3c`
    /// as the weights of its two vectors.
    fn run(upper: f32, lower: f32) -> ([f32; 4], Vec<u32>, u32) {
        let mut e = Engine::new();
        e.map(0x0104_d000, 0x1000);
        e.mem.set_u32(LARGEST_SINGLE, 0x7f7f_ffee);
        let matrix = |e: &mut Engine, shift: [f32; 3]| {
            let at = e.mem.alloc(0x40);
            for row in 0..4u32 {
                let mut lanes = [0.0f32; 4];
                if row < 3 {
                    lanes[row as usize] = 1.0;
                } else {
                    lanes = [shift[0], shift[1], shift[2], 1.0];
                }
                write_lanes_00d39150(e, at + row * 0x10, lanes);
            }
            at
        };
        let first_matrix = matrix(&mut e, [10.0, 20.0, 30.0]);
        let second_matrix = matrix(&mut e, [100.0, 200.0, 300.0]);
        let object = |e: &mut Engine, radius: f32, vectors: usize| {
            let at = e.mem.alloc(0x60);
            e.mem.set_f32(at + 0x10, radius);
            for index in 0..vectors {
                write_lanes_00d39150(e, at + 0x20 + index as u32 * 0x10, [1.0, 2.0, 3.0, 0.0]);
            }
            at
        };
        let first_object = object(&mut e, 0.5, 2);
        let second_object = object(&mut e, 0.25, 3);
        let first_pointer = e.mem.alloc(0x10);
        e.mem.set_u32(first_pointer, first_object);
        e.mem.set_u32(first_pointer + 8, first_matrix);
        let second_pointer = e.mem.alloc(0x10);
        e.mem.set_u32(second_pointer, second_object);
        e.mem.set_u32(second_pointer + 8, second_matrix);
        let job = e.mem.alloc(0x10);
        e.mem.set_u32(job, first_pointer);
        e.mem.set_u32(job + 4, second_pointer);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u32>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cd_6520, move |e, a| {
            let mut words = a.to_vec();
            for index in 0..2 {
                for lane in 0..4 {
                    words.push(e.mem.u32(a[0] + index * 0x10 + lane * 4));
                }
            }
            for index in 0..3 {
                for lane in 0..4 {
                    words.push(e.mem.u32(a[2] + index * 0x10 + lane * 4));
                }
            }
            // The output area: two vectors with their weights in the fourth lane.
            for lane in 0..4 {
                e.mem.set_f32(a[7] + 0x10 + lane * 4, 1.0 + lane as f32);
                e.mem.set_f32(a[7] + 0x30 + lane * 4, 11.0 + lane as f32);
            }
            e.mem.set_f32(a[7] + 0x1c, lower);
            e.mem.set_f32(a[7] + 0x3c, upper);
            *log.borrow_mut() = words;
            Ret::default()
        });
        let settings = e.mem.alloc(0x20);
        let result = e.mem.alloc(0x10);
        e.call(0x00d3_9150, &args![job, 0u32, settings, result]);
        let words = seen.borrow().clone();
        (read_lanes_00d39150(&e, result), words, settings)
    }

    #[test]
    fn fn_00d39150_picks_the_vector_with_the_smaller_weight() {
        // upper > lower: the vector at +0x10 (weights lower), else the one at +0x30.
        assert_eq!(run(5.0, 2.0).0, [1.0, 2.0, 3.0, 2.0]);
        assert_eq!(run(2.0, 5.0).0, [11.0, 12.0, 13.0, 2.0]);
        assert_eq!(run(2.0, 2.0).0, [11.0, 12.0, 13.0, 2.0]);
        assert_eq!(run(f32::NAN, 2.0).0[0], 11.0);
    }

    #[test]
    fn fn_00d39150_transforms_the_vectors_and_passes_the_constants() {
        let (_, words, settings) = run(5.0, 2.0);
        // (&A0, first radius, &B0, second radius, settings + 8, large float, 0, &area, 0).
        assert_eq!(words[1], 0.5f32.to_bits());
        assert_eq!(words[3], 0.25f32.to_bits());
        assert_eq!(words[5], 0x7f7f_ffee);
        assert_eq!((words[6], words[8]), (0, 0));
        assert_eq!(words[4], settings + 8);
        let lanes = |offset: usize| -> Vec<f32> {
            (0..4)
                .map(|lane| f32::from_bits(words[9 + offset + lane]))
                .collect()
        };
        // First object, two vectors moved by (10, 20, 30); second, three by (100, 200, 300).
        assert_eq!(lanes(0), vec![11.0, 22.0, 33.0, 1.0]);
        assert_eq!(lanes(4), vec![11.0, 22.0, 33.0, 1.0]);
        assert_eq!(lanes(8), vec![101.0, 202.0, 303.0, 1.0]);
        assert_eq!(lanes(12), vec![101.0, 202.0, 303.0, 1.0]);
        assert_eq!(lanes(16), vec![101.0, 202.0, 303.0, 1.0]);
    }
}

#[cfg(test)]
mod tests_a07_gap {
    use super::*;

    /// The argument words of every call to `address` in the engine's call log (which is taken).
    fn calls_to(e: &mut Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn fn_00d39030_frees_the_slot_that_holds_the_value() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x20);
        for (index, value) in [0x1111u16, 0x2222, 0x3333].into_iter().enumerate() {
            e.mem.set_u16(record + 2 * index as u32, value);
        }
        // Only the low 16 bits of the value count.
        assert_eq!(
            e.call(0x00d3_9030, &args![0u32, record, 0xabcd_2222u32])
                .u32(),
            1
        );
        assert_eq!(e.mem.u16(record + 2), 0xffff);
        assert_eq!(e.mem.u16(record), 0x1111);
        assert_eq!(e.mem.u16(record + 4), 0x3333);
        assert_eq!(
            e.call(0x00d3_9030, &args![0u32, record, 0x4444u32]).u32(),
            3
        );
        assert_eq!(
            e.call(0x00d3_9030, &args![0u32, record, 0x3333u32]).u32(),
            2
        );
        assert_eq!(e.mem.u16(record + 4), 0xffff);
    }

    #[test]
    fn fn_00d39060_fills_the_first_free_slot() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x20);
        for (index, value) in [0x1111u16, 0xffff, 0xffff].into_iter().enumerate() {
            e.mem.set_u16(record + 2 * index as u32, value);
        }
        assert_eq!(
            e.call(0x00d3_9060, &args![0u32, record, 0xabcd_7777u32])
                .u32(),
            1
        );
        assert_eq!(e.mem.u16(record + 2), 0x7777);
        assert_eq!(
            e.call(0x00d3_9060, &args![0u32, record, 0x8888u32]).u32(),
            2
        );
        assert_eq!(e.mem.u16(record + 4), 0x8888);
        // Nothing is free now.
        assert_eq!(
            e.call(0x00d3_9060, &args![0u32, record, 0x9999u32]).u32(),
            3
        );
        assert_eq!(e.mem.u16(record), 0x1111);
    }

    #[test]
    fn fn_00d39090_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d3_9090, &args![]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00d390a0_clears_the_record_and_runs_the_fill() {
        let mut e = Engine::new();
        e.register(FILL_FROM_SHAPE, |_, _| Ret::default());
        let shape = e.mem.alloc(0x40);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder, shape);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 4, holder);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u8(entry + 2, 0x55);
        let record = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d3_90a0, &args![input, entry, record]).u32(),
            record + 0x20
        );
        assert_eq!(e.mem.u8(entry + 2), 0);
        for offset in [0, 2, 4] {
            assert_eq!(e.mem.u16(record + offset), 0xffff);
        }
        assert_eq!(
            calls_to(&mut e, FILL_FROM_SHAPE),
            vec![vec![shape + 0x20, record + 8]]
        );
    }

    #[test]
    fn fn_00d39130_hands_its_words_to_the_notifying_function() {
        let mut e = Engine::new();
        e.register(0x0900_00c1, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x14, 0x0900_00c1);
        let receiver = e.mem.alloc(0x10);
        e.mem.set_u32(receiver, vtable);
        let record = e.mem.alloc(0x30);
        for (index, value) in [5u16, 0xffff, 7].into_iter().enumerate() {
            e.mem.set_u16(record + 2 * index as u32, value);
        }
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d3_9130, &args![1u32, record, receiver, 9u32])
                .u32(),
            record + 0x20
        );
        assert_eq!(
            calls_to(&mut e, 0x0900_00c1),
            vec![vec![receiver, 5, 9], vec![receiver, 7, 9]]
        );
    }
}
