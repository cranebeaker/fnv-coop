//! `fallout/magic/run_008117d0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MagicBoltProjectile` members at their PC offsets (the Xbox PDB's minus 0x10, the
//! `TESForm` difference): `spBoltProperty` +0xA8, `eState` +0xAC, `pAreaBolts` +0xB0,
//! `spBoltNode` +0xB4, `spSource` +0xB8, `spTarget` +0xBC, `spSoundNode` +0xC0, `pTarget`
//! +0xC4, `Sound` +0xC8, `fCollideTime` +0xD4; `fSpeed` (`MagicProjectile`) +0x88.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MagicProjectile` base constructor `0081dbb0` (unnamed in the engine map;
/// `thiscall`, no stack words).
const MAGIC_PROJECTILE_CONSTRUCTOR: u32 = 0x0081_dbb0;
/// `MagicBoltProjectile`'s two vtables (`0107a8f4` at +0, `0107a8e8` at +0x18; RTTI
/// `.?AVMagicBoltProjectile@@`).
const MAGIC_BOLT_PROJECTILE_VTABLE: u32 = 0x0107_a8f4;
const MAGIC_BOLT_PROJECTILE_VTABLE_18: u32 = 0x0107_a8e8;
/// `00633c90` (`thiscall`, one word): constructs a reference-counted pointer from the word
/// (stores it and takes a reference when it is not null).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `0066b0d0` (`thiscall`, one word): assigns the word to a reference-counted pointer.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `0041a250` (`thiscall`, no stack words): constructs the sound handle member.
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
/// `008133d0` (`thiscall`, one word): stores the word at +0xAC (`eState`).
const SET_STATE: u32 = 0x0081_33d0;
/// The float `fSpeed` starts at (`01017a3c`).
const INITIAL_SPEED: u32 = 0x0101_7a3c;
/// `00440d80` (`thiscall`): bit 0x20 of the form flags (+8).
const FORM_FLAG_20: u32 = 0x0044_0d80;
/// The destructor body `00811e10` (`MagicBoltProjectile::~MagicBoltProjectile` in the
/// engine map).
const MAGIC_BOLT_PROJECTILE_DESTRUCTOR: u32 = 0x0081_1e10;
/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

/// The five reference-counted pointers the constructor builds, in its order.
const POINTER_MEMBERS: [u32; 5] = [0xa8, 0xb4, 0xb8, 0xbc, 0xc0];

// Translated from 008117d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008117d0 - the FS:[0] exception frame (destroying the members built so far when a constructor throws) is not translated
/// The `MagicBoltProjectile` constructor (by the vtables it stores; unnamed in the engine
/// map; `thiscall`, plain `RET`): the `MagicProjectile` constructor `0081dbb0`, the two
/// vtables, null pointers at +0xA8, +0xB4, +0xB8, +0xBC and +0xC0 (`00633c90`), the sound
/// handle at +0xC8 (`0041a250`), `eState` 0 (`008133d0`), `fSpeed` (+0x88) from the float
/// at `01017a3c` (`FLD`/`FSTP`: a signalling NaN is stored quiet), +0xB0 and +0xC4
/// cleared, the pointers at +0xA8, +0xB4, +0xB8 and +0xBC assigned null again
/// (`0066b0d0`) and `fCollideTime` (+0xD4) 0.0. Returns `this`.
pub fn fn_008117d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MAGIC_PROJECTILE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, MAGIC_BOLT_PROJECTILE_VTABLE);
    e.mem.set_u32(base + 0x18, MAGIC_BOLT_PROJECTILE_VTABLE_18);
    for offset in POINTER_MEMBERS {
        e.call(POINTER_CONSTRUCT, &args![base + offset, 0u32]);
    }
    e.call(SOUND_HANDLE_CONSTRUCTOR, &args![base + 0xc8]);
    e.call(SET_STATE, &args![this, 0u32]);
    let speed = x87_quiet_bits(e.global::<u32>(INITIAL_SPEED));
    e.mem.set_u32(base + 0x88, speed);
    e.mem.set_u32(base + 0xb0, 0);
    e.mem.set_u32(base + 0xc4, 0);
    for offset in [0xa8, 0xb4, 0xb8, 0xbc] {
        e.call(POINTER_ASSIGN, &args![base + offset, 0u32]);
    }
    e.mem.set_f32(base + 0xd4, 0.0);
    this
}

// Translated from 00811930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): true (`EAX` = 1) when bit 0x20 of the form
/// flags (`00440d80`) is clear, false (0) when it is set (`NEG`/`SBB`/`ADD`).
pub fn fn_00811930(e: &mut Engine, this: Ptr) -> bool {
    !e.call(FORM_FLAG_20, &args![this]).bool()
}

// Translated from 00811950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MagicBoltProjectile` scalar deleting destructor (by the destructor it calls;
/// `thiscall`, `RET 4`): the destructor body `00811e10`, then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn fn_00811950(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAGIC_BOLT_PROJECTILE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_17d0, fn_008117d0(Ptr) -> Ptr),
        entry!(0x0081_1930, fn_00811930(Ptr) -> bool),
        entry!(0x0081_1950, fn_00811950(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_008117d0_constructs_the_bolt_projectile() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global::<u32>(INITIAL_SPEED, 0x7f80_0001);
        for callee in [
            MAGIC_PROJECTILE_CONSTRUCTOR,
            POINTER_CONSTRUCT,
            POINTER_ASSIGN,
            SOUND_HANDLE_CONSTRUCTOR,
            SET_STATE,
        ] {
            e.register(callee, |_, a| a[0].into_ret());
        }
        let this = e.mem.alloc(0xe0);
        for i in 0..0x38 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_17d0, &[this]).u32(), this);
        assert_eq!(calls(&e, MAGIC_PROJECTILE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(
            calls(&e, POINTER_CONSTRUCT),
            POINTER_MEMBERS
                .iter()
                .map(|offset| vec![this + offset, 0])
                .collect::<Vec<_>>()
        );
        assert_eq!(
            calls(&e, POINTER_ASSIGN),
            [0xa8, 0xb4, 0xb8, 0xbc]
                .iter()
                .map(|offset| vec![this + offset, 0])
                .collect::<Vec<_>>()
        );
        assert_eq!(calls(&e, SOUND_HANDLE_CONSTRUCTOR), vec![vec![this + 0xc8]]);
        assert_eq!(calls(&e, SET_STATE), vec![vec![this, 0]]);
        assert_eq!(e.mem.u32(this), MAGIC_BOLT_PROJECTILE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), MAGIC_BOLT_PROJECTILE_VTABLE_18);
        assert_eq!(e.mem.u32(this + 0x88), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert_eq!(e.mem.u32(this + 0xc4), 0);
        assert_eq!(e.mem.u32(this + 0xd4), 0);
    }

    #[test]
    fn fn_00811930_inverts_the_flag() {
        let mut e = Engine::new();
        e.register(FORM_FLAG_20, |_, _| 0x100u32.into_ret());
        assert_eq!(e.call(0x0081_1930, &[0x10]).u32(), 1);
        e.register(FORM_FLAG_20, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x0081_1930, &[0x10]).u32(), 0);
    }

    #[test]
    fn fn_00811950_deletes_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(MAGIC_BOLT_PROJECTILE_DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_1950, &[0x500, 0]).u32(), 0x500);
        assert_eq!(e.call(0x0081_1950, &[0x500, 1]).u32(), 0x500);
        assert_eq!(calls(&e, MAGIC_BOLT_PROJECTILE_DESTRUCTOR).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x500]]);
    }
}
