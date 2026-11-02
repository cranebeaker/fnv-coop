//! `Havok SDK/hkserializedeprecated.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The stubs the game links when `hkSerializeDeprecated::initDeprecated()` was not called.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkStringPtr::operator=` (Xbox PDB name), `thiscall`, one string word.
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
/// The message the loading stubs store: "Call hkSerializeDeprecated::initDeprecated() to
/// include loading support for this file".
const INIT_DEPRECATED_MESSAGE: u32 = 0x010d_6280;
/// The load result the stubs store in the result structure's first word.
const LOAD_RESULT_UNSUPPORTED: u32 = 7;

// Translated from 00dba5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeDeprecated::saveXmlPackfile` (Xbox PDB), `RET 0x14` (five words, none read):
/// returns 1.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hk_serialize_deprecated_save_xml_packfile(
    _e: &mut Engine,
    _this: u32,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> u32 {
    1
}

// Translated from 00dba5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeDeprecated::isLoadable` (Xbox PDB), `RET 4` (one word, not read): returns 0.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hk_serialize_deprecated_is_loadable(_e: &mut Engine, _this: u32, _unused_0: u32) -> u32 {
    0
}

/// The body both loading stubs share: with a result structure, stores 7 in its first word and
/// the "call `initDeprecated()`" message in the string pointer at `+4`; returns 0.
fn load_unsupported(e: &mut Engine, result: u32) -> u32 {
    if result != 0 {
        e.mem.set_u32(result, LOAD_RESULT_UNSUPPORTED);
        e.call(
            STRING_PTR_ASSIGN,
            &args![result + 4, INIT_DEPRECATED_MESSAGE],
        );
    }
    0
}

// Translated from 00dba5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeDeprecated::loadOldPackfile` (Xbox PDB), `RET 0xc` (three words, the first two not
/// read): when `result` is not null stores 7 at `result` and the `initDeprecated()` message
/// in the `hkStringPtr` at `result + 4`; returns 0 (no object).
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hk_serialize_deprecated_load_old_packfile(
    e: &mut Engine,
    _this: u32,
    _unused_0: u32,
    _unused_1: u32,
    result: u32,
) -> u32 {
    load_unsupported(e, result)
}

// Translated from 00dba5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeDeprecated::loadOldPackfileOnHeap` (Xbox PDB), `RET 0xc`: the same stub as
/// `00dba5d0` (a separate copy): stores 7 and the `initDeprecated()` message in `result` when
/// it is not null; returns 0.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hk_serialize_deprecated_load_old_packfile_on_heap(
    e: &mut Engine,
    _this: u32,
    _unused_0: u32,
    _unused_1: u32,
    result: u32,
) -> u32 {
    load_unsupported(e, result)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dba5b0, hk_serialize_deprecated_save_xml_packfile(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00dba5c0, hk_serialize_deprecated_is_loadable(u32, u32) -> u32),
        entry!(0x00dba5d0, hk_serialize_deprecated_load_old_packfile(u32, u32, u32, u32) -> u32),
        entry!(0x00dba5f0, hk_serialize_deprecated_load_old_packfile_on_heap(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    #[test]
    fn save_and_is_loadable_return_their_constants() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(
                0x00db_a5b0,
                &args![VCALL_THIS, 1u32, 2u32, 3u32, 4u32, 5u32]
            )
            .u32(),
            1
        );
        assert_eq!(e.call(0x00db_a5c0, &args![VCALL_THIS, 9u32]).u32(), 0);
    }

    fn check_load_stub(address: u32) {
        let mut e = Engine::new();
        e.register(STRING_PTR_ASSIGN, |_, _| Ret::default());
        let result = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(address, &args![VCALL_THIS, 1u32, 2u32, result])
                .u32(),
            0
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(result), 7);
        assert_eq!(
            log[1],
            (STRING_PTR_ASSIGN, vec![result + 4, INIT_DEPRECATED_MESSAGE])
        );
        // Without a result structure nothing is stored and nothing is called.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(address, &args![VCALL_THIS, 1u32, 2u32, 0u32]).u32(),
            0
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn load_old_packfile_reports_the_missing_support() {
        check_load_stub(0x00db_a5d0);
    }

    #[test]
    fn load_old_packfile_on_heap_reports_the_missing_support() {
        check_load_stub(0x00db_a5f0);
    }
}
