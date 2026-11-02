//! `fallout/magic/darknesseffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0082ba60` (thiscall: three words; unnamed): the `ValueModifierEffect` constructor.
const VALUE_MODIFIER_EFFECT_CONSTRUCTOR: u32 = 0x0082_ba60;
/// `DarknessEffect`'s vtable (stored by its constructor).
const DARKNESS_EFFECT_VTABLE: u32 = 0x0107_9d2c;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `ValueModifierEffect::Start` (`0082c000`).
const VALUE_MODIFIER_EFFECT_START: u32 = 0x0082_c000;
/// `ValueModifierEffect::Finish` (`0082c810`).
const VALUE_MODIFIER_EFFECT_FINISH: u32 = 0x0082_c810;
/// `ValueModifierEffect::FinishInitLoadGame` (`0082cd10`, thiscall: one word).
const VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME: u32 = 0x0082_cd10;
/// `004534f0` (thiscall: one word; unnamed, `tes.cpp`): the base `Revert`.
const BASE_REVERT_004534F0: u32 = 0x0045_34f0;
/// `BSShaderManager::RecursiveSetForcedDarkness` (`00b55b00`, cdecl: node, `float`).
const RECURSIVE_SET_FORCED_DARKNESS: u32 = 0x00b5_5b00;
/// `0043fcd0` (thiscall on the actor, returns a node; unnamed, `modelloader.cpp`).
const ACTOR_NODE_0043FCD0: u32 = 0x0043_fcd0;
/// `100.0` (`double`, `01017a40`).
const HUNDRED: u32 = 0x0101_7a40;
/// `0.0` (`double`, `01012060`).
const ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`, `01012070`).
const ONE: u32 = 0x0101_2070;

/// The darkness of the actor: one minus its value (slot +0xc of its object at +0xa4 with
/// the effect's +0x48 word) over 100, stored as a `float`, raised to 0 when below 0 and
/// lowered to 1 when above 1 (`FCOMP`; NaN stays).
fn darkness(e: &mut Engine, this: u32, actor: u32) -> u32 {
    let value_index = e.mem.u32(this + 0x48);
    let value = e.vcall(actor + 0xa4, 0xc, &args![value_index]).f64();
    let hundred: f64 = e.global(HUNDRED);
    let mut darkness = x87_store32(x87_sub(1.0, x87_div(value, hundred)));
    let zero: f64 = e.global(ZERO);
    // FCOMP; TEST AH,5; JP: raised only when below.
    if x87_load32(darkness) < zero {
        darkness = 0;
    }
    let one: f64 = e.global(ONE);
    // FCOMP; TEST AH,0x41; JNZ: lowered only when above.
    if x87_load32(darkness) > one {
        darkness = 1.0f32.to_bits();
    }
    darkness
}

/// The darkness of a target actor's 3D (slot +0x1d0, when set) set from its value
/// (`RecursiveSetForcedDarkness`).
fn darken(e: &mut Engine, this: u32, actor: u32) {
    if actor == 0 || e.vcall(actor, 0x1d0, &[]).u32() == 0 {
        return;
    }
    let darkness = darkness(e, this, actor);
    let node = e.vcall(actor, 0x1d0, &[]).u32();
    e.call(
        RECURSIVE_SET_FORCED_DARKNESS,
        &args![node, x87_quiet_bits(darkness)],
    );
}

/// The effect's target as an actor (`00822b40` of `0059bb30`), null without a target.
fn target_actor(e: &mut Engine, this: u32) -> u32 {
    if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
        0
    } else {
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target])
            .u32()
    }
}

// Translated from 0080c460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`darknesseffect.cpp`; thiscall, three words): the `DarknessEffect` constructor:
/// the `ValueModifierEffect` part (`0082ba60` with the three words), then its vtable;
/// returns `this`.
pub fn fn_0080c460(e: &mut Engine, this: Ptr, caster: u32, spell: u32, item: u32) -> u32 {
    e.call(
        VALUE_MODIFIER_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, item],
    );
    e.mem.set_u32(this.addr(), DARKNESS_EFFECT_VTABLE);
    this.addr()
}

// Translated from 0080c4c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080c4c0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `DarknessEffect::Copy` (Xbox PDB): a new 0x4c-byte effect (`operator new`, then
/// `fn_0080c460` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails), given this effect's data through `CopyData` (slot +0x44); returns it.
pub fn darkness_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        fn_0080c460(e, Ptr::new(block), caster, spell, item)
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0080c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DarknessEffect::Start` (Xbox PDB): `ValueModifierEffect::Start`, then the target
/// actor's 3D gets its darkness.
pub fn darkness_effect_start(e: &mut Engine, this: Ptr) {
    let actor = target_actor(e, this.addr());
    e.call(VALUE_MODIFIER_EFFECT_START, &args![this]);
    darken(e, this.addr(), actor);
}

// Translated from 0080c650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DarknessEffect::Finish` (Xbox PDB): `ValueModifierEffect::Finish`, then the target
/// actor's 3D gets its darkness again.
pub fn darkness_effect_finish(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_EFFECT_FINISH, &args![this]);
    let actor = target_actor(e, this.addr());
    darken(e, this.addr(), actor);
}

// Translated from 0080c730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080c730 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`darknesseffect.cpp`; cdecl, three words): a new 0x4c-byte `DarknessEffect`
/// (`fn_0080c460` with the three words; null when the allocation fails).
pub fn fn_0080c730(e: &mut Engine, caster: u32, spell: u32, item: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    if block == 0 {
        0
    } else {
        fn_0080c460(e, Ptr::new(block), caster, spell, item)
    }
}

// Translated from 0080c7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DarknessEffect::FinishInitLoadGame` (Xbox PDB; the word is the actor):
/// `ValueModifierEffect::FinishInitLoadGame`, then the actor's node (`0043fcd0`), when set,
/// gets its darkness.
pub fn darkness_effect_finish_init_load_game(e: &mut Engine, this: Ptr, actor: u32) {
    e.call(
        VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME,
        &args![this, actor],
    );
    let darkness = darkness(e, this.addr(), actor);
    if e.call(ACTOR_NODE_0043FCD0, &args![actor]).u32() != 0 {
        let node = e.call(ACTOR_NODE_0043FCD0, &args![actor]).u32();
        e.call(
            RECURSIVE_SET_FORCED_DARKNESS,
            &args![node, x87_quiet_bits(darkness)],
        );
    }
}

// Translated from 0080c850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DarknessEffect::Revert` (Xbox PDB; the word is the actor): the base revert
/// (`004534f0`), then the actor's node (`0043fcd0`), when set, gets darkness 1.0.
pub fn darkness_effect_revert(e: &mut Engine, this: Ptr, actor: u32) {
    e.call(BASE_REVERT_004534F0, &args![this, actor]);
    if e.call(ACTOR_NODE_0043FCD0, &args![actor]).u32() != 0 {
        let node = e.call(ACTOR_NODE_0043FCD0, &args![actor]).u32();
        e.call(RECURSIVE_SET_FORCED_DARKNESS, &args![node, 1.0f32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_c460, fn_0080c460(Ptr, u32, u32, u32) -> u32),
        entry!(0x0080_c4c0, darkness_effect_copy(Ptr) -> u32),
        entry!(0x0080_c570, darkness_effect_start(Ptr)),
        entry!(0x0080_c650, darkness_effect_finish(Ptr)),
        entry!(0x0080_c730, fn_0080c730(u32, u32, u32) -> u32),
        entry!(0x0080_c7b0, darkness_effect_finish_init_load_game(Ptr, u32)),
        entry!(0x0080_c850, darkness_effect_revert(Ptr, u32)),
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

    /// An actor with a 3D node (slot +0x1d0) and a value object answering `value`.
    fn rig(value: f64) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x8000);
        e.set_global(HUNDRED, 100.0f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        let actor = object(&mut e, &[(0x1d0, 0x3d00)]);
        let values = object(&mut e, &[]);
        let values_vtable = e.mem.u32(values);
        e.mem.set_u32(actor + 0xa4, values_vtable);
        e.mem.set_u32(values_vtable + 0xc, values_vtable + 0xc);
        e.register_double(values_vtable + 0xc, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
        for addr in [
            VALUE_MODIFIER_EFFECT_START,
            VALUE_MODIFIER_EFFECT_FINISH,
            VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME,
            BASE_REVERT_004534F0,
            RECURSIVE_SET_FORCED_DARKNESS,
        ] {
            answer(&mut e, addr, 0);
        }
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        let this = object(&mut e, &[(0x44, 0)]);
        e.mem.set_u32(this + 0x48, 0x2a);
        e.call_log = Some(vec![]);
        (e, actor, this)
    }

    #[test]
    fn constructor_and_copy_build_a_darkness_effect() {
        let (mut e, _, this) = rig(0.0);
        answer(&mut e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR, 0);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        let block = e.mem.alloc(0x4c);
        answer(&mut e, OPERATOR_NEW, block);
        assert_eq!(e.call(0x0080_c4c0, &args![this]).u32(), block);
        assert_eq!(e.mem.u32(block), DARKNESS_EFFECT_VTABLE);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR),
            vec![vec![block, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(
            e.call(0x0080_c460, &args![block, 1u32, 2u32, 3u32]).u32(),
            block
        );
        assert_eq!(e.call(0x0080_c730, &args![1u32, 2u32, 3u32]).u32(), block);
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_c730, &args![1u32, 2u32, 3u32]).u32(), 0);
        assert_eq!(e.call(0x0080_c4c0, &args![this]).u32(), 0);
    }

    #[test]
    fn start_and_finish_darken_the_actor_within_0_and_1() {
        for (value, expected) in [
            (25.0, 0.75f32.to_bits()),
            (250.0, 0),
            (-50.0, 1.0f32.to_bits()),
        ] {
            let (mut e, actor, this) = rig(value);
            e.call(0x0080_c570, &args![this]);
            e.call(0x0080_c650, &args![this]);
            assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_START), vec![vec![this]]);
            assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_FINISH), vec![vec![this]]);
            assert_eq!(
                calls(&e, RECURSIVE_SET_FORCED_DARKNESS),
                vec![vec![0x3d00, expected], vec![0x3d00, expected]]
            );
            let values_slot = e.mem.u32(actor + 0xa4) + 0xc;
            assert_eq!(calls(&e, values_slot)[0], vec![actor + 0xa4, 0x2a]);
        }
        // Without a 3D node nothing is set.
        let (mut e, actor, this) = rig(25.0);
        let node_slot = e.mem.u32(actor) + 0x1d0;
        answer(&mut e, node_slot, 0);
        e.call(0x0080_c570, &args![this]);
        assert!(calls(&e, RECURSIVE_SET_FORCED_DARKNESS).is_empty());
    }

    #[test]
    fn loads_and_reverts_use_the_actor_node() {
        let (mut e, actor, this) = rig(25.0);
        answer(&mut e, ACTOR_NODE_0043FCD0, 0x3e00);
        e.call(0x0080_c7b0, &args![this, actor]);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME),
            vec![vec![this, actor]]
        );
        e.call(0x0080_c850, &args![this, actor]);
        assert_eq!(calls(&e, BASE_REVERT_004534F0), vec![vec![this, actor]]);
        assert_eq!(
            calls(&e, RECURSIVE_SET_FORCED_DARKNESS),
            vec![
                vec![0x3e00, 0.75f32.to_bits()],
                vec![0x3e00, 1.0f32.to_bits()]
            ]
        );
        answer(&mut e, ACTOR_NODE_0043FCD0, 0);
        e.call(0x0080_c7b0, &args![this, actor]);
        e.call(0x0080_c850, &args![this, actor]);
        assert_eq!(calls(&e, RECURSIVE_SET_FORCED_DARKNESS).len(), 2);
    }
}
