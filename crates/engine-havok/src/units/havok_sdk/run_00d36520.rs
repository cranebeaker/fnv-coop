//! `Havok SDK/run_00d36520` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::hkadeltacompressedanimation::profile_record;

/// The memory router (`00c85750`): its field `+0x10` is the allocator `00c908c0` grows an array
/// through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;

/// Both words of element `index` of an array of 8-byte pairs.
fn pair_00d36520(e: &Engine, array: u32, index: i32) -> (u32, u32) {
    let at = array.wrapping_add((index as u32).wrapping_mul(8));
    (e.mem.u32(at), e.mem.u32(at.wrapping_add(4)))
}

// Translated from 00d36520 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, four words `array`, `low`, `high`, `unused_4`): quicksort of the elements
/// `low..=high` of the array of 8-byte pairs at `array`, ordered by the first word and then the
/// second, both as unsigned numbers (the sibling of `00d1b1d0`, which orders by the first word
/// only). The fourth word is only passed on to the recursion. The pivot is the pair in the middle
/// of the range; the partition scans, swaps and recursion (left part recursed, right part looped)
/// are those of the code.
pub fn fn_00d36520(e: &mut Engine, array: Ptr, low: i32, high: i32, unused_4: u32) {
    let array = array.addr();
    let mut low = low;
    loop {
        let (mut i, mut j) = (low, high);
        let pivot = pair_00d36520(e, array, low.wrapping_add(high) >> 1);
        loop {
            while pair_00d36520(e, array, i) < pivot {
                i = i.wrapping_add(1);
            }
            while pivot < pair_00d36520(e, array, j) {
                j = j.wrapping_sub(1);
            }
            if j < i {
                break;
            }
            if j != i {
                let at = |index: i32| array.wrapping_add((index as u32).wrapping_mul(8));
                let (from, to) = (at(i), at(j));
                let (to_first, to_second) = (e.mem.u32(to), e.mem.u32(to + 4));
                let (from_first, from_second) = (e.mem.u32(from), e.mem.u32(from + 4));
                e.mem.set_u32(to, from_first);
                e.mem.set_u32(to + 4, from_second);
                e.mem.set_u32(from, to_first);
                e.mem.set_u32(from + 4, to_second);
            }
            j = j.wrapping_sub(1);
            i = i.wrapping_add(1);
            if i > j {
                break;
            }
        }
        if low < j {
            fn_00d36520(e, Ptr::new(array), low, j, unused_4);
        }
        if i < high {
            low = i;
        } else {
            return;
        }
    }
}

// Translated from 00d365d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, five words `info`, `second`, `third`, `fourth`, `fifth`; descriptions, not
/// confirmed): between two profiling records (`010ced28` and the end record `010c41f8`; the time
/// stamps are the host clock) builds three records in a 0x154-byte frame and calls `00d35770`.
/// `00d33260` copies `info` into the object at frame `+0x64` (whose words at `+0x60`, `+0x70` ..
/// `+0x7c` are then set from `info`); `00cf5640` makes an object from the address of the word
/// `**info` (frame `+8`, with `1` and `0`) on the object at frame `+0xe4`; a record at frame
/// `+0x14` is filled (the new object, `-1`, the words `+8` and `+0xc` of `*info`, flag bytes, and
/// the object `00d01a60` constructs at frame `+0x34`, replaced by the 0x2c bytes at `*info + 0x20`
/// when the word at `*info + 0xc` is 0, with the float at `01012054` at frame `+0x60`); its
/// address and `*info` go into the first object (`+0` and `+8`). `00d35770(first object, second,
/// third, fourth, fifth)` gives the result. Afterwards the new object's `hkArray` at `+0x18` has
/// its size cleared (a grow to twice the capacity, never reached, since the capacity masked with
/// `0x3fffffff` is never negative) and the new object's slot 0 is called with 0. Answers the
/// result of `00d35770`. The words of the frame the exe leaves uninitialised are zero here.
pub fn fn_00d365d0(
    e: &mut Engine,
    info: Ptr,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
) -> u32 {
    profile_record(e, 0x010c_ed28);
    let result = e.with_stack(0x154, |e, frame| {
        let s = frame.addr();
        let at = info.addr();
        e.call(0x00d3_3260, &args![s + 0x64, info]);
        let word_70 = e.mem.u32(at + 0x70);
        let word_60 = e.mem.u32(at + 0x60);
        let word_74 = e.mem.u32(at + 0x74);
        e.mem.set_u32(s + 0xd4, word_70);
        let word_7c = e.mem.u32(at + 0x7c);
        e.mem.set_u32(s + 0xc4, word_60);
        let word_78 = e.mem.u32(at + 0x78);
        e.mem.set_u32(s + 0xe0, word_7c);
        e.mem.set_u32(s + 0xd8, word_74);
        let holder = e.mem.u32(at);
        e.mem.set_u32(s + 0xdc, word_78);
        let first_word = e.mem.u32(holder);
        e.mem.set_u32(s + 8, first_word);
        let object = e
            .call(0x00cf_5640, &args![s + 0xe4, s + 8, 1u32, 0u32])
            .u32();
        e.mem.set_u32(s + 0x14, 0);
        e.mem.set_u32(s + 0x1c, 0);
        e.mem.set_u32(s + 0x20, 0);
        e.mem.set_u32(s + 0x18, 0xffff_ffff);
        e.mem.set_u8(s + 0x24, 0);
        e.mem.set_u32(s + 0x28, 0);
        e.mem.set_u8(s + 0x2c, 0);
        e.mem.set_u32(s + 0x30, 0);
        e.mem.set_u8(s + 0x2e, 0xff);
        e.mem.set_u8(s + 0x2d, 0x7f);
        e.call(0x00d0_1a60, &args![s + 0x34]);
        let minus_one = x87_quiet_bits(e.mem.u32(0x0101_2054));
        e.mem.set_u32(s + 0x60, minus_one);
        e.mem.set_u16(s + 0x26, 0);
        let holder = e.mem.u32(at);
        // `SUB DL, AL` of the low bytes of the addresses of frame `+0x14` and frame `+0x28`.
        let distance = ((s + 0x14) as u8).wrapping_sub((s + 0x28) as u8);
        e.mem.set_u8(s + 0x2d, distance);
        e.mem.set_u8(s + 0x25, 0);
        if e.mem.u32(holder + 0xc) == 0 {
            for word in 0..11 {
                let value = e.mem.u32(holder + 0x20 + word * 4);
                e.mem.set_u32(s + 0x34 + word * 4, value);
            }
        }
        let word_8 = e.mem.u32(holder + 8);
        let word_c = e.mem.u32(holder + 0xc);
        e.mem.set_u32(s + 0x1c, word_8);
        e.mem.set_u32(s + 0x6c, holder);
        e.mem.set_u32(s + 0x20, word_c);
        e.mem.set_u32(s + 0x64, s + 0x14);
        e.mem.set_u32(s + 0x14, object);
        e.mem.set_u32(s + 0x18, 0xffff_ffff);
        let result = e
            .call(0x00d3_5770, &args![s + 0x64, second, third, fourth, fifth])
            .u32();
        e.mem.set_u32(s + 0x10, result);
        let array = object + 0x18;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
        // The exe's `JGE` after the `AND`: never taken the other way, so this block is dead.
        if (capacity as i32) < 0 {
            let doubled = (capacity as i32).wrapping_add(capacity as i32);
            let wanted = if doubled <= 0 { 0 } else { doubled };
            e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 0x10u32]);
        }
        e.mem.set_u32(array + 4, 0);
        e.vcall(object, 0, &args![0u32]);
        result
    });
    profile_record(e, 0x010c_41f8);
    result
}

// Translated from 00d367b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `object`): calls `00da6fd0` with `object`, then stores the
/// function `0x00d365d0` at `object + 0x2c`.
pub fn fn_00d367b0(e: &mut Engine, object: Ptr) {
    e.call(0x00da_6fd0, &args![object]);
    e.mem.set_u32(object.addr() + 0x2c, 0x00d3_65d0);
}

// Translated from 00d367d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): builds a 0x40-byte record
/// on the stack (words `+0x18` .. `+0x28` and bytes `+0x30` .. `+0x32` cleared, the rest
/// initialised by `00d367b0`, which fills it through `00da6fd0` and sets its function at `+0x2c`;
/// zero where the exe leaves it uninitialised) and registers it with `00cfbd60` on `queue` with
/// `(record, 0x12, 7)`.
pub fn fn_00d367d0(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x40, |e, record| {
        let r = record.addr();
        e.mem.set_u8(r + 0x30, 0);
        e.mem.set_u8(r + 0x31, 0);
        e.mem.set_u8(r + 0x32, 0);
        for offset in [0x18u32, 0x1c, 0x20, 0x24, 0x28] {
            e.mem.set_u32(r + offset, 0);
        }
        fn_00d367b0(e, record);
        e.call(0x00cf_bd60, &args![queue, record, 0x12u32, 7u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_6520, fn_00d36520(Ptr, i32, i32, u32)),
        entry!(0x00d3_65d0, fn_00d365d0(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00d3_67b0, fn_00d367b0(Ptr)),
        entry!(0x00d3_67d0, fn_00d367d0(Ptr)),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00d36520_sorts_the_pairs_by_both_words() {
        let mut e = Engine::new();
        let pairs: [(u32, u32); 10] = [
            (5, 9),
            (0xffff_fff0, 1),
            (5, 2),
            (1, 7),
            (5, 0xffff_ffff),
            (3, 3),
            (5, 2),
            (0, 0xffff_0000),
            (3, 1),
            (7, 4),
        ];
        let array = e.mem.alloc(8 * pairs.len() as u32);
        for (index, (first, second)) in pairs.iter().enumerate() {
            e.mem.set_u32(array + 8 * index as u32, *first);
            e.mem.set_u32(array + 8 * index as u32 + 4, *second);
        }
        e.call(
            0x00d3_6520,
            &args![Ptr::<()>::new(array), 0i32, pairs.len() as i32 - 1, 0u32],
        );
        let sorted: Vec<(u32, u32)> = (0..pairs.len() as u32)
            .map(|index| {
                (
                    e.mem.u32(array + 8 * index),
                    e.mem.u32(array + 8 * index + 4),
                )
            })
            .collect();
        let mut expected = pairs.to_vec();
        expected.sort();
        assert_eq!(sorted, expected);
    }

    #[test]
    fn fn_00d36520_sorts_a_sub_range_only() {
        let mut e = Engine::new();
        let pairs = [(9u32, 9u32), (4, 1), (3, 2), (2, 3), (1, 4), (0, 0)];
        let array = e.mem.alloc(8 * pairs.len() as u32);
        for (index, (first, second)) in pairs.iter().enumerate() {
            e.mem.set_u32(array + 8 * index as u32, *first);
            e.mem.set_u32(array + 8 * index as u32 + 4, *second);
        }
        e.call(0x00d3_6520, &args![Ptr::<()>::new(array), 1i32, 4i32, 0u32]);
        let words: Vec<u32> = (0..12).map(|index| e.mem.u32(array + index * 4)).collect();
        assert_eq!(words, vec![9, 9, 1, 4, 2, 3, 3, 2, 4, 1, 0, 0]);
    }

    struct Scene {
        info: u32,
        object: u32,
        holder: u32,
        records: u32,
        /// What the double of `00d35770` saw: its five words, the first object's 0x20 words, the
        /// record at the first object's `+0` (0x14 words) and five words of the first object.
        seen: Rc<RefCell<Vec<u32>>>,
    }

    /// Doubles for every callee of `00d365d0`. `00d33260` and `00d01a60` leave their objects
    /// alone, `00cf5640` answers the new object, `00d35770` records what it is given and
    /// answers `0x1234`.
    fn install(e: &mut Engine, holder_word_c: u32) -> Scene {
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(0x0101_2054, 0xbf80_0000);
        e.set_global(0x0126_835c, 3u32);
        let stream = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x40);
        e.register_double(0x00fd_f044, move |_, a| {
            assert_eq!(a[0], 3);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        let info = e.mem.alloc(0x80);
        for word in 0..0x20 {
            e.mem.set_u32(info + word * 4, 0x3000 + word);
        }
        let holder = e.mem.alloc(0x80);
        e.mem.set_u32(info, holder);
        e.mem.set_u32(holder, 0x5000);
        e.mem.set_u32(holder + 8, 0x5008);
        e.mem.set_u32(holder + 0xc, holder_word_c);
        for word in 0..11 {
            e.mem.set_u32(holder + 0x20 + word * 4, 0x6000 + word);
        }
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable, 0x0900_00a1);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 0x1c, 9);
        e.mem.set_u32(object + 0x20, 0x4000_0004);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0x7777);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [0x00d3_3260, 0x00d0_1a60, 0x0900_00a1] {
            e.register(address, |_, _| Ret::default());
        }
        e.register_double(0x00cf_5640, move |e, a| {
            // The call is given the address of the word `**info`.
            assert_eq!(e.mem.u32(a[1]), 0x5000);
            Ret {
                eax: object,
                ..Ret::default()
            }
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d3_5770, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..0x20 {
                words.push(e.mem.u32(a[0] + word * 4));
            }
            let record = e.mem.u32(a[0]);
            for word in 0..0x14 {
                words.push(e.mem.u32(record + word * 4));
            }
            for offset in [0x60, 0x70, 0x74, 0x78, 0x7c] {
                words.push(e.mem.u32(a[0] + offset));
            }
            *log.borrow_mut() = words;
            Ret {
                eax: 0x1234,
                ..Ret::default()
            }
        });
        Scene {
            info,
            object,
            holder,
            records,
            seen,
        }
    }

    #[test]
    fn fn_00d365d0_builds_the_records_and_calls_00d35770() {
        let mut e = Engine::new();
        let scene = install(&mut e, 0);
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x00d3_65d0,
                &args![scene.info, 0x11u32, 0x22u32, 0x33u32, 0x44u32],
            )
            .u32();
        assert_eq!(result, 0x1234);
        let log = e.call_log.take().unwrap();
        let addresses: Vec<u32> = log.iter().map(|(address, _)| *address).collect();
        assert_eq!(
            addresses,
            vec![
                0x00d3_65d0,
                0x00fd_f044,
                0x00d3_3260,
                0x00cf_5640,
                0x00d0_1a60,
                0x00d3_5770,
                MEMORY_ROUTER,
                0x0900_00a1,
                0x00fd_f044
            ]
        );
        // `00d33260` copies `info` into the frame object; `00cf5640` is called on the frame's
        // `+0xe4` object with the address of the frame's `+8` word, 1 and 0.
        assert_eq!(log[2].1[1], scene.info);
        assert_eq!(log[3].1[0], log[3].1[1] + 0xdc);
        assert_eq!(log[3].1[2..], [1, 0]);
        assert_eq!(log[2].1[0], log[3].1[0] - 0xe4 + 0x64);
        // The new object is destroyed through slot 0 with 0 after its array's size is cleared.
        assert_eq!(log[7].1, vec![scene.object, 0]);
        assert_eq!(e.mem.u32(scene.object + 0x1c), 0);
        let seen = scene.seen.borrow();
        assert_eq!(seen[1..5], [0x11, 0x22, 0x33, 0x44]);
        // The first object: `+0` is the record's address, `+8` is `*info`.
        assert_ne!(seen[5], 0);
        assert_eq!(seen[5 + 2], scene.holder);
        // The record: the object, -1, `*(holder + 8)`, `*(holder + 0xc)`, zero bytes, the
        // distance byte 0xec and the 0xff byte in the word at +0x18, then the copied block.
        let record = &seen[37..57];
        assert_eq!(
            record[..8],
            [scene.object, 0xffff_ffff, 0x5008, 0, 0, 0, 0x00ff_ec00, 0]
        );
        let block: Vec<u32> = (0..11).map(|word| 0x6000 + word).collect();
        assert_eq!(record[8..19], block[..]);
        assert_eq!(record[19], 0xbf80_0000);
        // The words copied from `info`: +0x60, +0x70, +0x74, +0x78, +0x7c.
        assert_eq!(seen[57..], [0x3018, 0x301c, 0x301d, 0x301e, 0x301f]);
        // Two profiling records around the call.
        assert_eq!(e.mem.u32(scene.records), 0x010c_ed28);
        assert_eq!(e.mem.u32(scene.records + 0xc), 0x010c_41f8);
    }

    #[test]
    fn fn_00d365d0_keeps_the_constructed_object_when_the_holder_word_is_set() {
        let mut e = Engine::new();
        let scene = install(&mut e, 0x5c5c);
        e.call(0x00d3_65d0, &args![scene.info, 0u32, 0u32, 0u32, 0u32]);
        let seen = scene.seen.borrow();
        let record = &seen[37..57];
        assert_eq!(record[3], 0x5c5c);
        // Nothing copies the block over the constructed object (a double that leaves it zero).
        assert!(record[8..19].iter().all(|word| *word == 0));
        assert_eq!(record[19], 0xbf80_0000);
    }

    #[test]
    fn fn_00d367b0_prepares_the_object() {
        let mut e = Engine::new();
        e.register(0x00da_6fd0, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00d3_67b0, &args![object]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x00da_6fd0, vec![object])]);
        assert_eq!(e.mem.u32(object + 0x2c), 0x00d3_65d0);
    }

    #[test]
    fn fn_00d367d0_registers_a_prepared_record() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::<u32>::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00da_6fd0, move |e, a| {
            // The record's words and bytes the function clears are zero when this is called.
            for offset in [0x18u32, 0x1c, 0x20, 0x24, 0x28] {
                assert_eq!(e.mem.u32(a[0] + offset), 0);
            }
            e.mem.set_u32(a[0], 0x1111);
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let registered = Rc::new(RefCell::new(Vec::<u32>::new()));
        let call = Rc::clone(&registered);
        e.register_double(0x00cf_bd60, move |e, a| {
            call.borrow_mut().extend_from_slice(a);
            call.borrow_mut().push(e.mem.u32(a[1]));
            call.borrow_mut().push(e.mem.u32(a[1] + 0x2c));
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d3_67d0, &args![queue]);
        let record = seen.borrow()[0];
        assert_eq!(
            *registered.borrow(),
            vec![queue, record, 0x12, 7, 0x1111, 0x00d3_65d0]
        );
    }
}
