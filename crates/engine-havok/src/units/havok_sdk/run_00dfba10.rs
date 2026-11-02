//! `Havok SDK/run_00dfba10` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfbb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_e90c.
pub fn return_constant_00dfbb70(_e: &mut Engine) -> u32 {
    0x010d_e90c
}

// Translated from 00dfba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00e34800` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfba80(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00e3_4800, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfba10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x50-byte frame stores the vtable word 0x010c_4dec at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x8 (argument 0) and
/// returns the word at the start of the frame. Nothing is destroyed.
pub fn fn_00dfba10(e: &mut Engine) -> u32 {
    e.with_stack(0x50, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_4dec);
        e.call(0x00cb_0a70, &args![base + 0x8, 0u32]);
        e.mem.u32(base)
    })
}

// Translated from 00dfb910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011ba09c). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00dfb910(_e: &mut Engine) {}

// Translated from 00dfb930 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011ba0b4). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00dfb930(_e: &mut Engine) {}

// Translated from 00dfb950 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011ba0c8). Cdecl `(object, flag)`: does nothing
/// for a null `object`; otherwise calls `00ce5360` (thiscall on `object` with `flag` as its one stack word).
pub fn fn_00dfb950(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00ce_5360, &args![object, flag]);
    }
}

// Translated from 00dfb980 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011ba0cc). Cdecl
/// `(object)`: loads `ECX` from its one stack word and tail-jumps to `00ce5370` (thiscall, no
/// stack words), so it answers that function's result.
pub fn fn_00dfb980(e: &mut Engine, object: Ptr) -> u32 {
    e.call(0x00ce_5370, &args![object]).u32()
}

// Translated from 00dfb9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc1749). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfb9a0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfb9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011ba230). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00dfb9c0(_e: &mut Engine) {}

// Translated from 00dfb9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011ba234). Cdecl
/// `(slot)`: `slot` is the address of a pointer. When that pointer is not null, calls `00c905b0` on
/// it (thiscall, no stack words); the pointer is set to null either way.
pub fn fn_00dfb9d0(e: &mut Engine, slot: Ptr) {
    let held = e.mem.u32(slot.addr());
    if held != 0 {
        e.call(0x00c9_05b0, &args![held]);
    }
    e.mem.set_u32(slot.addr(), 0);
}

// Translated from 00dfb9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc173f, the static
/// initializer that also installs `00dfba10` and `00dfb9a0`). Cdecl `(object, flag)`. For a non-null
/// `object`: stores the vtable word `0x010c_4dec` (RTTI `hkaSkeleton`) at `+0` and runs the
/// constructor `00cb0a70` (thiscall, `RET 4`) on the member at `+8` with `flag`.
pub fn fn_00dfb9f0(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    e.mem.set_u32(object.addr(), 0x010c_4dec);
    e.call(0x00cb_0a70, &args![object.byte_add(8), flag]);
}

// Translated from 00dfba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc17df). Cdecl `(object, flag)`: does nothing
/// for a null `object`; otherwise calls `00e34800` (thiscall on `object` with `flag` as its one stack word).
pub fn fn_00dfba50(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00e3_4800, &args![object, flag]);
    }
}

// Translated from 00dfba70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc17e9). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfba70(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfbab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011ba274). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00dfbab0(_e: &mut Engine) {}

// Translated from 00dfbac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011ba278). Cdecl
/// `(array)`: empties an array of 2-byte elements laid out as data pointer `+0`, size `+4`, capacity
/// word `+8` (the memory router `00c85750`'s allocator is at `+0x10` of the router). Clears the
/// size; when the capacity word's sign bit is clear, frees the data through slot `0x10` of the
/// allocator with `(data, 2 * (capacity & 0x3fffffff))`; then the capacity word becomes
/// `0x80000000` and the data pointer null.
pub fn fn_00dfbac0(e: &mut Engine, array: Ptr) {
    let me = array.addr();
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let capacity = e.mem.u32(me.wrapping_add(8));
    e.mem.set_u32(me.wrapping_add(4), 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me);
        let masked = capacity & 0x3fff_ffff;
        e.vcall(allocator, 0x10, &args![data, masked.wrapping_add(masked)]);
    }
    e.mem.set_u32(me.wrapping_add(8), 0x8000_0000);
    e.mem.set_u32(me, 0);
}

// Translated from 00dfbb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011ba300). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00dfbb00(_e: &mut Engine) {}

// Translated from 00dfbb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011ba304). A bare `RET`: it reads no stack
/// word and leaves no result.
pub fn fn_00dfbb10(_e: &mut Engine) {}

// Translated from 00dfbb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc1889). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfbb30(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfbb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc187f). Cdecl
/// `(object, flag)`. For a non-null `object`: stores the vtable word `0x010d_e90c` (RTTI
/// `hkaDeltaCompressedAnimation`) at `+0`, then, when `flag` is not 0, tail-jumps to `00e35050`
/// (thiscall on `object`, no stack words).
pub fn fn_00dfbb50(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    e.mem.set_u32(object.addr(), 0x010d_e90c);
    if flag != 0 {
        e.call(0x00e3_5050, &args![object]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_ba80, fn_00dfba80() -> u32),
        entry!(0x00dfbb70, return_constant_00dfbb70() -> u32),
        entry!(0x00dfba10, fn_00dfba10() -> u32),
        entry!(0x00df_b910, fn_00dfb910()),
        entry!(0x00df_b930, fn_00dfb930()),
        entry!(0x00df_b950, fn_00dfb950(Ptr, u32)),
        entry!(0x00df_b980, fn_00dfb980(Ptr) -> u32),
        entry!(0x00df_b9a0, fn_00dfb9a0(Ptr) -> u32),
        entry!(0x00df_b9c0, fn_00dfb9c0()),
        entry!(0x00df_b9d0, fn_00dfb9d0(Ptr)),
        entry!(0x00df_b9f0, fn_00dfb9f0(Ptr, u32)),
        entry!(0x00df_ba50, fn_00dfba50(Ptr, u32)),
        entry!(0x00df_ba70, fn_00dfba70(Ptr) -> u32),
        entry!(0x00df_bab0, fn_00dfbab0()),
        entry!(0x00df_bac0, fn_00dfbac0(Ptr)),
        entry!(0x00df_bb00, fn_00dfbb00()),
        entry!(0x00df_bb10, fn_00dfbb10()),
        entry!(0x00df_bb30, fn_00dfbb30(Ptr) -> u32),
        entry!(0x00df_bb50, fn_00dfbb50(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfbb70_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_bb70, &args![]).u32(), 0x010d_e90c);
    }
}

#[cfg(test)]
mod a07_first_word_tests {
    use crate::prelude::*;

    const BUILT_WORD: u32 = 0x0abc_def0;

    /// A double over `callee` writes `BUILT_WORD` at the buffer it is given; `function`
    /// must call it once with that buffer and a zero flag word, and return the word.
    fn check(function: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |e, a| {
            e.mem.set_u32(a[0], BUILT_WORD);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(function, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0].1[0], 0);
        assert_eq!(calls[0].1[1], 0);
        assert_eq!(result, BUILT_WORD);
    }

    macro_rules! first_word_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check($function, $callee);
            }
        };
    }

    first_word_test!(
        fn_00dfba80_returns_first_word_of_built_object,
        0x00df_ba80,
        0x00e3_4800
    );
}

#[cfg(test)]
mod record_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type ConstructorLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Double for a constructor: logs its argument words and writes `vtable` at the start of `this`.
    fn constructor_double(e: &mut Engine, address: u32, vtable: u32) -> ConstructorLog {
        let log: ConstructorLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], vtable);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    /// What the probe saw on one call: `this`, the argument, the word `behind` bytes before `this`
    /// (the frame's vtable slot) and, when asked for, the `u32` at +0x20, `u16` at +0x24 and `u16`
    /// at +0x26 of the frame.
    type ProbeCall = (u32, u32, u32, [u32; 3]);
    type ProbeLog = Rc<RefCell<Vec<ProbeCall>>>;

    /// Double for the member constructor: logs, per call, what `ProbeCall` describes, with the
    /// vtable slot `behind[call]` bytes before `this`.
    fn frame_probe(e: &mut Engine, address: u32, behind: &[u32], with_tail: bool) -> ProbeLog {
        let log: ProbeLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        let behind = behind.to_vec();
        e.register_double(address, move |e, a| {
            let call = seen.borrow().len();
            let slot = a[0] - behind[call];
            let mut tail = [0; 3];
            if with_tail {
                tail = [
                    e.mem.u32(slot + 0x20),
                    u32::from(e.mem.u16(slot + 0x24)),
                    u32::from(e.mem.u16(slot + 0x26)),
                ];
            }
            seen.borrow_mut().push((a[0], a[1], e.mem.u32(slot), tail));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn fn_00dfba10_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x8], false);
        let result = e.call(0x00df_ba10, &args![]).u32();
        assert_eq!(result, 0x010c_4dec);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_4dec);
        }
    }

    #[test]
    fn fn_00dfba80_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e3_4800, 0x0a0b_0c0d);
        let result = e.call(0x00df_ba80, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
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

    /// A bare `RET` function runs, answers nothing and calls nothing.
    fn check_ret(function: u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(function, &args![]), Ret::default());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    macro_rules! ret_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_ret($function);
            }
        };
    }

    /// `function(object, flag)` calls `callee(object, flag)` for a non-null object and nothing for
    /// a null one.
    fn check_finish(function: u32, callee: u32) {
        let mut e = Engine::new();
        let log = recording_double(&mut e, callee, 0);
        e.call(function, &args![0u32, 1u32]);
        assert!(log.borrow().is_empty());
        e.call(function, &args![0x4000u32, 1u32]);
        e.call(function, &args![0x4000u32, 0u32]);
        assert_eq!(*log.borrow(), vec![vec![0x4000, 1], vec![0x4000, 0]]);
    }

    macro_rules! finish_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check_finish($function, $callee);
            }
        };
    }

    /// `function(object)` calls slot 0 of the object's vtable with `(object, 0)` and answers
    /// what the slot answers.
    fn check_cleanup(function: u32) {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x7000_0000, 0x1234_5678);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, 0x7000_0000);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, table);
        assert_eq!(e.call(function, &args![object]).u32(), 0x1234_5678);
        assert_eq!(*log.borrow(), vec![vec![object, 0]]);
    }

    macro_rules! cleanup_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_cleanup($function);
            }
        };
    }

    ret_test!(fn_00dfb910_does_nothing, 0x00df_b910);
    ret_test!(fn_00dfb930_does_nothing, 0x00df_b930);
    ret_test!(fn_00dfb9c0_does_nothing, 0x00df_b9c0);
    ret_test!(fn_00dfbab0_does_nothing, 0x00df_bab0);
    ret_test!(fn_00dfbb00_does_nothing, 0x00df_bb00);
    ret_test!(fn_00dfbb10_does_nothing, 0x00df_bb10);
    finish_test!(
        fn_00dfb950_calls_00ce5360_for_a_non_null_object,
        0x00df_b950,
        0x00ce_5360
    );
    finish_test!(
        fn_00dfba50_calls_00e34800_for_a_non_null_object,
        0x00df_ba50,
        0x00e3_4800
    );
    cleanup_test!(fn_00dfb9a0_calls_virtual_slot_0_with_zero, 0x00df_b9a0);
    cleanup_test!(fn_00dfba70_calls_virtual_slot_0_with_zero, 0x00df_ba70);
    cleanup_test!(fn_00dfbb30_calls_virtual_slot_0_with_zero, 0x00df_bb30);

    #[test]
    fn fn_00dfb980_hands_the_object_to_00ce5370_and_answers_its_result() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00ce_5370, 0x77);
        assert_eq!(e.call(0x00df_b980, &args![0x4000u32]).u32(), 0x77);
        assert_eq!(*log.borrow(), vec![vec![0x4000]]);
    }

    #[test]
    fn fn_00dfb9d0_releases_the_held_pointer_and_clears_the_slot() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00c9_05b0, 0);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x5000);
        e.call(0x00df_b9d0, &args![slot]);
        assert_eq!(*log.borrow(), vec![vec![0x5000]]);
        assert_eq!(e.mem.u32(slot), 0);
        // An empty slot calls nothing and stays empty.
        e.call(0x00df_b9d0, &args![slot]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(e.mem.u32(slot), 0);
    }

    #[test]
    fn fn_00dfb9f0_stores_the_vtable_and_constructs_the_member_at_8() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00cb_0a70, 0);
        let object = e.mem.alloc(0x20);
        e.call(0x00df_b9f0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_4dec);
        assert_eq!(*log.borrow(), vec![vec![object + 8, 1]]);
        // A null object is left alone.
        e.call(0x00df_b9f0, &args![0u32, 1u32]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn fn_00dfbac0_frees_the_buffer_unless_the_capacity_word_is_negative() {
        for (capacity, freed) in [(0x4000_0005u32, true), (0x8000_0005, false)] {
            let mut e = Engine::new();
            let free_log = recording_double(&mut e, 0x7000_0010, 0);
            let allocator_table = e.mem.alloc(0x20);
            e.mem.set_u32(allocator_table + 0x10, 0x7000_0010);
            let allocator = e.mem.alloc(0x10);
            e.mem.set_u32(allocator, allocator_table);
            let router = e.mem.alloc(0x20);
            e.mem.set_u32(router + 0x10, allocator);
            recording_double(&mut e, 0x00c8_5750, router);
            let array = e.mem.alloc(0x10);
            e.mem.set_u32(array, 0x6000);
            e.mem.set_u32(array + 4, 3);
            e.mem.set_u32(array + 8, capacity);
            e.call(0x00df_bac0, &args![array]);
            if freed {
                assert_eq!(*free_log.borrow(), vec![vec![allocator, 0x6000, 10]]);
            } else {
                assert!(free_log.borrow().is_empty());
            }
            assert_eq!(e.mem.u32(array), 0);
            assert_eq!(e.mem.u32(array + 4), 0);
            assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        }
    }

    #[test]
    fn fn_00dfbb50_stores_the_vtable_and_runs_00e35050_only_when_flagged() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00e3_5050, 0);
        let object = e.mem.alloc(0x20);
        e.call(0x00df_bb50, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010d_e90c);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(object, 0);
        e.call(0x00df_bb50, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010d_e90c);
        assert_eq!(*log.borrow(), vec![vec![object]]);
        // A null object is left alone.
        e.call(0x00df_bb50, &args![0u32, 1u32]);
        assert_eq!(log.borrow().len(), 1);
    }
}
