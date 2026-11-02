//! `Havok SDK/hkpcharacterstate.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cd3b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterState::enterState` (Xbox PDB): the base class does nothing. `this` and four
/// stack words (`RET 0x10`), none read.
pub fn hkp_character_state_enter_state(
    _e: &mut Engine,
    _this: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
}

// Translated from 00cd3b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterState::leaveState` (Xbox PDB): the base class does nothing. `this` and four
/// stack words (`RET 0x10`), none read.
pub fn hkp_character_state_leave_state(
    _e: &mut Engine,
    _this: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cd3b00,
            hkp_character_state_enter_state(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00cd3b10,
            hkp_character_state_leave_state(Ptr, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_and_leave_state_do_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for address in [0x00cd3b00, 0x00cd3b10] {
            let ret = e.call(address, &args![1u32, 2u32, 3u32, 4u32, 5u32]);
            assert_eq!(ret, Ret::default());
        }
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }
}
