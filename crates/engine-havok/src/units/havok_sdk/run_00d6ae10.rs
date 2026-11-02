//! `Havok SDK/run_00d6ae10` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d6aec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0xf0-byte stack buffer by calling `00c90e00` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d6aec0(e: &mut Engine) -> u32 {
    e.with_stack(0xf0, |e, buffer| {
        e.call(0x00c9_0e00, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00d6ae50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x40-byte frame zeroes the `u16` at +0x24, stores
/// the vtable word 0x010c_c040 at the start, 0 at +0x20 and 0x8000 in the `u16` at +0x26, runs the
/// constructor at `00cb0a70` on the member at frame+0x28 (argument 0) and returns the word at the start of the frame.
pub fn fn_00d6ae50(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, frame| {
        let base = frame.addr();
        e.mem.set_u16(base + 0x24, 0);
        e.mem.set_u32(base, 0x010c_c040);
        e.mem.set_u32(base + 0x20, 0);
        e.mem.set_u16(base + 0x26, 0x8000);
        e.call(0x00cb_0a70, &args![base + 0x28, 0u32]);
        e.mem.u32(base)
    })
}

/// `00cb0a70` (thiscall on the string at `+0x28`, one argument: the text pointer; its body is
/// `MOV EAX,ECX` and `RET 4`, so it is the constructor of a string pointer that keeps its
/// argument unconstructed).
const STRING_POINTER_CONSTRUCT: u32 = 0x00cb_0a70;
/// The vtable pointer the object gets (`010cc040`).
const OBJECT_VTABLE: u32 = 0x010c_c040;

// Translated from 00d6ae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises an object in the memory at `object` (cdecl, two stack words; nothing happens when
/// `object` is null): vtable pointer `010cc040`, the halfword `0x8000` at `+0x26`, zero at `+0x20`
/// and the halfword zero at `+0x24`, then `00cb0a70` on `object + 0x28` with `text`, and
/// `0xfffffff0` at `+0x34`.
pub fn fn_00d6ae10(e: &mut Engine, object: Ptr, text: u32) {
    if object.is_null() {
        return;
    }
    let at = object.addr();
    e.mem.set_u32(at, OBJECT_VTABLE);
    e.mem.set_u16(at.wrapping_add(0x26), 0x8000);
    e.mem.set_u32(at.wrapping_add(0x20), 0);
    e.mem.set_u16(at.wrapping_add(0x24), 0);
    e.call(
        STRING_POINTER_CONSTRUCT,
        &args![at.wrapping_add(0x28), text],
    );
    e.mem.set_u32(at.wrapping_add(0x34), 0xffff_fff0);
}

// Translated from 00d6add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b6254): cdecl with one stack word (never read); only returns.
pub fn fn_00d6add0(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00d6adf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbf519): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00d6adf0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00d6ae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbf5ff): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00c90e00` (the `CALL` at 00d6ae9d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00d6ae90(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00c9_0e00, &args![object.addr(), finishing]);
    }
}

// Translated from 00d6aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbf609): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00d6aeb0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_ae10, fn_00d6ae10(Ptr, u32)),
        entry!(0x00d6_aec0, fn_00d6aec0() -> u32),
        entry!(0x00d6ae50, fn_00d6ae50() -> u32),
        entry!(0x00d6add0, fn_00d6add0(u32)),
        entry!(0x00d6adf0, fn_00d6adf0(Ptr) -> u32),
        entry!(0x00d6ae90, fn_00d6ae90(Ptr, u32)),
        entry!(0x00d6aeb0, fn_00d6aeb0(Ptr) -> u32),
    ]
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
        fn_00d6aec0_returns_first_word_of_built_object,
        0x00d6_aec0,
        0x00c9_0e00
    );
}

#[cfg(test)]
mod tests {
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
    fn fn_00d6ae50_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let frame = e.mem.alloc(0x40);
        for offset in (0..0x40).step_by(4) {
            e.mem.set_u32(frame + offset, 0xffff_ffff);
        }
        e.mem.free(frame);
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x28], true);
        let result = e.call(0x00d6_ae50, &args![]).u32();
        assert_eq!(result, 0x010c_c040);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c040);
        }
        assert_eq!(calls[0].3, [0, 0, 0x8000]);
    }

    #[test]
    fn fn_00d6aec0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00c9_0e00, 0x0a0b_0c0d);
        let result = e.call(0x00d6_aec0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }
}

#[cfg(test)]
mod object_init_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    #[test]
    fn fn_00d6ae10_initialises_the_object() {
        let mut e = Engine::new();
        let seen: Seen = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(STRING_POINTER_CONSTRUCT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x20, 0xffff_ffff);
        e.mem.set_u16(object + 0x24, 0x1234);
        e.call(0x00d6_ae10, &args![object, 0x7777u32]);
        assert_eq!(e.mem.u32(object), OBJECT_VTABLE);
        assert_eq!(e.mem.u16(object + 0x26), 0x8000);
        assert_eq!(e.mem.u32(object + 0x20), 0);
        assert_eq!(e.mem.u16(object + 0x24), 0);
        assert_eq!(e.mem.u32(object + 0x34), 0xffff_fff0);
        assert_eq!(*seen.borrow(), vec![vec![object + 0x28, 0x7777]]);
    }

    #[test]
    fn fn_00d6ae10_does_nothing_for_a_null_object() {
        let mut e = Engine::new();
        let seen: Seen = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(STRING_POINTER_CONSTRUCT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00d6_ae10, &args![0u32, 0x7777u32]);
        assert!(seen.borrow().is_empty());
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

    /// The address the doubled virtual slot 0 of [`object_with_slot_0`] answers at.
    const SLOT_0_TARGET: u32 = 0x0900_0001;

    /// An object whose vtable's slot 0 is a double answering 0x1234.
    fn object_with_slot_0(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable, SLOT_0_TARGET);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.register(SLOT_0_TARGET, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        object
    }

    #[test]
    fn fn_00d6add0_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d6_add0, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00d6adf0_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d6_adf0, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00d6ae90_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00c9_0e00, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00d6_ae90, &args![0u32, 7u32]);
        e.call(0x00d6_ae90, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00c9_0e00), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00d6aeb0_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d6_aeb0, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }
}
