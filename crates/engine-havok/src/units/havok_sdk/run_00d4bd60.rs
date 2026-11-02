//! `Havok SDK/run_00d4bd60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d4be00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_b400`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d4be00(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_b400, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d4bdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `00d4be00`. `00d4be00` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d4bdf0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d4_be00, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 00d4be60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x50-byte stack buffer by calling `00d4b3b0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4be60(e: &mut Engine) -> u32 {
    e.with_stack(0x50, |e, buffer| {
        e.call(0x00d4_b3b0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00d4bec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x70-byte stack buffer by calling `00d4ba30` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4bec0(e: &mut Engine) -> u32 {
    e.with_stack(0x70, |e, buffer| {
        e.call(0x00d4_ba30, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00d4bf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x60-byte stack buffer by calling `00dcd380` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00d4bf40(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, buffer| {
        e.call(0x00dc_d380, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00d4bd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`__fastcall`, no stack words; class unconfirmed): frees two `hkArray`s and writes
/// the base vtable words. After `00c85750`, for the 16-bit array at `+0x40` (size `+0x44`,
/// capacity word `+0x48`): sets the size to 0 and, when the capacity word's sign bit is clear,
/// frees the data through slot `0x10` of the router's allocator (`(capacity & 0x3fffffff) * 2`
/// bytes); clears the data pointer and makes the capacity word `0x80000000`. The same, after
/// another `00c85750`, for the array of 0x38-byte elements at `+0x34` (size `+0x38`, capacity word
/// `+0x3c`, `(capacity & 0x3fffffff) * 0x38` bytes). Then `+0x10` = `0x010c_6fb0` and `+0` =
/// `0x0102_e388`.
pub fn fn_00d4bd60(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x48);
    e.mem.set_u32(me + 0x44, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0x40);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(2);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0x40, 0);
    e.mem.set_u32(me + 0x48, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x3c);
    e.mem.set_u32(me + 0x38, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0x34);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(0x38);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0x34, 0);
    e.mem.set_u32(me + 0x3c, 0x8000_0000);
    e.mem.set_u32(me + 0x10, 0x010c_6fb0);
    e.mem.set_u32(me, 0x0102_e388);
}

// Translated from 00d4be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbf0ff): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00d4b3b0` (the `CALL` at 00d4be3d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00d4be30(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00d4_b3b0, &args![object.addr(), finishing]);
    }
}

// Translated from 00d4be50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbf109): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00d4be50(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00d4be90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbf16f): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00d4ba30` (the `CALL` at 00d4be9d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00d4be90(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00d4_ba30, &args![object.addr(), finishing]);
    }
}

// Translated from 00d4beb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbf179): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00d4beb0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00d4bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b5668): cdecl with one stack word (never read); only returns.
pub fn fn_00d4bef0(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00d4bf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbf24f): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00dcd380` (the `CALL` at 00d4bf1d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00d4bf10(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00dc_d380, &args![object.addr(), finishing]);
    }
}

// Translated from 00d4bf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbf259): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00d4bf30(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_be60, fn_00d4be60() -> u32),
        entry!(0x00d4_bec0, fn_00d4bec0() -> u32),
        entry!(0x00d4_bf40, fn_00d4bf40() -> u32),
        entry!(0x00d4_be00, deleting_destructor_00d4be00(Ptr, u8) -> Ptr),
        entry!(0x00d4_bdf0, fn_00d4bdf0(Ptr, u32) -> u32),
        entry!(0x00d4_bd60, fn_00d4bd60(Ptr)),
        entry!(0x00d4be30, fn_00d4be30(Ptr, u32)),
        entry!(0x00d4be50, fn_00d4be50(Ptr) -> u32),
        entry!(0x00d4be90, fn_00d4be90(Ptr, u32)),
        entry!(0x00d4beb0, fn_00d4beb0(Ptr) -> u32),
        entry!(0x00d4bef0, fn_00d4bef0(u32)),
        entry!(0x00d4bf10, fn_00d4bf10(Ptr, u32)),
        entry!(0x00d4bf30, fn_00d4bf30(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d4be00_frees_only_when_bit_zero_is_set() {
        check(0x00d4_be00, 0x00d4_b400);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00d4bdf0() {
        assert_eq!(
            check(0x00d4_bdf0, 0x00d4_be00, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
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
        fn_00d4be60_returns_first_word_of_built_object,
        0x00d4_be60,
        0x00d4_b3b0
    );
    first_word_test!(
        fn_00d4bec0_returns_first_word_of_built_object,
        0x00d4_bec0,
        0x00d4_ba30
    );
    first_word_test!(
        fn_00d4bf40_returns_first_word_of_built_object,
        0x00d4_bf40,
        0x00dc_d380
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

    #[test]
    fn fn_00d4be60_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d4_b3b0, 0x0a0b_0c0d);
        let result = e.call(0x00d4_be60, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00d4bec0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d4_ba30, 0x0a0b_0c0d);
        let result = e.call(0x00d4_bec0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00d4bf40_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00dc_d380, 0x0a0b_0c0d);
        let result = e.call(0x00d4_bf40, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d4bd60_frees_both_arrays() {
        for (capacities, freed) in [
            ((0x4000_0004u32, 0x4000_0002u32), true),
            ((0x8000_0004, 0x8000_0002), false),
        ] {
            let mut e = Engine::new();
            let vtable = e.mem.alloc(0x40);
            e.mem.set_u32(vtable + 0x10, 0x0900_0101);
            let allocator = e.mem.alloc(0x10);
            e.mem.set_u32(allocator, vtable);
            let router = e.mem.alloc(0x20);
            e.mem.set_u32(router + 0x10, allocator);
            e.register_double(MEMORY_ROUTER, move |_, _| Ret {
                eax: router,
                ..Ret::default()
            });
            e.register(0x0900_0101, |_, _| Ret::default());
            let this = e.mem.alloc(0x60);
            e.mem.set_u32(this + 0x34, 0x7000);
            e.mem.set_u32(this + 0x38, 5);
            e.mem.set_u32(this + 0x3c, capacities.1);
            e.mem.set_u32(this + 0x40, 0x8000);
            e.mem.set_u32(this + 0x44, 9);
            e.mem.set_u32(this + 0x48, capacities.0);
            e.call_log = Some(vec![]);
            e.call(0x00d4_bd60, &args![this]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(MEMORY_ROUTER, vec![])];
            if freed {
                expected.push((0x0900_0101, vec![allocator, 0x8000, 8]));
            }
            expected.push((MEMORY_ROUTER, vec![]));
            if freed {
                expected.push((0x0900_0101, vec![allocator, 0x7000, 2 * 0x38]));
            }
            assert_eq!(log[1..], expected[..]);
            assert_eq!(e.mem.u32(this + 0x40), 0);
            assert_eq!(e.mem.u32(this + 0x44), 0);
            assert_eq!(e.mem.u32(this + 0x48), 0x8000_0000);
            assert_eq!(e.mem.u32(this + 0x34), 0);
            assert_eq!(e.mem.u32(this + 0x38), 0);
            assert_eq!(e.mem.u32(this + 0x3c), 0x8000_0000);
            assert_eq!(e.mem.u32(this + 0x10), 0x010c_6fb0);
            assert_eq!(e.mem.u32(this), 0x0102_e388);
        }
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
    fn fn_00d4be30_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00d4_b3b0, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00d4_be30, &args![0u32, 7u32]);
        e.call(0x00d4_be30, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00d4_b3b0), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00d4be50_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_be50, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00d4be90_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00d4_ba30, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00d4_be90, &args![0u32, 7u32]);
        e.call(0x00d4_be90, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00d4_ba30), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00d4beb0_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_beb0, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00d4bef0_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_bef0, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00d4bf10_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00dc_d380, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00d4_bf10, &args![0u32, 7u32]);
        e.call(0x00d4_bf10, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00dc_d380), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00d4bf30_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_bf30, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }
}
