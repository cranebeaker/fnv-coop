//! `Havok SDK/run_00dfc290` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfc290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_00cc.
pub fn return_constant_00dfc290(_e: &mut Engine) -> u32 {
    0x010e_00cc
}

// Translated from 00dfc2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_00dc.
pub fn return_constant_00dfc2b0(_e: &mut Engine) -> u32 {
    0x010e_00dc
}

// Translated from 00dfc270 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpTyremarkPoint` (the record at `011ba864` of the data table, name pointer `010e00f0`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00dfc270(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00dfc210 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpTyremarkPoint` (the record at `011ba864` of the data table, name pointer `010e00f0`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00dfc210(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00dfc280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpTyremarksWheel` (the record at `0126b480`, name pointer `010e0104`, that the static initializer `00fc1da0` fills). When `object` is not null, stores the word `010e00cc` (the record's word `+0xc`, which the record's builder function returns) at the start of the object; the second stack word is never read. A null object does nothing.
pub fn fn_00dfc280(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_00cc);
    }
}

// Translated from 00dfc230 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpTyremarksWheel` (the record at `0126b480`, name pointer `010e0104`, that the static initializer `00fc1da0` fills). Calls the object's virtual function in vtable slot `0` (`this` = the object) with the one stack word `0`. There is no null test: the object's vtable is read at once. The slot has the shape of a scalar deleting destructor called with flag 0 (unconfirmed).
pub fn fn_00dfc230(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfc2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpTyremarksInfo` (the record at `0126b4c8`, name pointer `010e0118`, that the static initializer `00fc1e10` fills). When `object` is not null, stores the word `010e00dc` (the record's word `+0xc`, which the record's builder function returns) at the start of the object; the second stack word is never read. A null object does nothing.
pub fn fn_00dfc2a0(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_00dc);
    }
}

// Translated from 00dfc250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpTyremarksInfo` (the record at `0126b4c8`, name pointer `010e0118`, that the static initializer `00fc1e10` fills). Calls the object's virtual function in vtable slot `0` (`this` = the object) with the one stack word `0`. There is no null test: the object's vtable is read at once. The slot has the shape of a scalar deleting destructor called with flag 0 (unconfirmed).
pub fn fn_00dfc250(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfc290, return_constant_00dfc290() -> u32),
        entry!(0x00dfc2b0, return_constant_00dfc2b0() -> u32),
        entry!(0x00df_c270, fn_00dfc270(Ptr, u32)),
        entry!(0x00df_c210, fn_00dfc210(Ptr)),
        entry!(0x00df_c280, fn_00dfc280(Ptr, u32)),
        entry!(0x00df_c230, fn_00dfc230(Ptr)),
        entry!(0x00df_c2a0, fn_00dfc2a0(Ptr, u32)),
        entry!(0x00df_c250, fn_00dfc250(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfc290_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c290, &args![]).u32(), 0x010e_00cc);
    }

    #[test]
    fn return_constant_00dfc2b0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c2b0, &args![]).u32(), 0x010e_00dc);
    }
}

#[cfg(test)]
mod tests_a03_type_info {
    use crate::prelude::*;

    /// Calls `function` with an object holding a pattern (and, for two words, a finishing word):
    /// nothing else is called and the object is unchanged.
    fn check_nothing(function: u32, words: usize) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        for i in 0..16 {
            e.mem.set_u32(object + 4 * i, 0x1000_0000 + i);
        }
        e.call_log = Some(vec![]);
        if words == 2 {
            e.call(function, &args![object, 1u32]);
        } else {
            e.call(function, &args![object]);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        for i in 0..16 {
            assert_eq!(e.mem.u32(object + 4 * i), 0x1000_0000 + i);
        }
    }

    macro_rules! nothing_test {
        ($name:ident, $function:expr, $words:expr) => {
            #[test]
            fn $name() {
                check_nothing($function, $words);
            }
        };
    }

    const SLOT_ZERO_TARGET: u32 = 0x0900_0021;

    /// The object's vtable slot 0 is a double: it must run once with `this` = the object and the
    /// word 0.
    fn check_virtual_call(function: u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable, SLOT_ZERO_TARGET);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.register(SLOT_ZERO_TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(function, &args![object]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == SLOT_ZERO_TARGET).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![object, 0]);
    }

    macro_rules! virtual_call_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_virtual_call($function);
            }
        };
    }

    /// A non-null object gets `word` at its start; a null object is left alone (nothing is
    /// called either way).
    fn check_store(function: u32, word: u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 4, 0xaaaa_5555);
        e.call_log = Some(vec![]);
        e.call(function, &args![0u32, 7u32]);
        e.call(function, &args![object, 7u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(e.mem.u32(object), word);
        assert_eq!(e.mem.u32(object + 4), 0xaaaa_5555);
    }

    macro_rules! store_word_test {
        ($name:ident, $function:expr, $word:expr) => {
            #[test]
            fn $name() {
                check_store($function, $word);
            }
        };
    }

    nothing_test!(fn_00dfc270_does_nothing, 0x00df_c270, 2);
    nothing_test!(fn_00dfc210_does_nothing, 0x00df_c210, 1);
    store_word_test!(fn_00dfc280_stores_its_word, 0x00df_c280, 0x010e_00cc);
    virtual_call_test!(fn_00dfc230_calls_vtable_slot_0_with_flag_0, 0x00df_c230);
    store_word_test!(fn_00dfc2a0_stores_its_word, 0x00df_c2a0, 0x010e_00dc);
    virtual_call_test!(fn_00dfc250_calls_vtable_slot_0_with_flag_0, 0x00df_c250);
}
