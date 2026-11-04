//! `fallout/magic/detectlifeeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0080cd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01079e7c` (RTTI `.?AVDetectLifeEffect@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `0080d930`. One word, plain `RET`, returns nothing.
pub fn fn_0080cd90(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_9e7c);
    e.call(0x0080_d930, &args![this]);
}

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, cdecl: the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `0080cd30` (thiscall on a new 0x4c-byte block: caster, spell, effect item; unnamed):
/// the `DetectLifeEffect` constructor `Copy` uses.
const CONSTRUCTOR_0080CD30: u32 = 0x0080_cd30;
/// `ValueModifierEffect::Start` (`0082c000`).
const VALUE_MODIFIER_EFFECT_START: u32 = 0x0082_c000;
/// `ValueModifierEffect::Finish` (`0082c810`).
const VALUE_MODIFIER_EFFECT_FINISH: u32 = 0x0082_c810;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// The player pointer (`011dea3c`).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `0080cec0` (no arguments; unnamed): clears the byte at `011dc034`.
const CLEAR_0080CEC0: u32 = 0x0080_cec0;

// Translated from 0080cd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectLifeEffect::__scalar_deleting_destructor` (Xbox PDB): the destructor
/// (`fn_0080cd90`), then `operator delete` when bit 0 of `flags` is set; returns `this`.
pub fn detect_life_effect_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    fn_0080cd90(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this.addr()
}

// Translated from 0080cdb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080cdb0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `DetectLifeEffect::Copy` (Xbox PDB): a new 0x4c-byte effect (`operator new`, then
/// `0080cd30` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails), given this effect's data through `CopyData` (slot +0x44); returns it.
pub fn detect_life_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        e.call(CONSTRUCTOR_0080CD30, &args![block, caster, spell, item])
            .u32()
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

/// `0080cec0` when the effect's target's reference (slot +4) is the player.
fn clear_for_the_player(e: &mut Engine, this: Ptr) {
    let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
    if e.vcall(target, 4, &[]).u32() == e.global::<u32>(PLAYER_POINTER) {
        e.call(CLEAR_0080CEC0, &[]);
    }
}

// Translated from 0080ce60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectLifeEffect::Start` (Xbox PDB): `ValueModifierEffect::Start`, then `0080cec0`
/// when the target's reference is the player.
pub fn detect_life_effect_start(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_EFFECT_START, &args![this]);
    clear_for_the_player(e, this);
}

// Translated from 0080d0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DetectLifeEffect::Finish` (Xbox PDB): `ValueModifierEffect::Finish`, then `0080cec0`
/// when the target's reference is the player.
pub fn detect_life_effect_finish(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_EFFECT_FINISH, &args![this]);
    clear_for_the_player(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_cd90, fn_0080cd90(Ptr)),
        entry!(
            0x0080_cd60,
            detect_life_effect_scalar_deleting_destructor(Ptr, u32) -> u32
        ),
        entry!(0x0080_cdb0, detect_life_effect_copy(Ptr) -> u32),
        entry!(0x0080_ce60, detect_life_effect_start(Ptr)),
        entry!(0x0080_d0e0, detect_life_effect_finish(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_0080cd90_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0080_d930, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0080_cd90, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0080_d930)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0107_9e7c);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0107_9e7c,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod effect_tests {
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

    /// An object whose vtable slot `offset` answers `value` (its double sits at the slot).
    fn object(e: &mut Engine, offset: u32, value: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + offset, vtable + offset);
        answer(e, vtable + offset, value);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_with_bit_0() {
        for (flags, deletes) in [(1u32, 1usize), (0, 0)] {
            let mut e = Engine::new();
            let this = e.mem.alloc(0x4c);
            answer(&mut e, 0x0080_d930, 0);
            answer(&mut e, OPERATOR_DELETE, 0);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0080_cd60, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), 0x0107_9e7c);
            assert_eq!(calls(&e, OPERATOR_DELETE).len(), deletes);
        }
    }

    #[test]
    fn copy_allocates_constructs_and_copies_data() {
        let mut e = Engine::new();
        let this = object(&mut e, 0x44, 0);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        answer(&mut e, OPERATOR_NEW, 0x9000);
        answer(&mut e, CONSTRUCTOR_0080CD30, 0x9004);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_cdb0, &args![this]).u32(), 0x9004);
        assert_eq!(
            calls(&e, CONSTRUCTOR_0080CD30),
            vec![vec![0x9000, 0x28, 0x08, 0x0c]]
        );
        let slot = e.mem.u32(this) + 0x44;
        assert_eq!(calls(&e, slot), vec![vec![this, 0x9004]]);
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_cdb0, &args![this]).u32(), 0);
    }

    #[test]
    fn start_and_finish_clear_for_the_player_only() {
        for (reference, clears) in [(0x0700_0000u32, 2usize), (0x0700_0004, 0)] {
            let mut e = Engine::new();
            e.map(0x011d_e000, 0x1000);
            e.set_global(PLAYER_POINTER, 0x0700_0000u32);
            let target = object(&mut e, 4, reference);
            answer(&mut e, EFFECT_TARGET_0059BB30, target);
            for addr in [
                VALUE_MODIFIER_EFFECT_START,
                VALUE_MODIFIER_EFFECT_FINISH,
                CLEAR_0080CEC0,
            ] {
                answer(&mut e, addr, 0);
            }
            e.call_log = Some(vec![]);
            e.call(0x0080_ce60, &args![0x1000u32]);
            e.call(0x0080_d0e0, &args![0x1000u32]);
            assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_START), vec![vec![0x1000]]);
            assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_FINISH), vec![vec![0x1000]]);
            assert_eq!(calls(&e, CLEAR_0080CEC0).len(), clears);
        }
    }
}
