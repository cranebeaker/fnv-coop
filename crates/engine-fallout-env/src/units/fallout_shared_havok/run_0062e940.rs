//! `fallout shared/havok/run_0062e940` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004b5a80(object)` (cdecl, one argument): calls `004b5aa0` on it and
/// returns the result.
const ENTITY_OF: u32 = 0x004b_5a80;
/// `bhkEntityListener::RemoveEntity` (engine map name of `00c9a520`;
/// `RET 4`, `this` is the listener).
const REMOVE_ENTITY: u32 = 0x00c9_a520;

// Translated from 0062e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `object` through `004b5a80` and removes what that gives from the
/// listener at `this` (`bhkEntityListener::RemoveEntity`). The exe does not
/// name this function.
pub fn fn_0062e940(e: &mut Engine, this: Ptr, object: u32) {
    let entity = e.call(ENTITY_OF, &args![object]).u32();
    e.call(REMOVE_ENTITY, &args![this, entity]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_e940, fn_0062e940(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_converted_object_is_removed_from_the_listener() {
        let mut e = Engine::new();
        e.register(ENTITY_OF, |_, a| (a[0] + 0x10).into_ret());
        e.register(REMOVE_ENTITY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_e940, &args![0x3000u32, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .into_iter()
            .filter(|(a, _)| *a == ENTITY_OF || *a == REMOVE_ENTITY)
            .collect();
        assert_eq!(
            calls,
            vec![
                (ENTITY_OF, vec![0x9000]),
                (REMOVE_ENTITY, vec![0x3000, 0x9010])
            ]
        );
    }
}
