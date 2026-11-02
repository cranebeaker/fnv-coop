//! `Havok SDK/run_00d47f90` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d47f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x0126_8b54.
pub fn return_constant_00d47f90(_e: &mut Engine) -> u32 {
    0x0126_8b54
}

/// The memory router (`00c85750`, no arguments): its word at `+0x10` is the allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Releases one reference (`00c905b0`, fastcall: the referenced object in `ECX`).
const RELEASE_REFERENCE: u32 = 0x00c9_05b0;
/// The `hkStringPtr` destructor (`00cb0c00`, fastcall: the string in `ECX`).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;

// Translated from 00d47f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b4e08): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00d48300` (the `CALL` at 00d47f4d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00d47f40(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00d4_8300, &args![object.addr(), finishing]);
    }
}

// Translated from 00d47f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b4e20): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00d48320` (the `CALL` at 00d47f7d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00d47f70(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00d4_8320, &args![object.addr(), finishing]);
    }
}

// Translated from 00d47fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b4e0c): cdecl with one stack word, `object`. When the word at `object + 8` is not 0, releases it (`00c905b0`); then clears that word, runs the `hkStringPtr` destructor (`00cb0c00`) on `object + 4` and jumps to it again for `object`, so it answers what the last one answers.
pub fn fn_00d47fa0(e: &mut Engine, object: Ptr) -> u32 {
    let this = object.addr();
    let referenced = e.mem.u32(this + 8);
    if referenced != 0 {
        e.call(RELEASE_REFERENCE, &args![referenced]);
    }
    e.mem.set_u32(this + 8, 0);
    e.call(STRING_PTR_DESTRUCT, &args![this + 4]);
    e.call(STRING_PTR_DESTRUCT, &args![this]).u32()
}

// Translated from 00d47fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b4e24): cdecl with one stack word, `object`. Runs [`fn_00d47fe0`] on it with the flag word 0 and answers what it answers (the object).
pub fn fn_00d47fd0(e: &mut Engine, object: Ptr) -> Ptr {
    fn_00d47fe0(e, object, 0)
}

// Translated from 00d47fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name; thiscall with one stack word (`RET 4`), `this` and `flags`: an array of 12-byte elements (`+0` the data pointer, `+4` the size, `+8` the capacity and flags, bit 31 set when the block is not owned) whose elements each hold two `hkStringPtr` (at `+0` and `+4`) and a referenced object at `+8`. Takes the allocator from the memory router (`00c85750`, its word at `+0x10`). For each element in order releases the object at `+8` (when not 0, `00c905b0`), clears the word and runs the `hkStringPtr` destructor (`00cb0c00`) on `+4` and then `+0`. Clears the size; when the capacity word is not negative frees the block through the allocator's virtual slot `0x10` with the data pointer and `(capacity & 0x3fffffff) * 12` bytes. Clears the data pointer and sets the capacity word to `0x80000000`. When bit 0 of `flags` is set, frees the array itself through the allocator (read again from the memory router) virtual slot 8 with `this` and 0xc bytes. Answers `this`.
pub fn fn_00d47fe0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let array = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let count = e.mem.u32(array + 4) as i32;
    let data = e.mem.u32(array);
    if count > 0 {
        let mut element = data + 8;
        for _ in 0..count {
            let referenced = e.mem.u32(element);
            if referenced != 0 {
                e.call(RELEASE_REFERENCE, &args![referenced]);
            }
            e.mem.set_u32(element, 0);
            e.call(STRING_PTR_DESTRUCT, &args![element - 4]);
            e.call(STRING_PTR_DESTRUCT, &args![element - 8]);
            element += 0xc;
        }
    }
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(12);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![array, 0xc]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d47f90, return_constant_00d47f90() -> u32),
        entry!(0x00d47f40, fn_00d47f40(Ptr, u32)),
        entry!(0x00d47f70, fn_00d47f70(Ptr, u32)),
        entry!(0x00d47fa0, fn_00d47fa0(Ptr) -> u32),
        entry!(0x00d47fd0, fn_00d47fd0(Ptr) -> Ptr),
        entry!(0x00d47fe0, fn_00d47fe0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00d47f90_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_7f90, &args![]).u32(), 0x0126_8b54);
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
    fn fn_00d47f40_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00d4_8300, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00d4_7f40, &args![0u32, 7u32]);
        e.call(0x00d4_7f40, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00d4_8300), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00d47f70_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00d4_8320, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00d4_7f70, &args![0u32, 7u32]);
        e.call(0x00d4_7f70, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00d4_8320), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00d47fa0_releases_the_object_and_destructs_both_strings() {
        let mut e = Engine::new();
        e.register(RELEASE_REFERENCE, |_, _| Ret::default());
        e.register(STRING_PTR_DESTRUCT, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        // With nothing referenced only the strings are destructed.
        assert_eq!(e.call(0x00d4_7fa0, &args![object]).u32(), 0x77);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(address, _)| *address == RELEASE_REFERENCE));
        e.mem.set_u32(object + 8, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x00d4_7fa0, &args![object]);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (RELEASE_REFERENCE, vec![0x1234]),
                (STRING_PTR_DESTRUCT, vec![object + 4]),
                (STRING_PTR_DESTRUCT, vec![object]),
            ]
        );
        assert_eq!(e.mem.u32(object + 8), 0);
    }

    /// An array of two elements (the second without a referenced object), the allocator
    /// behind the memory router with its slots 8 and 0x10 doubled; returns
    /// `(array, data, allocator)`.
    fn array_rig(e: &mut Engine, capacity: u32) -> (u32, u32, u32) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 8, 0x0900_0008);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(RELEASE_REFERENCE, |_, _| Ret::default());
        e.register(STRING_PTR_DESTRUCT, |_, _| Ret::default());
        e.register(0x0900_0008, |_, _| Ret::default());
        e.register(0x0900_0010, |_, _| Ret::default());
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + 8, 0x1111);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 2);
        e.mem.set_u32(array + 8, capacity);
        (array, data, allocator)
    }

    #[test]
    fn fn_00d47fe0_destructs_the_elements_then_frees_the_block() {
        let mut e = Engine::new();
        let (array, data, allocator) = array_rig(&mut e, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_7fe0, &args![array, 0u32]).u32(), array);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|(address, _)| {
                [RELEASE_REFERENCE, STRING_PTR_DESTRUCT, 0x0900_0010].contains(address)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                (RELEASE_REFERENCE, vec![0x1111]),
                (STRING_PTR_DESTRUCT, vec![data + 4]),
                (STRING_PTR_DESTRUCT, vec![data]),
                (STRING_PTR_DESTRUCT, vec![data + 0x10]),
                (STRING_PTR_DESTRUCT, vec![data + 0xc]),
                (0x0900_0010, vec![allocator, data, 48]),
            ]
        );
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
    }

    #[test]
    fn fn_00d47fe0_frees_the_array_itself_when_flag_bit_0_is_set() {
        let mut e = Engine::new();
        let (array, _, allocator) = array_rig(&mut e, 4);
        e.call_log = Some(vec![]);
        e.call(0x00d4_7fe0, &args![array, 1u32]);
        assert_eq!(
            calls_to(&mut e, 0x0900_0008),
            vec![vec![allocator, array, 0xc]]
        );
    }

    #[test]
    fn fn_00d47fe0_keeps_a_block_it_does_not_own() {
        let mut e = Engine::new();
        let (array, _, _) = array_rig(&mut e, 0x8000_0004);
        e.call_log = Some(vec![]);
        e.call(0x00d4_7fe0, &args![array, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(address, _)| *address == 0x0900_0010));
        assert!(!log.iter().any(|(address, _)| *address == 0x0900_0008));
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
    }

    #[test]
    fn fn_00d47fe0_skips_the_loop_for_an_empty_array() {
        let mut e = Engine::new();
        let (array, _, _) = array_rig(&mut e, 4);
        e.mem.set_u32(array + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00d4_7fe0, &args![array, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log
            .iter()
            .any(|(address, _)| *address == STRING_PTR_DESTRUCT));
    }

    #[test]
    fn fn_00d47fd0_destructs_with_the_flag_word_zero() {
        let mut e = Engine::new();
        let (array, _, _) = array_rig(&mut e, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_7fd0, &args![array]).u32(), array);
        assert!(calls_to(&mut e, 0x0900_0008).is_empty());
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
    }
}
