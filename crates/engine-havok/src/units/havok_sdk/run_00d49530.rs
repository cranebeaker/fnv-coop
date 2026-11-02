//! `Havok SDK/run_00d49530` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d495f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_fb34.
pub fn return_constant_00d495f0(_e: &mut Engine) -> u32 {
    0x010c_fb34
}

// Translated from 00d49550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): builds a temporary object in a 0x220-byte frame with
/// the constructor at `00c9ce00` (`this` = the frame, one argument 0), does not destroy it and returns
/// the constant 0x010c_7888 (the word the constructor stored is not read back).
pub fn fn_00d49550(e: &mut Engine) -> u32 {
    e.with_stack(0x220, |e, record| {
        e.call(0x00c9_ce00, &args![record, 0u32]);
    });
    0x010c_7888
}

// Translated from 00d49530 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words `object` and `argument`): when `object` is not null calls
/// `00c9ce00` on it with `argument` (thiscall) and then writes the vtable word `0x010c_7888` at
/// `object + 0`. (The exe's wrapper around a base destructor; the class is unconfirmed.)
pub fn fn_00d49530(e: &mut Engine, object: u32, argument: u32) {
    if object != 0 {
        e.call(0x00c9_ce00, &args![object, argument]);
        e.mem.set_u32(object, 0x010c_7888);
    }
}

// Translated from 00d49510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `object`): calls the object's virtual function at
/// vtable offset 0 (the destructor) with the one stack word 0 and answers what it answers. It is
/// the cleanup function of a type-info record (word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject`
/// calls it with `(object)`): the start-up initializer at `00fbecd9` stores its address there.
pub fn fn_00d49510(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00d49580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `object`): calls the object's virtual function at
/// vtable offset 0 (the destructor) with the one stack word 0 and answers what it answers. It is
/// the cleanup function of a type-info record (word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject`
/// calls it with `(object)`): the start-up initializer at `00fbed59` stores its address there.
pub fn fn_00d49580(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00d495a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words `(object, extra)`): the finish function of a
/// type-info record (word `+4`; the start-up initializer at `00fbed4f` stores its address there,
/// the cleanup function `00d49580` follows at `+8`). Nothing happens for a null `object`.
/// Otherwise it sets the fields a chain of three inlined constructors leaves, in the order they
/// write them: the vtable `010305a8`; with a non-zero `extra` the word at `+0xc` is cleared, and
/// it then becomes `0x19`; the vtable `010ce31c`; with `extra` the word at `+0xc` becomes `0xd`;
/// the vtable `010cfb34`; with `extra` the byte at `+0x1d` is cleared. Answers `object` (`EAX`).
pub fn fn_00d495a0(e: &mut Engine, object: Ptr, extra: u32) -> u32 {
    let base = object.addr();
    if base == 0 {
        return 0;
    }
    e.mem.set_u32(base, 0x0103_05a8);
    if extra != 0 {
        e.mem.set_u32(base.wrapping_add(0xc), 0);
    }
    e.mem.set_u32(base.wrapping_add(0xc), 0x19);
    e.mem.set_u32(base, 0x010c_e31c);
    if extra != 0 {
        e.mem.set_u32(base.wrapping_add(0xc), 0xd);
    }
    e.mem.set_u32(base, 0x010c_fb34);
    if extra != 0 {
        e.mem.set_u8(base.wrapping_add(0x1d), 0);
    }
    base
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d49510, fn_00d49510(Ptr) -> u32),
        entry!(0x00d49580, fn_00d49580(Ptr) -> u32),
        entry!(0x00d495a0, fn_00d495a0(Ptr, u32) -> u32),
        entry!(0x00d495f0, return_constant_00d495f0() -> u32),
        entry!(0x00d49550, fn_00d49550() -> u32),
        entry!(0x00d4_9530, fn_00d49530(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00d49510_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0x0900_0100]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(7),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00d4_9510, &args![object]).u32();
        assert_eq!(answer, object + 7);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0900_0100)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x0900_0100, vec![object, 0])]);
    }

    #[test]
    fn fn_00d49580_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0x0900_0100]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(7),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00d4_9580, &args![object]).u32();
        assert_eq!(answer, object + 7);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0900_0100)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x0900_0100, vec![object, 0])]);
    }

    #[test]
    fn fn_00d495a0_without_extra_leaves_the_base_state() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.mem.set_u8(object + 0x1d, 0x55);
        assert_eq!(e.call(0x00d4_95a0, &args![object, 0u32]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x010c_fb34);
        assert_eq!(e.mem.u32(object + 0xc), 0x19);
        assert_eq!(e.mem.u8(object + 0x1d), 0x55);
    }

    #[test]
    fn fn_00d495a0_with_extra_leaves_the_final_state() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.mem.set_u8(object + 0x1d, 0x55);
        assert_eq!(e.call(0x00d4_95a0, &args![object, 1u32]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x010c_fb34);
        assert_eq!(e.mem.u32(object + 0xc), 0xd);
        assert_eq!(e.mem.u8(object + 0x1d), 0);
    }

    #[test]
    fn fn_00d495a0_leaves_a_null_object_alone() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_95a0, &args![0u32, 1u32]).u32(), 0);
    }

    #[test]
    fn return_constant_00d495f0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_95f0, &args![]).u32(), 0x010c_fb34);
    }
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
    fn fn_00d49550_constructs_a_temporary_and_returns_the_constant() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00c9_ce00, 0x0a0b_0c0d);
        let result = e.call(0x00d4_9550, &args![]).u32();
        assert_eq!(result, 0x010c_7888);
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
    fn fn_00d49530_runs_the_base_and_sets_the_vtable_for_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00c9_ce00, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00d4_9530, &args![object, 0x33u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x00c9_ce00, vec![object, 0x33])]);
        assert_eq!(e.mem.u32(object), 0x010c_7888);
        e.call_log = Some(vec![]);
        e.call(0x00d4_9530, &args![0u32, 0x33u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
