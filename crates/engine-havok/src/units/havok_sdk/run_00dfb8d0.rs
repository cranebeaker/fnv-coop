//! `Havok SDK/run_00dfb8d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfb8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_e0ec.
pub fn return_constant_00dfb8d0(_e: &mut Engine) -> u32 {
    0x010d_e0ec
}

// Translated from 00dfb850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b9e88): cdecl with one stack word (never read); only returns.
pub fn fn_00dfb850(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfb870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b9ea0): cdecl with one stack word (never read); only returns.
pub fn fn_00dfb870(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfb890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc1609): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfb890(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfb8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc15ff): cdecl with two stack words, `object` and `finishing`. When `object` is not null, stores the vtable word 0x010d_e0ec at its start and, when `finishing` is not 0, jumps to `00e31900` (fastcall, no stack word) with the object in `ECX`; the jump's result is not returned (the function is void).
pub fn fn_00dfb8b0(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() == 0 {
        return;
    }
    e.mem.set_u32(object.addr(), 0x010d_e0ec);
    if finishing != 0 {
        e.call(0x00e3_1900, &args![object.addr()]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfb8d0, return_constant_00dfb8d0() -> u32),
        entry!(0x00dfb850, fn_00dfb850(u32)),
        entry!(0x00dfb870, fn_00dfb870(u32)),
        entry!(0x00dfb890, fn_00dfb890(Ptr) -> u32),
        entry!(0x00dfb8b0, fn_00dfb8b0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfb8d0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_b8d0, &args![]).u32(), 0x010d_e0ec);
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
    fn fn_00dfb850_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_b850, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00dfb870_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_b870, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00dfb890_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_b890, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfb8b0_stores_the_vtable_and_runs_the_finish_only_when_finishing() {
        let mut e = Engine::new();
        e.register(0x00e3_1900, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00df_b8b0, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00df_b8b0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010d_e0ec);
        assert!(calls_to(&mut e, 0x00e3_1900).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x00df_b8b0, &args![object, 1u32]);
        assert_eq!(calls_to(&mut e, 0x00e3_1900), vec![vec![object]]);
    }
}
