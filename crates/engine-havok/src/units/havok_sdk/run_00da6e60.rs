//! `Havok SDK/run_00da6e60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkAgent1nMachine_UpdateShapeCollectionFilter` (`00da48e0`, cdecl `(track, input)`), the
/// agent track's first-sector function `00da4430` (cdecl `(track)`) and the dispatcher's agent
/// registration `00cfbd60` (thiscall `(table, -1, 9)`).
const UPDATE_SHAPE_COLLECTION_FILTER: u32 = 0x00da_48e0;
const TRACK_FIRST_SECTOR: u32 = 0x00da_4430;
const REGISTER_AGENT: u32 = 0x00cf_bd60;

// Translated from 00da6e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, seven stack words `(a0, track, a2, a3, a4, a5, a6)`: asks the object `[a3]` for the
/// value of its virtual slot `+0x10`, packs `a2`, `a3`, that value (at `+0xc`), `a4`, `a5` and
/// `a6` in a 0x1c-byte block on the stack (its word at `+8` is never written) and runs
/// `hkAgent1nMachine_UpdateShapeCollectionFilter(track, block)`. `a0` is not used. The exe's
/// name for it is unknown (a function of the list agent's table, see `00da6fd0`).
#[allow(clippy::too_many_arguments)]
pub fn fn_00da6e60(
    e: &mut Engine,
    _a0: u32,
    track: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
) {
    let object = e.mem.u32(a3);
    let value = e.vcall(object, 0x10, &args![]).u32();
    e.with_stack(0x20, |e, block| {
        let block = block.addr();
        e.mem.set_u32(block, a2);
        e.mem.set_u32(block + 0xc, value);
        e.mem.set_u32(block + 0x10, a4);
        e.mem.set_u32(block + 0x14, a5);
        e.mem.set_u32(block + 4, a3);
        e.mem.set_u32(block + 0x18, a6);
        e.call(UPDATE_SHAPE_COLLECTION_FILTER, &args![track, block]);
    });
}

// Translated from 00da6fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, three stack words `(a0, a1, track)` of which the first two are not used: a non-null
/// `track` (an `hkAgent1nMachine` track: sector array of data, size 0 and capacity word
/// `0x80000000`) is cleared, then it is given its first sector (`00da4430`; also called with a
/// null `track`, as the exe does). Answers `track + 0x10`. The exe's name for it is unknown (the
/// list agent's first table entry, see `00da6fd0`).
pub fn fn_00da6fa0(e: &mut Engine, _a0: u32, _a1: u32, track: u32) -> u32 {
    if track != 0 {
        e.mem.set_u32(track, 0);
        e.mem.set_u32(track + 4, 0);
        e.mem.set_u32(track + 8, 0x8000_0000);
    }
    e.call(TRACK_FIRST_SECTOR, &args![track]);
    track.wrapping_add(0x10)
}

// Translated from 00da6fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the 0x34-byte agent function table `table` (cdecl `(table)`): the function `00da6fa0`
/// at `+0`, the code at `00da6dc0` at `+4`, zeros at `+8` to `+0x14`, `00da6e60` at `+0x18`,
/// `00da6de0` at `+0x1c` (the engine map names it `hkListAgent3::calcStatistics`), `00da6e10`
/// at `+0x20`, `00da6e30` at `+0x24`, 0 at `+0x28`, `00da6eb0` at `+0x2c` and the byte 1 at
/// `+0x30`. The exe's name for it is unknown (the list agent's table).
pub fn fn_00da6fd0(e: &mut Engine, table: u32) {
    e.mem.set_u32(table, 0x00da_6fa0);
    e.mem.set_u32(table + 0x2c, 0x00da_6eb0);
    e.mem.set_u32(table + 0x28, 0);
    for offset in [0x8, 0xc, 0x10, 0x14] {
        e.mem.set_u32(table + offset, 0);
    }
    e.mem.set_u32(table + 0x18, 0x00da_6e60);
    e.mem.set_u32(table + 0x1c, 0x00da_6de0);
    e.mem.set_u32(table + 0x20, 0x00da_6e10);
    e.mem.set_u32(table + 0x24, 0x00da_6e30);
    e.mem.set_u32(table + 4, 0x00da_6dc0);
    e.mem.set_u8(table + 0x30, 1);
}

// Translated from 00da7020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the list agent with the dispatcher (cdecl, one word: the dispatcher): builds the
/// function table on the stack (`00da6fd0` after clearing the bytes `+0x30` to `+0x32` and the
/// words `+0x18` to `+0x28`) and calls `00cfbd60` on the dispatcher with the table, -1 and 9.
/// The exe's name for it is unknown.
pub fn fn_00da7020(e: &mut Engine, dispatcher: u32) {
    e.with_stack(0x40, |e, table| {
        let table = table.addr();
        for offset in [0x30, 0x31, 0x32] {
            e.mem.set_u8(table + offset, 0);
        }
        for offset in [0x18, 0x1c, 0x20, 0x24, 0x28] {
            e.mem.set_u32(table + offset, 0);
        }
        fn_00da6fd0(e, table);
        e.call(REGISTER_AGENT, &args![dispatcher, table, u32::MAX, 9u32]);
    });
}

// Translated from 00da6e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, three words `_unused_0`, `first`, `second`; descriptions, not confirmed):
/// the list agent's table stores its address (`00da7000`). Hands its last two words to
/// `00da3510` (cdecl, two words) and answers what that answers.
pub fn fn_00da6e10(e: &mut Engine, _unused_0: u32, first: u32, second: u32) -> u32 {
    e.call(0x00da_3510, &args![first, second]).u32()
}

// Translated from 00da6e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, five words `_unused_0`, `first`, `second`, `third`, `fourth`; `second` and
/// `third` are floats; descriptions, not confirmed): the list agent's table stores its address
/// (`00da7007`). Hands `first`, the two floats (each through `FLD`/`FSTP`, a signalling NaN
/// quieted) and `fourth` to `00da35f0` (cdecl, four words) and answers what that answers.
pub fn fn_00da6e30(
    e: &mut Engine,
    _unused_0: u32,
    first: u32,
    second: f32,
    third: f32,
    fourth: u32,
) -> u32 {
    e.call(
        0x00da_35f0,
        &args![first, x87_quiet(second), x87_quiet(third), fourth],
    )
    .u32()
}

// Translated from 00da6eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, five words `input`, `_unused_1`, `data`, `_unused_3`, `state`; the process
/// function of the list agent: `hkpSaveContactPointsUtil::getSerializedAgentType` numbers it 6;
/// the table at `00da6fdc` stores its address). Between the profiling records `010d50b8` and
/// `010c41f8` it lists, in the memory router's scratch space (the word at the router's `+0`
/// moves up by `(4 * n + 0x83) & !0x7f` for the duration), the indices `i` below `n` (the word
/// at `+0x1c` of the object `[[input + 4]]`, signed) that are at least 0x100 or whose bit is set
/// in the bit mask of 32-bit words at `+0x50` of that object; ends the list with
/// `0xffffffff` and runs `00da4aa0` (cdecl) with `(data, input, object + 0x10, list, state)`.
/// Answers `data + 0x10`.
pub fn fn_00da6eb0(
    e: &mut Engine,
    input: Ptr,
    _unused_1: u32,
    data: u32,
    _unused_3: u32,
    state: u32,
) -> u32 {
    crate::units::havok_sdk::hkadeltacompressedanimation::profile_record(e, 0x010d_50b8);
    let holder = e.mem.u32(input.addr().wrapping_add(4));
    let object = e.mem.u32(holder);
    let count = e.mem.i32(object.wrapping_add(0x1c));
    let router = e.call(0x00c8_5750, &args![]).u32();
    let list = e.mem.u32(router);
    let reserved = ((count as u32).wrapping_mul(4).wrapping_add(0x83)) & 0xffff_ff80;
    e.mem.set_u32(router, reserved.wrapping_add(list));
    let mut found = 0u32;
    let mut mask = 1u32;
    let mut index = 0i32;
    while index < count {
        let wanted = if index as u32 >= 0x100 {
            true
        } else {
            let word = e.mem.u32(
                object
                    .wrapping_add((index as u32 >> 5) * 4)
                    .wrapping_add(0x50),
            );
            word & mask != 0
        };
        if wanted {
            e.mem.set_u32(list.wrapping_add(found * 4), index as u32);
            found += 1;
        }
        index += 1;
        mask = mask.rotate_left(1);
    }
    e.mem.set_u32(list.wrapping_add(found * 4), 0xffff_ffff);
    e.call(
        0x00da_4aa0,
        &args![data, input, object.wrapping_add(0x10), list, state],
    );
    crate::units::havok_sdk::hkadeltacompressedanimation::profile_record(e, 0x010c_41f8);
    let router = e.call(0x00c8_5750, &args![]).u32();
    e.mem.set_u32(router, list);
    data.wrapping_add(0x10)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da6e10, fn_00da6e10(u32, u32, u32) -> u32),
        entry!(0x00da6e30, fn_00da6e30(u32, u32, f32, f32, u32) -> u32),
        entry!(0x00da6eb0, fn_00da6eb0(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00da_6e60, fn_00da6e60(u32, u32, u32, u32, u32, u32, u32)),
        entry!(0x00da_6fa0, fn_00da6fa0(u32, u32, u32) -> u32),
        entry!(0x00da_6fd0, fn_00da6fd0(u32)),
        entry!(0x00da_7020, fn_00da7020(u32)),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The calls a double saw: (address, argument words).
    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn recording_double(e: &mut Engine, calls: &Calls, address: u32, eax: u32) {
        let seen = Rc::clone(calls);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn fn_00da6e10_hands_its_last_two_words_to_00da3510() {
        let mut e = Engine::new();
        let calls: Calls = Rc::default();
        recording_double(&mut e, &calls, 0x00da_3510, 0x55);
        let answer = e.call(0x00da_6e10, &args![1u32, 2u32, 3u32]).u32();
        assert_eq!(answer, 0x55);
        assert_eq!(*calls.borrow(), vec![(0x00da_3510, vec![2, 3])]);
    }

    #[test]
    fn fn_00da6e30_hands_the_floats_through_the_x87_stack() {
        let mut e = Engine::new();
        let calls: Calls = Rc::default();
        recording_double(&mut e, &calls, 0x00da_35f0, 0x66);
        let signalling = f32::from_bits(0x7fa0_0001);
        let answer = e
            .call(0x00da_6e30, &args![1u32, 2u32, 1.5f32, signalling, 5u32])
            .u32();
        assert_eq!(answer, 0x66);
        assert_eq!(
            *calls.borrow(),
            vec![(0x00da_35f0, vec![2, 1.5f32.to_bits(), 0x7fe0_0001, 5])]
        );
    }

    /// The thread's record stream (TLS index 3) and the router with a scratch area.
    struct Setup00da6eb0 {
        e: Engine,
        calls: Calls,
        stream: u32,
        records: u32,
        router: u32,
        scratch: u32,
        input: u32,
        object: u32,
    }

    fn setup_00da6eb0(count: u32) -> Setup00da6eb0 {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(0x0126_835c, 3u32);
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x40);
        e.register_double(0x00fd_f044, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        let scratch = e.mem.alloc(0x800);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, scratch);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object + 0x1c, count);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, object);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 4, holder);
        let calls: Calls = Rc::default();
        // The process helper: notes its words, the list it was handed and the scratch cursor.
        let seen = Rc::clone(&calls);
        e.register_double(0x00da_4aa0, move |e, a| {
            let mut words = a.to_vec();
            let mut at = a[3];
            loop {
                let value = e.mem.u32(at);
                words.push(value);
                if value == 0xffff_ffff {
                    break;
                }
                at += 4;
            }
            words.push(e.mem.u32(router));
            seen.borrow_mut().push((0x00da_4aa0, words));
            Ret::default()
        });
        Setup00da6eb0 {
            e,
            calls,
            stream,
            records,
            router,
            scratch,
            input,
            object,
        }
    }

    fn labels_00da6eb0(s: &Setup00da6eb0) -> Vec<u32> {
        let end = s.e.mem.u32(s.stream + 4);
        (s.records..end)
            .step_by(12)
            .map(|record| s.e.mem.u32(record))
            .collect()
    }

    #[test]
    fn fn_00da6eb0_lists_the_indices_whose_bits_are_set() {
        let mut s = setup_00da6eb0(5);
        s.e.mem.set_u32(s.object + 0x50, 0b10110);
        let (input, data) = (s.input, 0x3000u32);
        let answer =
            s.e.call(0x00da_6eb0, &args![input, 1u32, data, 2u32, 0x77u32])
                .u32();
        assert_eq!(answer, 0x3010);
        let call = s.calls.borrow()[0].1.clone();
        // (data, input, object + 0x10, list, state), the list, the scratch cursor during the call.
        assert_eq!(call[..5], [data, input, s.object + 0x10, s.scratch, 0x77]);
        assert_eq!(call[5..9], [1, 2, 4, 0xffff_ffff]);
        assert_eq!(call[9], s.scratch + 0x80);
        assert_eq!(s.e.mem.u32(s.router), s.scratch);
        assert_eq!(labels_00da6eb0(&s), vec![0x010d_50b8, 0x010c_41f8]);
    }

    #[test]
    fn fn_00da6eb0_takes_every_index_from_0x100_up_and_reads_bits_only_below() {
        let mut s = setup_00da6eb0(0x104);
        s.e.mem.set_u32(s.object + 0x50 + 4 * 7, 0x8000_0000);
        let (input, data) = (s.input, 0x3000u32);
        s.e.call(0x00da_6eb0, &args![input, 1u32, data, 2u32, 0x77u32]);
        let call = s.calls.borrow()[0].1.clone();
        assert_eq!(
            call[5..],
            [
                0xff,
                0x100,
                0x101,
                0x102,
                0x103,
                0xffff_ffff,
                s.scratch + 0x480
            ]
        );
    }

    #[test]
    fn fn_00da6eb0_with_no_entries_hands_over_an_empty_list() {
        let mut s = setup_00da6eb0(0);
        let (input, data) = (s.input, 0x3000u32);
        s.e.call(0x00da_6eb0, &args![input, 1u32, data, 2u32, 0x77u32]);
        let call = s.calls.borrow()[0].1.clone();
        assert_eq!(call[5..], [0xffff_ffff, s.scratch + 0x80]);
    }

    const VALUE_SLOT: u32 = 0x0900_0010;

    #[test]
    fn the_block_is_packed_and_handed_to_the_filter_update() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, VALUE_SLOT);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, object);
        e.register(VALUE_SLOT, |_, a| a[0].into_ret());
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(UPDATE_SHAPE_COLLECTION_FILTER, move |e, a| {
            let block: Vec<u32> = [0u32, 4, 0xc, 0x10, 0x14, 0x18]
                .iter()
                .map(|offset| e.mem.u32(a[1] + offset))
                .collect();
            log.borrow_mut().push((a[0], block));
            Ret::default()
        });
        e.call(
            0x00da_6e60,
            &args![1u32, 0x7000u32, 0x22u32, cell, 0x44u32, 0x55u32, 0x66u32],
        );
        assert_eq!(
            *seen.borrow(),
            vec![(0x7000, vec![0x22, cell, object, 0x44, 0x55, 0x66])]
        );
    }

    #[test]
    fn the_track_is_cleared_and_given_its_first_sector() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(TRACK_FIRST_SECTOR, |_, _| Ret::default());
        let track = e.mem.alloc(0x20);
        for offset in [0, 4, 8, 0xc] {
            e.mem.set_u32(track + offset, 0x1111);
        }
        let result = e.call(0x00da_6fa0, &args![5u32, 6u32, track]).u32();
        assert_eq!(result, track + 0x10);
        assert_eq!(e.mem.u32(track), 0);
        assert_eq!(e.mem.u32(track + 4), 0);
        assert_eq!(e.mem.u32(track + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(track + 0xc), 0x1111);
        let log = e.call_log.as_ref().unwrap();
        assert!(log.contains(&(TRACK_FIRST_SECTOR, vec![track])));
    }

    #[test]
    fn a_null_track_still_runs_the_first_sector_function() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(TRACK_FIRST_SECTOR, |_, _| Ret::default());
        let result = e.call(0x00da_6fa0, &args![0u32, 0u32, 0u32]).u32();
        assert_eq!(result, 0x10);
        let log = e.call_log.as_ref().unwrap();
        assert!(log.contains(&(TRACK_FIRST_SECTOR, vec![0])));
    }

    #[test]
    fn the_function_table_is_filled() {
        let mut e = Engine::new();
        let table = e.mem.alloc(0x40);
        for word in 0..0x10u32 {
            e.mem.set_u32(table + 4 * word, 0xdead_beef);
        }
        e.call(0x00da_6fd0, &args![table]);
        let words: Vec<u32> = (0..0xc).map(|i| e.mem.u32(table + 4 * i)).collect();
        assert_eq!(
            words,
            [
                0x00da_6fa0,
                0x00da_6dc0,
                0,
                0,
                0,
                0,
                0x00da_6e60,
                0x00da_6de0,
                0x00da_6e10,
                0x00da_6e30,
                0,
                0x00da_6eb0,
            ]
        );
        assert_eq!(e.mem.u8(table + 0x30), 1);
        // Only the byte at +0x30 is written there; the rest of that word is untouched.
        assert_eq!(e.mem.u32(table + 0x30), 0xdead_be01);
        assert_eq!(e.mem.u32(table + 0x34), 0xdead_beef);
    }

    #[test]
    fn the_agent_is_registered_with_the_table_minus_one_and_nine() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(REGISTER_AGENT, move |e, a| {
            let words: Vec<u32> = (0..0xc).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut()
                .push((a[0], a[2], a[3], words, e.mem.u8(a[1] + 0x30)));
            Ret::default()
        });
        e.call(0x00da_7020, &args![0x9000u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!((seen[0].0, seen[0].1, seen[0].2), (0x9000, u32::MAX, 9));
        assert_eq!(seen[0].3[0], 0x00da_6fa0);
        assert_eq!(seen[0].3[6], 0x00da_6e60);
        assert_eq!(seen[0].4, 1);
    }
}
