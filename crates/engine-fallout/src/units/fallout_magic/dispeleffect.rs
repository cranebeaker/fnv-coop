//! `fallout/magic/dispeleffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0080d900` (thiscall on a new 0x48-byte block: caster, spell, effect item; unnamed):
/// the `DispelEffect` constructor `Copy` uses.
const CONSTRUCTOR_0080D900: u32 = 0x0080_d900;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `0084e3a0` (thiscall): the word at +0xc.
const WORD_AT_C_0084E3A0: u32 = 0x0084_e3a0;
/// `0045cd60` (thiscall): the effect's caster (+0x28).
const EFFECT_CASTER_0045CD60: u32 = 0x0045_cd60;
/// `MagicCaster::GetCasterAsActor` (`00815410`).
const MAGIC_CASTER_GET_CASTER_AS_ACTOR: u32 = 0x0081_5410;
/// `008256d0` (thiscall on a list node, `AL`): true when the node's item and next are null.
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (thiscall on a list node, returns the address of its item).
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (thiscall on a list node, returns the next node).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `0044ddc0` (thiscall): the word at +8 (an effect's spell).
const WORD_AT_8_0044DDC0: u32 = 0x0044_ddc0;
/// `00495460` (thiscall on the effect, returns `ST0`; unnamed): its magnitude.
const EFFECT_MAGNITUDE_00495460: u32 = 0x0049_5460;
/// `00403e20` (thiscall on a setting): the address of its `float` value.
const SETTING_VALUE_00403E20: u32 = 0x0040_3e20;
/// The setting whose value scales the magnitude (`011d1b20`).
const SETTING_011D1B20: u32 = 0x011d_1b20;
/// `ActiveEffect::Dispel` (`00804210`, thiscall: one byte).
const ACTIVE_EFFECT_DISPEL: u32 = 0x0080_4210;

// Translated from 0080d950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080d950 - the exception-unwinding frame (FS:[0]) is not translated.
/// `DispelEffect::Copy` (Xbox PDB): a new 0x48-byte effect (`operator new`, then
/// `0080d900` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails), given this effect's data through `CopyData` (slot +0x44); returns it.
pub fn dispel_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        e.call(CONSTRUCTOR_0080D900, &args![block, caster, spell, item])
            .u32()
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0080da00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DispelEffect::Start` (Xbox PDB): walks the target's effects (slot 8) until an empty
/// node; each other effect whose spell is of type 0 (slot +0x18) is dispelled (`Dispel`
/// with 0) when the magnitude times the setting at `011d1b20` (`FMUL`) is not below the
/// spell's cost (slot 8 of its list at +0xc, with the caster actor when this effect's
/// effect item has a null +0xc word, otherwise null; `FCOMP`, NaN keeps it).
pub fn dispel_effect_start(e: &mut Engine, this: Ptr) {
    let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
    let mut node = e.vcall(target, 8, &[]).u32();
    let item = e.call(WORD_AT_C_0084E3A0, &args![this]).u32();
    let with_caster = e.call(WORD_AT_C_0084E3A0, &args![item]).u32() == 0;
    let caster = if e.call(EFFECT_CASTER_0045CD60, &args![this]).u32() == 0 {
        0
    } else {
        let caster = e.call(EFFECT_CASTER_0045CD60, &args![this]).u32();
        e.call(MAGIC_CASTER_GET_CASTER_AS_ACTOR, &args![caster])
            .u32()
    };
    while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let effect = e.mem.u32(slot);
        let next = e.call(LIST_NODE_NEXT, &args![node]).u32();
        if effect != 0 && effect != this.addr() {
            let spell = e.call(WORD_AT_8_0044DDC0, &args![effect]).u32();
            if e.vcall(spell, 0x18, &[]).u32() == 0 {
                let caster_word = if with_caster { caster } else { 0 };
                let cost = e.vcall(spell + 0xc, 8, &args![caster_word]).f64();
                let magnitude = e.call(EFFECT_MAGNITUDE_00495460, &args![this]).f64();
                let value = e
                    .call(SETTING_VALUE_00403E20, &args![SETTING_011D1B20])
                    .u32();
                let scaled = x87_mul(x87_load32(e.mem.u32(value)), magnitude);
                // FCOMP; TEST AH,1; JNZ: kept when below or unordered.
                if !(scaled < cost || scaled.is_nan() || cost.is_nan()) {
                    e.call(ACTIVE_EFFECT_DISPEL, &args![effect, 0u32]);
                }
            }
        }
        node = next;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_d950, dispel_effect_copy(Ptr) -> u32),
        entry!(0x0080_da00, dispel_effect_start(Ptr)),
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

    #[test]
    fn copy_allocates_constructs_and_copies_data() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + 0x44, vtable + 0x44);
        answer(&mut e, vtable + 0x44, 0);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        answer(&mut e, OPERATOR_NEW, 0x9000);
        answer(&mut e, CONSTRUCTOR_0080D900, 0x9004);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_d950, &args![this]).u32(), 0x9004);
        assert_eq!(
            calls(&e, CONSTRUCTOR_0080D900),
            vec![vec![0x9000, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(calls(&e, vtable + 0x44), vec![vec![this, 0x9004]]);
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_d950, &args![this]).u32(), 0);
    }

    /// An effect whose spell (+8) has type `kind` (slot +0x18) and a list at +0xc whose
    /// slot 8 costs `cost`.
    fn effect(e: &mut Engine, kind: u32, cost: f64) -> u32 {
        let spell_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(spell_vtable + 0x18, spell_vtable + 0x18);
        answer(e, spell_vtable + 0x18, kind);
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(list_vtable + 8, list_vtable + 8);
        e.register_double(list_vtable + 8, move |_, _| Ret {
            st0: cost,
            ..Ret::default()
        });
        let spell = e.mem.alloc(0x20);
        e.mem.set_u32(spell, spell_vtable);
        e.mem.set_u32(spell + 0xc, list_vtable);
        let effect = e.mem.alloc(0x48);
        e.mem.set_u32(effect + 8, spell);
        effect
    }

    #[test]
    fn start_dispels_the_cheaper_effects_of_type_0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, item);
        let cheap = effect(&mut e, 0, 10.0);
        let dear = effect(&mut e, 0, 40.0);
        let other_kind = effect(&mut e, 1, 1.0);
        let unordered = effect(&mut e, 0, f64::NAN);
        // A list of the four, this effect and a null item.
        let mut next = 0;
        for item in [cheap, this, dear, 0, other_kind, unordered].iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        let target_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(target_vtable + 8, target_vtable + 8);
        answer(&mut e, target_vtable + 8, next);
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, target_vtable);
        answer(&mut e, EFFECT_TARGET_0059BB30, target);
        e.register(WORD_AT_C_0084E3A0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(WORD_AT_8_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        answer(&mut e, EFFECT_CASTER_0045CD60, 0x7100);
        answer(&mut e, MAGIC_CASTER_GET_CASTER_AS_ACTOR, 0x5100);
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u8::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_NODE_ITEM, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EFFECT_MAGNITUDE_00495460, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 2.0);
        answer(&mut e, SETTING_VALUE_00403E20, value);
        answer(&mut e, ACTIVE_EFFECT_DISPEL, 0);
        e.call_log = Some(vec![]);
        e.call(0x0080_da00, &args![this]);
        // 2 * 10 = 20: the 10 is dispelled, the 40 and the NaN are not.
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![cheap, 0]]);
        // The effect item's +0xc word is null: the caster actor is passed.
        let cheap_list = e.mem.u32(e.mem.u32(cheap + 8) + 0xc);
        assert_eq!(calls(&e, cheap_list + 8)[0][1], 0x5100);
    }
}
