//! `Havok SDK/run_00cfb050` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cfb050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words `_unused_0`, `entry`, `data`; the roles are
/// descriptions, not confirmed): `00cfb080` (it sets up a collision dispatcher's tables) stores its address in the
/// first word of the agent record at `+0x16b0` (the create function the agent machine calls with
/// `(input, entry, data)`). Sets the byte at `entry` to 0 and answers `data`.
pub fn fn_00cfb050(e: &mut Engine, _unused_0: u32, entry: Ptr, data: u32) -> u32 {
    e.mem.set_u8(entry.addr(), 0);
    data
}

// Translated from 00cfb060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, five words, none of them read): `00cfb080` (it sets up a
/// collision dispatcher's tables) stores its address in the word at `+0x16b4` of the agent
/// record, the destroy
/// function the agent machine calls with `(entry, agent data, partner data, context,
/// dispatcher)`. It does nothing.
pub fn fn_00cfb060(
    _e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
}

// Translated from 00cfb070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, five words, only the third read; the roles are
/// descriptions, not confirmed): `00cfb080` (it sets up a collision dispatcher's tables) stores its address in the
/// word at `+0x16dc` of the agent record, the process function the agent machine calls with
/// `(input, entry, data, 0, state)`. Answers its third word, `data`.
pub fn fn_00cfb070(
    _e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    data: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> u32 {
    data
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cfb050, fn_00cfb050(u32, Ptr, u32) -> u32),
        entry!(0x00cfb060, fn_00cfb060(u32, u32, u32, u32, u32)),
        entry!(0x00cfb070, fn_00cfb070(u32, u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00cfb050_clears_the_entry_byte_and_answers_the_data() {
        let mut e = Engine::new();
        let entry = e.mem.alloc(8);
        e.mem.set_u8(entry, 0x7f);
        let answer = e.call(0x00cf_b050, &args![1u32, entry, 0x1234u32]).u32();
        assert_eq!(answer, 0x1234);
        assert_eq!(e.mem.u8(entry), 0);
    }

    #[test]
    fn fn_00cfb060_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00cf_b060, &args![1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00cfb070_answers_its_third_word() {
        let mut e = Engine::new();
        let answer = e
            .call(0x00cf_b070, &args![1u32, 2u32, 0x3333u32, 4u32, 5u32])
            .u32();
        assert_eq!(answer, 0x3333);
    }
}
