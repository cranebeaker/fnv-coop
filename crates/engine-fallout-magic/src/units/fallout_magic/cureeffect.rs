//! `fallout/magic/cureeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0080c070` (thiscall on a new 0x50-byte block: caster, spell, effect item, the +0x48
/// word; unnamed): the `CureEffect` constructor for an effect whose +0x4c is -1.
const CONSTRUCTOR_0080C070: u32 = 0x0080_c070;
/// `0080c0b0` (thiscall on a new 0x50-byte block: caster, spell, effect item, the +0x4c
/// word; unnamed): the other `CureEffect` constructor.
const CONSTRUCTOR_0080C0B0: u32 = 0x0080_c0b0;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `00824630` (thiscall on the target: one word; unnamed, `magictarget.cpp`).
const MAGIC_TARGET_CURE_00824630: u32 = 0x0082_4630;
/// `MagicTarget::Dispel_ov3` (`00824800`, thiscall: two words).
const MAGIC_TARGET_DISPEL_OV3: u32 = 0x0082_4800;

// Translated from 0080c0f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080c0f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `CureEffect::Copy` (Xbox PDB): a new 0x50-byte effect made with the caster +0x28, spell
/// +8 and effect item +0xc and, when the word at +0x4c is -1, the word at +0x48
/// (`0080c070`), otherwise the word at +0x4c (`0080c0b0`); null when the allocation fails.
/// It gets this effect's data through `CopyData` (slot +0x44) and is returned.
pub fn cure_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let by_kind = e.mem.u32(base + 0x4c) == 0xffff_ffff;
    let block = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let (constructor, word) = if by_kind {
            (CONSTRUCTOR_0080C070, e.mem.u32(base + 0x48))
        } else {
            (CONSTRUCTOR_0080C0B0, e.mem.u32(base + 0x4c))
        };
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        e.call(constructor, &args![block, caster, spell, item, word])
            .u32()
    };
    e.vcall(base, 0x44, &args![copy]);
    copy
}

// Translated from 0080c210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CureEffect::Start` (Xbox PDB): on the effect's target, `00824630` with the word at +0x48
/// when the word at +0x4c is -1, otherwise `MagicTarget::Dispel_ov3` with the word at +0x4c
/// and 0.
pub fn cure_effect_start(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x4c) == 0xffff_ffff {
        let word = e.mem.u32(base + 0x48);
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        e.call(MAGIC_TARGET_CURE_00824630, &args![target, word]);
    } else {
        let word = e.mem.u32(base + 0x4c);
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        e.call(MAGIC_TARGET_DISPEL_OV3, &args![target, word, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_c0f0, cure_effect_copy(Ptr) -> u32),
        entry!(0x0080_c210, cure_effect_start(Ptr)),
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

    fn effect(e: &mut Engine, word_4c: u32) -> u32 {
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + 0x44, vtable + 0x44);
        answer(e, vtable + 0x44, 0);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        e.mem.set_u32(this + 0x48, 0x48);
        e.mem.set_u32(this + 0x4c, word_4c);
        this
    }

    #[test]
    fn copy_picks_the_constructor_by_the_word_at_4c() {
        for (word, constructor, last) in [
            (0xffff_ffffu32, CONSTRUCTOR_0080C070, 0x48u32),
            (7, CONSTRUCTOR_0080C0B0, 7),
        ] {
            let mut e = Engine::new();
            let this = effect(&mut e, word);
            answer(&mut e, OPERATOR_NEW, 0x9000);
            answer(&mut e, CONSTRUCTOR_0080C070, 0x9004);
            answer(&mut e, CONSTRUCTOR_0080C0B0, 0x9004);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0080_c0f0, &args![this]).u32(), 0x9004);
            assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x50]]);
            assert_eq!(
                calls(&e, constructor),
                vec![vec![0x9000, 0x28, 0x08, 0x0c, last]]
            );
            let slot = e.mem.u32(this) + 0x44;
            assert_eq!(calls(&e, slot), vec![vec![this, 0x9004]]);
            answer(&mut e, OPERATOR_NEW, 0);
            assert_eq!(e.call(0x0080_c0f0, &args![this]).u32(), 0);
            assert_eq!(calls(&e, slot)[1], vec![this, 0]);
        }
    }

    #[test]
    fn start_cures_by_kind_or_dispels_by_word() {
        let mut e = Engine::new();
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_CURE_00824630, 0);
        answer(&mut e, MAGIC_TARGET_DISPEL_OV3, 0);
        e.call_log = Some(vec![]);
        let by_kind = effect(&mut e, 0xffff_ffff);
        e.call(0x0080_c210, &args![by_kind]);
        assert_eq!(
            calls(&e, MAGIC_TARGET_CURE_00824630),
            vec![vec![0x7000, 0x48]]
        );
        let by_word = effect(&mut e, 9);
        e.call(0x0080_c210, &args![by_word]);
        assert_eq!(calls(&e, MAGIC_TARGET_DISPEL_OV3), vec![vec![0x7000, 9, 0]]);
    }
}
