//! `fallout/magic/commandhumanoideffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0080ba90` (thiscall on a new 0x48-byte block: caster, spell, effect item; unnamed):
/// the `CommandHumanoidEffect` constructor `Copy` uses.
const CONSTRUCTOR_0080BA90: u32 = 0x0080_ba90;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfDelta, source type, target type, isReference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `MagicTarget`'s RTTI type descriptor (`.?AVMagicTarget@@`).
const MAGIC_TARGET_TYPE: u32 = 0x011a_0ef8;
/// `Character`'s RTTI type descriptor (`.?AVCharacter@@`).
const CHARACTER_TYPE: u32 = 0x011a_0f14;
/// `00495460` (thiscall on the effect, returns `ST0`; unnamed): its magnitude.
const EFFECT_MAGNITUDE_00495460: u32 = 0x0049_5460;
/// `0087f9f0` (thiscall on the actor, returns `AX`; unnamed, `actor.cpp`): its level.
const ACTOR_LEVEL_0087F9F0: u32 = 0x0087_f9f0;
/// `MagicFormulas::CheckTargetLevelMagnitude` (`006494d0`, cdecl: level, magnitude).
const CHECK_TARGET_LEVEL_MAGNITUDE: u32 = 0x0064_94d0;

// Translated from 0080bac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080bac0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `CommandHumanoidEffect::Copy` (Xbox PDB): a new 0x48-byte effect (`operator new`, then
/// `0080ba90` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails), given this effect's data through `CopyData` (slot +0x44); returns it.
pub fn command_humanoid_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        e.call(CONSTRUCTOR_0080BA90, &args![block, caster, spell, item])
            .u32()
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0080bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CommandHumanoidEffect::CheckTarget` (Xbox PDB): for a target that is a `Character`
/// (`__RTDynamicCast` from `MagicTarget`) whose slot +0x360 is false (otherwise false), the
/// answer of `CheckTargetLevelMagnitude` for its level (`0087f9f0`) and the effect's
/// magnitude (`00495460`, stored as a `float`).
pub fn command_humanoid_effect_check_target(e: &mut Engine, this: Ptr, target: u32) -> bool {
    let character = e
        .call(
            RT_DYNAMIC_CAST,
            &args![target, 0u32, MAGIC_TARGET_TYPE, CHARACTER_TYPE, 0u32],
        )
        .u32();
    if character == 0 || e.vcall(character, 0x360, &[]).bool() {
        return false;
    }
    let magnitude = x87_store32(e.call(EFFECT_MAGNITUDE_00495460, &args![this]).f64());
    let level = u32::from(e.call(ACTOR_LEVEL_0087F9F0, &args![character]).u16());
    e.call(CHECK_TARGET_LEVEL_MAGNITUDE, &args![level, magnitude])
        .bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_bac0, command_humanoid_effect_copy(Ptr) -> u32),
        entry!(
            0x0080_bb70,
            command_humanoid_effect_check_target(Ptr, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// An object whose vtable slots answer the given words; each slot's double sits at the
    /// slot's own address.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x600);
        for (offset, value) in slots {
            e.mem.set_u32(vtable + offset, vtable + offset);
            answer(e, vtable + offset, *value);
        }
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn copy_allocates_constructs_and_copies_data() {
        let mut e = Engine::new();
        let this = object(&mut e, &[(0x44, 0)]);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        answer(&mut e, OPERATOR_NEW, 0x9000);
        answer(&mut e, CONSTRUCTOR_0080BA90, 0x9004);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_bac0, &args![this]).u32(), 0x9004);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x48]]);
        assert_eq!(
            calls(&e, CONSTRUCTOR_0080BA90),
            vec![vec![0x9000, 0x28, 0x08, 0x0c]]
        );
        let slot = e.mem.u32(this) + 0x44;
        assert_eq!(calls(&e, slot), vec![vec![this, 0x9004]]);
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_bac0, &args![this]).u32(), 0);
    }

    #[test]
    fn check_target_needs_a_free_character() {
        for (cast, busy, accepted, expected) in [
            (true, 0u32, 1u32, true),
            (true, 0, 0, false),
            (true, 1, 1, false),
            (false, 0, 1, false),
        ] {
            let mut e = Engine::new();
            let character = object(&mut e, &[(0x360, busy)]);
            answer(&mut e, RT_DYNAMIC_CAST, if cast { character } else { 0 });
            e.register(EFFECT_MAGNITUDE_00495460, |_, _| Ret {
                st0: 3.0,
                ..Ret::default()
            });
            answer(&mut e, ACTOR_LEVEL_0087F9F0, 4);
            answer(&mut e, CHECK_TARGET_LEVEL_MAGNITUDE, accepted);
            e.call_log = Some(vec![]);
            let result = e.call(0x0080_bb70, &args![0x1000u32, 0x2000u32]).bool();
            assert_eq!(result, expected);
            assert_eq!(
                calls(&e, RT_DYNAMIC_CAST),
                vec![vec![0x2000, 0, MAGIC_TARGET_TYPE, CHARACTER_TYPE, 0]]
            );
            if cast && busy == 0 {
                assert_eq!(
                    calls(&e, CHECK_TARGET_LEVEL_MAGNITUDE),
                    vec![vec![4, 3.0f32.to_bits()]]
                );
            }
        }
    }
}
