//! `Havok SDK/run_00d92300` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpCollisionDispatcher::registerAgent3` (`00cfbd60`, in `hkpcollisiondispatcher.cpp`; the
/// engine map has no name for it): `thiscall(dispatcher, functions, type_a, type_b)`, `RET 0xc`.
const REGISTER_AGENT3: u32 = 0x00cf_bd60;

// Translated from 00d92300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(input, header, out)`: the agent3 creation function the default registration
/// (`00d923b0`) installs first. Sets the byte at `header` to `(byte & 8) | 6`. Looks up the
/// creation function in the dispatcher `D = [[input + 0xc]]`: the table at `D + 0x5a0` when the
/// word at `[input + 0xc] + 0x14` is non-zero, otherwise at `D + 0x1a0`, row `type_a * 32`
/// (`type_a` the word at `+0xc` of what `[input]` points to), column `type_b` (the same word of
/// `[input + 4]`'s target); the byte found selects the entry `D + 0x9a0 + 0x14 * byte`. That
/// function is called (cdecl) with `([input], [input + 4], [input + 0xc], [input + 0x10])`; its
/// result is stored at `out`, the byte at `header + 2` becomes `0xff`, and `out + 0x10` is
/// returned. The class is unconfirmed (an `hkpCollisionDispatcher` agent3 `create`).
pub fn fn_00d92300(e: &mut Engine, input: u32, header: u32, out: u32) -> u32 {
    let first = e.mem.u8(header);
    e.mem.set_u8(header, (first & 8) | 6);
    let fifth = e.mem.u32(input.wrapping_add(0x10));
    let second = e.mem.u32(input.wrapping_add(4));
    let second_shape = e.mem.u32(second);
    let type_b = e.mem.u32(second_shape.wrapping_add(0xc));
    let first_body = e.mem.u32(input);
    let context = e.mem.u32(input.wrapping_add(0xc));
    let nonzero = e.mem.u32(context.wrapping_add(0x14)) != 0;
    let dispatcher = e.mem.u32(context);
    let first_shape = e.mem.u32(first_body);
    let type_a = e.mem.u32(first_shape.wrapping_add(0xc));
    let table = if nonzero {
        dispatcher.wrapping_add(0x5a0)
    } else {
        dispatcher.wrapping_add(0x1a0)
    };
    let row = (type_a << 5).wrapping_add(table);
    let selector = u32::from(e.mem.u8(row.wrapping_add(type_b)));
    let function = e.mem.u32(
        dispatcher
            .wrapping_add(selector.wrapping_mul(5).wrapping_mul(4))
            .wrapping_add(0x9a0),
    );
    let result = e
        .call(function, &args![first_body, second, context, fifth])
        .u32();
    e.mem.set_u32(out, result);
    e.mem.set_u8(header.wrapping_add(2), 0xff);
    out.wrapping_add(0x10)
}

// Translated from 00d923b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionDispatcher::registerDefaultAgent3`-shaped (the unit's call site is
/// `hkpcollisiondispatcher.rs`): cdecl `(dispatcher)`. Builds on the stack the table of agent3
/// functions of the default agent (`00d92300` first, then the code addresses `00d921e0`,
/// `00d922a0`, `00d922c0`, `00d922e0`, `00d92200`, `00d92230`, `00d92250`, `00d92270`, with zero
/// words at `+8` and `+0x28`, `00d92380` at `+0x2c`, the byte 1 at `+0x30` and zero bytes at
/// `+0x31` and `+0x32`) and registers it with `00cfbd60` on the dispatcher for the types
/// `-1, -1`. Returns the callee's `EAX`.
pub fn fn_00d923b0(e: &mut Engine, dispatcher: u32) -> u32 {
    e.with_stack(0x40, |e, table| {
        let table = table.addr();
        e.mem.set_u8(table + 0x31, 0);
        e.mem.set_u8(table + 0x32, 0);
        e.mem.set_u32(table + 0x28, 0);
        e.mem.set_u32(table + 8, 0);
        e.mem.set_u32(table + 0x10, 0x00d9_22c0);
        e.mem.set_u32(table + 0x14, 0x00d9_22e0);
        e.mem.set_u32(table + 0xc, 0x00d9_22a0);
        e.mem.set_u32(table, 0x00d9_2300);
        e.mem.set_u32(table + 0x2c, 0x00d9_2380);
        e.mem.set_u32(table + 4, 0x00d9_21e0);
        e.mem.set_u32(table + 0x18, 0x00d9_2200);
        e.mem.set_u32(table + 0x1c, 0x00d9_2230);
        e.mem.set_u32(table + 0x20, 0x00d9_2250);
        e.mem.set_u32(table + 0x24, 0x00d9_2270);
        e.mem.set_u8(table + 0x30, 1);
        e.call(
            REGISTER_AGENT3,
            &args![dispatcher, table, 0xffff_ffffu32, 0xffff_ffffu32],
        )
        .u32()
    })
}

// Translated from 00d921e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00d923ff). A function of
/// the table `00d923b0` registers with `00cfbd60`, cdecl with at least four stack words
/// `(_unused_0, holder, _unused_2, argument)`: `holder` is the address of a pointer to an object;
/// calls that object's virtual slot `0x1c` with `argument` and answers the result.
pub fn fn_00d921e0(
    e: &mut Engine,
    _unused_0: u32,
    holder: Ptr,
    _unused_2: u32,
    argument: u32,
) -> u32 {
    let object = e.mem.u32(holder.addr());
    e.vcall(object, 0x1c, &args![argument]).u32()
}

// Translated from 00d92200 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00d92407). A function of
/// the table `00d923b0` registers with `00cfbd60`, cdecl with at least seven stack words
/// `(_unused_0, holder, first, second, third, _unused_5, fourth)`: `holder` is the address of a
/// pointer to an object; calls that object's virtual slot `0x20` with `(first, second, third,
/// fourth)` and answers the result.
pub fn fn_00d92200(
    e: &mut Engine,
    _unused_0: u32,
    holder: Ptr,
    first: u32,
    second: u32,
    third: u32,
    _unused_5: u32,
    fourth: u32,
) -> u32 {
    let object = e.mem.u32(holder.addr());
    e.vcall(object, 0x20, &args![first, second, third, fourth])
        .u32()
}

// Translated from 00d92230 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00d9240f). A function of
/// the table `00d923b0` registers with `00cfbd60`, cdecl with at least four stack words
/// `(_unused_0, holder, _unused_2, target)`: `holder` is the address of a pointer to an object.
/// Calls virtual slot `0xc` of `target` with `(0x010d3db0, that object, 0)` (the first word is the
/// address of the string `"ChildAgent2"`) and answers the result.
pub fn fn_00d92230(
    e: &mut Engine,
    _unused_0: u32,
    holder: Ptr,
    _unused_2: u32,
    target: Ptr,
) -> u32 {
    let object = e.mem.u32(holder.addr());
    e.vcall(target.addr(), 0xc, &args![0x010d_3db0u32, object, 0u32])
        .u32()
}

// Translated from 00d92250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00d92417). A function of
/// the table `00d923b0` registers with `00cfbd60`, cdecl with at least three stack words
/// `(_unused_0, holder, argument)`: `holder` is the address of a pointer to an object; calls that
/// object's virtual slot `0x24` with `argument` and answers the result.
pub fn fn_00d92250(e: &mut Engine, _unused_0: u32, holder: Ptr, argument: u32) -> u32 {
    let object = e.mem.u32(holder.addr());
    e.vcall(object, 0x24, &args![argument]).u32()
}

// Translated from 00d92270 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00d9241f). A function of
/// the table `00d923b0` registers with `00cfbd60`, cdecl with at least five stack words
/// `(_unused_0, holder, first, second, third)`: `holder` is the address of a pointer to an object.
/// Calls that object's virtual slot `0x28` with `(first, second, third)` and answers the result;
/// `first` and `second` are floats that go through a `FLD`/`FSTP` copy, which quiets a signalling
/// NaN.
pub fn fn_00d92270(
    e: &mut Engine,
    _unused_0: u32,
    holder: Ptr,
    first: f32,
    second: f32,
    third: u32,
) -> u32 {
    let object = e.mem.u32(holder.addr());
    e.vcall(
        object,
        0x28,
        &args![x87_quiet(first), x87_quiet(second), third],
    )
    .u32()
}

// Translated from 00d922a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00d923e7). A function of
/// the table `00d923b0` registers with `00cfbd60`, cdecl with at least three stack words
/// `(_unused_0, holder, argument)`: `holder` is the address of a pointer to an object; calls that
/// object's virtual slot `0x2c` with `argument` and answers the result.
pub fn fn_00d922a0(e: &mut Engine, _unused_0: u32, holder: Ptr, argument: u32) -> u32 {
    let object = e.mem.u32(holder.addr());
    e.vcall(object, 0x2c, &args![argument]).u32()
}

// Translated from 00d922c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00d923d7). A function of
/// the table `00d923b0` registers with `00cfbd60`, cdecl with at least three stack words
/// `(_unused_0, holder, argument)`: `holder` is the address of a pointer to an object; calls that
/// object's virtual slot `0x30` with `argument` and answers the result.
pub fn fn_00d922c0(e: &mut Engine, _unused_0: u32, holder: Ptr, argument: u32) -> u32 {
    let object = e.mem.u32(holder.addr());
    e.vcall(object, 0x30, &args![argument]).u32()
}

// Translated from 00d922e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00d923df). A function of
/// the table `00d923b0` registers with `00cfbd60`, cdecl with at least three stack words
/// `(_unused_0, holder, argument)`: `holder` is the address of a pointer to an object; calls that
/// object's virtual slot `0x34` with `argument` and answers the result.
pub fn fn_00d922e0(e: &mut Engine, _unused_0: u32, holder: Ptr, argument: u32) -> u32 {
    let object = e.mem.u32(holder.addr());
    e.vcall(object, 0x34, &args![argument]).u32()
}

// Translated from 00d92380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00d923f7). The last
/// function of the table `00d923b0` registers with `00cfbd60`, cdecl with five stack words
/// `(input, _unused_1, agent_data, _unused_3, argument)`. Calls virtual slot `0x18` of the object
/// stored at `agent_data` with `(input[0], input[1], input[3], argument)` (the words at `+0`,
/// `+4` and `+0xc` of `input`) and answers `agent_data + 0x10`.
pub fn fn_00d92380(
    e: &mut Engine,
    input: Ptr,
    _unused_1: u32,
    agent_data: Ptr,
    _unused_3: u32,
    argument: u32,
) -> u32 {
    let object = e.mem.u32(agent_data.addr());
    let first = e.mem.u32(input.addr().wrapping_add(0xc));
    let second = e.mem.u32(input.addr().wrapping_add(4));
    let third = e.mem.u32(input.addr());
    e.vcall(object, 0x18, &args![third, second, first, argument]);
    agent_data.addr().wrapping_add(0x10)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9_2300, fn_00d92300(u32, u32, u32) -> u32),
        entry!(0x00d9_23b0, fn_00d923b0(u32) -> u32),
        entry!(0x00d9_21e0, fn_00d921e0(u32, Ptr, u32, u32) -> u32),
        entry!(0x00d9_2200, fn_00d92200(u32, Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00d9_2230, fn_00d92230(u32, Ptr, u32, Ptr) -> u32),
        entry!(0x00d9_2250, fn_00d92250(u32, Ptr, u32) -> u32),
        entry!(0x00d9_2270, fn_00d92270(u32, Ptr, f32, f32, u32) -> u32),
        entry!(0x00d9_22a0, fn_00d922a0(u32, Ptr, u32) -> u32),
        entry!(0x00d9_22c0, fn_00d922c0(u32, Ptr, u32) -> u32),
        entry!(0x00d9_22e0, fn_00d922e0(u32, Ptr, u32) -> u32),
        entry!(0x00d9_2380, fn_00d92380(Ptr, u32, Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    const CREATE_FUNCTION: u32 = 0x0900_0100;

    /// A dispatcher with the creation function for selector 3 at `D + 0x9a0 + 3 * 0x14`, the
    /// byte 3 at row `type_a`, column `type_b` of both tables (the other table holds 0).
    fn setup(e: &mut Engine, use_second_table: bool) -> (u32, u32, u32) {
        let dispatcher = e.mem.alloc(0xb00);
        e.mem
            .set_u32(dispatcher + 0x9a0 + 3 * 0x14, CREATE_FUNCTION);
        let shape_a = e.mem.alloc(0x20);
        e.mem.set_u32(shape_a + 0xc, 2);
        let shape_b = e.mem.alloc(0x20);
        e.mem.set_u32(shape_b + 0xc, 5);
        let body_a = e.mem.alloc(0x10);
        e.mem.set_u32(body_a, shape_a);
        let body_b = e.mem.alloc(0x10);
        e.mem.set_u32(body_b, shape_b);
        let table = if use_second_table { 0x5a0 } else { 0x1a0 };
        e.mem.set_u8(dispatcher + table + (2 << 5) + 5, 3);
        let context = e.mem.alloc(0x20);
        e.mem.set_u32(context, dispatcher);
        e.mem.set_u32(context + 0x14, u32::from(use_second_table));
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input, body_a);
        e.mem.set_u32(input + 4, body_b);
        e.mem.set_u32(input + 0xc, context);
        e.mem.set_u32(input + 0x10, 0x4444);
        (input, body_b, context)
    }

    #[test]
    fn the_creation_function_comes_from_the_table_the_context_selects() {
        for use_second_table in [false, true] {
            let mut e = Engine::new();
            let (input, body_b, context) = setup(&mut e, use_second_table);
            e.register(CREATE_FUNCTION, |_, _| Ret {
                eax: 0x7777,
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            let header = e.mem.alloc(8);
            e.mem.set_u8(header, 0xff);
            let out = e.mem.alloc(0x20);
            let result = e.call(0x00d9_2300, &args![input, header, out]).u32();
            assert_eq!(result, out + 0x10);
            assert_eq!(e.mem.u32(out), 0x7777);
            assert_eq!(e.mem.u8(header), 0x0e);
            assert_eq!(e.mem.u8(header + 2), 0xff);
            let log = e.call_log.take().unwrap();
            let calls: Vec<_> = log.iter().filter(|(a, _)| *a == CREATE_FUNCTION).collect();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].1, vec![e.mem.u32(input), body_b, context, 0x4444]);
        }
    }

    #[test]
    fn the_header_keeps_only_bit_three() {
        let mut e = Engine::new();
        let (input, _, _) = setup(&mut e, false);
        e.register(CREATE_FUNCTION, |_, _| Ret::default());
        let header = e.mem.alloc(8);
        let out = e.mem.alloc(0x20);
        e.mem.set_u8(header, 0x10);
        e.call(0x00d9_2300, &args![input, header, out]);
        assert_eq!(e.mem.u8(header), 0x06);
    }

    #[test]
    fn default_registration_builds_the_function_table_and_registers_it_for_all_types() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(REGISTER_AGENT3, move |e, a| {
            let words: Vec<u32> = (0..13).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push((
                a.to_vec(),
                words,
                e.mem.u8(a[1] + 0x31),
                e.mem.u8(a[1] + 0x32),
            ));
            Ret {
                eax: 0x1357,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00d9_23b0, &args![0x5000u32]).u32(), 0x1357);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (arguments, words, byte_one, byte_two) = &seen[0];
        assert_eq!(arguments[0], 0x5000);
        assert_eq!(arguments[2..], [u32::MAX, u32::MAX]);
        assert_eq!(words[0], 0x00d9_2300);
        assert_eq!(words[1], 0x00d9_21e0);
        assert_eq!(words[2], 0);
        assert_eq!(
            words[3..10],
            [
                0x00d9_22a0,
                0x00d9_22c0,
                0x00d9_22e0,
                0x00d9_2200,
                0x00d9_2230,
                0x00d9_2250,
                0x00d9_2270
            ]
        );
        assert_eq!(words[10], 0);
        assert_eq!(words[11], 0x00d9_2380);
        assert_eq!(words[12] & 0xff, 1);
        assert_eq!((*byte_one, *byte_two), (0, 0));
    }
}

#[cfg(test)]
mod a05_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A double at `address` that records the argument words of each call and answers `answer`.
    fn recording_double(e: &mut Engine, address: u32, answer: u32) -> CallLog {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    /// An object whose vtable (zeroed, 0x100 bytes) holds `0x7000_0000 + slot` in each of `slots`.
    fn object_with_slots(e: &mut Engine, slots: &[u32]) -> u32 {
        let table = e.mem.alloc(0x100);
        for slot in slots {
            e.mem.set_u32(table + slot, 0x7000_0000 + slot);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        object
    }

    /// `(holder, object)`: a word holding the address of `object_with_slots(slots)`.
    fn held_object(e: &mut Engine, slots: &[u32]) -> (u32, u32) {
        let object = object_with_slots(e, slots);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, object);
        (holder, object)
    }

    #[test]
    fn fn_00d921e0_calls_slot_1c_of_the_held_object() {
        let mut e = Engine::new();
        let (holder, object) = held_object(&mut e, &[0x1c]);
        let log = recording_double(&mut e, 0x7000_001c, 0x1111);
        assert_eq!(
            e.call(0x00d9_21e0, &args![0u32, holder, 0u32, 7u32]).u32(),
            0x1111
        );
        assert_eq!(*log.borrow(), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00d92200_calls_slot_20_of_the_held_object_skipping_the_sixth_word() {
        let mut e = Engine::new();
        let (holder, object) = held_object(&mut e, &[0x20]);
        let log = recording_double(&mut e, 0x7000_0020, 0x2222);
        let words = args![0xa0u32, holder, 3u32, 4u32, 5u32, 0xa5u32, 7u32];
        assert_eq!(e.call(0x00d9_2200, &words).u32(), 0x2222);
        assert_eq!(*log.borrow(), vec![vec![object, 3, 4, 5, 7]]);
    }

    #[test]
    fn fn_00d92230_calls_slot_c_of_the_target_with_the_name_and_the_held_object() {
        let mut e = Engine::new();
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, 0x5555);
        let target = object_with_slots(&mut e, &[0xc]);
        let log = recording_double(&mut e, 0x7000_000c, 0x3333);
        let words = args![0u32, holder, 0u32, target];
        assert_eq!(e.call(0x00d9_2230, &words).u32(), 0x3333);
        assert_eq!(*log.borrow(), vec![vec![target, 0x010d_3db0, 0x5555, 0]]);
    }

    #[test]
    fn fn_00d92250_calls_slot_24_of_the_held_object() {
        let mut e = Engine::new();
        let (holder, object) = held_object(&mut e, &[0x24]);
        let log = recording_double(&mut e, 0x7000_0024, 0x4444);
        assert_eq!(
            e.call(0x00d9_2250, &args![0u32, holder, 9u32]).u32(),
            0x4444
        );
        assert_eq!(*log.borrow(), vec![vec![object, 9]]);
    }

    #[test]
    fn fn_00d92270_passes_the_two_floats_quieted_and_the_word() {
        let mut e = Engine::new();
        let (holder, object) = held_object(&mut e, &[0x28]);
        let log = recording_double(&mut e, 0x7000_0028, 0x5555);
        let signalling = f32::from_bits(0x7f80_0001);
        let words = args![0u32, holder, 1.5f32, signalling, 9u32];
        assert_eq!(e.call(0x00d9_2270, &words).u32(), 0x5555);
        assert_eq!(
            *log.borrow(),
            vec![vec![object, 1.5f32.to_bits(), 0x7fc0_0001, 9]]
        );
    }

    #[test]
    fn fn_00d922a0_calls_slot_2c_of_the_held_object() {
        let mut e = Engine::new();
        let (holder, object) = held_object(&mut e, &[0x2c]);
        let log = recording_double(&mut e, 0x7000_002c, 0x6666);
        assert_eq!(
            e.call(0x00d9_22a0, &args![0u32, holder, 9u32]).u32(),
            0x6666
        );
        assert_eq!(*log.borrow(), vec![vec![object, 9]]);
    }

    #[test]
    fn fn_00d922c0_calls_slot_30_of_the_held_object() {
        let mut e = Engine::new();
        let (holder, object) = held_object(&mut e, &[0x30]);
        let log = recording_double(&mut e, 0x7000_0030, 0x7777);
        assert_eq!(
            e.call(0x00d9_22c0, &args![0u32, holder, 9u32]).u32(),
            0x7777
        );
        assert_eq!(*log.borrow(), vec![vec![object, 9]]);
    }

    #[test]
    fn fn_00d922e0_calls_slot_34_of_the_held_object() {
        let mut e = Engine::new();
        let (holder, object) = held_object(&mut e, &[0x34]);
        let log = recording_double(&mut e, 0x7000_0034, 0x8888);
        assert_eq!(
            e.call(0x00d9_22e0, &args![0u32, holder, 9u32]).u32(),
            0x8888
        );
        assert_eq!(*log.borrow(), vec![vec![object, 9]]);
    }

    #[test]
    fn fn_00d92380_calls_slot_18_of_the_agent_object_and_answers_the_data_plus_16() {
        let mut e = Engine::new();
        let (agent_data, object) = held_object(&mut e, &[0x18]);
        let log = recording_double(&mut e, 0x7000_0018, 0);
        let input = e.mem.alloc(0x14);
        e.mem.set_u32(input, 0x11);
        e.mem.set_u32(input + 4, 0x22);
        e.mem.set_u32(input + 0xc, 0x33);
        let words = args![input, 0u32, agent_data, 0u32, 0x44u32];
        assert_eq!(e.call(0x00d9_2380, &words).u32(), agent_data + 0x10);
        assert_eq!(*log.borrow(), vec![vec![object, 0x11, 0x22, 0x33, 0x44]]);
    }
}
