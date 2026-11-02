//! `fallout/magic/nonactormagictarget.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `MagicTarget::AddTarget` (engine map, `008230f0`, `thiscall`, four
/// words, returns a byte) and `MagicTarget::UpdateTarget` (`00823c40`,
/// `thiscall`, one `float`).
const MAGIC_TARGET_ADD_TARGET: u32 = 0x0082_30f0;
const MAGIC_TARGET_UPDATE_TARGET: u32 = 0x0082_3c40;

// Translated from 00826060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NonActorMagicTarget::AddTarget` (engine map; `thiscall`, `RET 0x10`:
/// three words and a byte): `MagicTarget::AddTarget` with the four (the
/// byte zero-extended); when it answers true, `MagicTarget::UpdateTarget`
/// with 1.0. Returns the answer.
pub fn non_actor_magic_target_add_target(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    flag: u8,
) -> u8 {
    let added = e
        .call(
            MAGIC_TARGET_ADD_TARGET,
            &args![this, first, second, third, u32::from(flag)],
        )
        .u8();
    if added != 0 {
        e.call(MAGIC_TARGET_UPDATE_TARGET, &args![this, 1.0f32.to_bits()]);
    }
    added
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0082_6060,
        non_actor_magic_target_add_target(Ptr, u32, u32, u32, u8) -> u8
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_target_updates_when_added() {
        let mut e = Engine::new();
        e.register(MAGIC_TARGET_ADD_TARGET, |_, a| Ret {
            eax: 0x1200 | (a[4] & 1),
            ..Ret::default()
        });
        e.register(MAGIC_TARGET_UPDATE_TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let r = e.call(0x0082_6060, &args![0x40u32, 1u32, 2u32, 3u32, 0x301u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(r.u32(), 1);
        let added: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == MAGIC_TARGET_ADD_TARGET)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(added, vec![vec![0x40, 1, 2, 3, 1]]);
        let updated: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == MAGIC_TARGET_UPDATE_TARGET)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(updated, vec![vec![0x40, 1.0f32.to_bits()]]);
        e.call_log = Some(vec![]);
        let r = e.call(0x0082_6060, &args![0x40u32, 1u32, 2u32, 3u32, 0x300u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(r.u32(), 0);
        assert!(!log.iter().any(|(a, _)| *a == MAGIC_TARGET_UPDATE_TARGET));
    }
}
