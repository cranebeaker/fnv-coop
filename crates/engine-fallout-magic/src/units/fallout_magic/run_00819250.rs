//! `fallout/magic/run_00819250` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450` (`thiscall`): the word at `this`.
const FIRST_WORD: u32 = 0x0055_9450;
/// `0043b4f0` (`thiscall`, one word: an out word): stores a value found for `this` into the
/// out word and returns its address.
const LOOKUP_0043B4F0: u32 = 0x0043_b4f0;
/// `004a3a20` (`thiscall`): the high 16 bits of the word at `this`.
const HIGH_HALF: u32 = 0x004a_3a20;

// Translated from 00819250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`). When the first word of the member
/// at `this + 0x594` (`00559450`) is not null, it is looked up through `0043b4f0` into a
/// stack word and the result is the high 16 bits of that word (`004a3a20` on the address
/// `0043b4f0` returns); otherwise 0.
pub fn fn_00819250(e: &mut Engine, this: Ptr) -> u32 {
    let member = this.addr() + 0x594;
    if e.call(FIRST_WORD, &args![member]).u32() == 0 {
        return 0;
    }
    e.with_stack(4, |e, out| {
        let first = e.call(FIRST_WORD, &args![member]).u32();
        let found = e.call(LOOKUP_0043B4F0, &args![first, out]).u32();
        e.call(HIGH_HALF, &args![found]).u32()
    })
}

/// The `MagicProjectile` base constructor `0081dbb0` (unnamed in the engine map;
/// `thiscall`, no stack words).
const MAGIC_PROJECTILE_CONSTRUCTOR: u32 = 0x0081_dbb0;
/// `MagicFogProjectile`'s two vtables (`0107ad84` at +0, `0107ad78` at +0x18; RTTI
/// `.?AVMagicFogProjectile@@`).
const MAGIC_FOG_PROJECTILE_VTABLE: u32 = 0x0107_ad84;
const MAGIC_FOG_PROJECTILE_VTABLE_18: u32 = 0x0107_ad78;
/// `0041a250` (`thiscall`, no stack words): constructs the sound handle member.
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
/// The destructor body `008196e0` (`magicfogprojectile.cpp` in the engine map).
const MAGIC_FOG_PROJECTILE_DESTRUCTOR: u32 = 0x0081_96e0;
/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 008192a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008192a0 - the FS:[0] exception frame (destroying the MagicProjectile base when a member constructor throws) is not translated
/// The `MagicFogProjectile` constructor (by the vtables it stores; unnamed in the engine
/// map; `thiscall`, plain `RET`): the `MagicProjectile` constructor `0081dbb0`, the two
/// vtables, the sound handle at +0xBC (`0041a250`; `Sound`, Xbox PDB +0xCC), then the words
/// at +0xCC (`pLight`, Xbox PDB +0xDC) and +0xC8 (`pTargetList`, Xbox PDB +0xD8) cleared.
/// Returns `this`.
pub fn fn_008192a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MAGIC_PROJECTILE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, MAGIC_FOG_PROJECTILE_VTABLE);
    e.mem.set_u32(base + 0x18, MAGIC_FOG_PROJECTILE_VTABLE_18);
    e.call(SOUND_HANDLE_CONSTRUCTOR, &args![base + 0xbc]);
    e.mem.set_u32(base + 0xcc, 0);
    e.mem.set_u32(base + 0xc8, 0);
    this
}

// Translated from 00819330 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor (`thiscall`, `RET 4`): the destructor body `008196e0`,
/// then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00819330(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAGIC_FOG_PROJECTILE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_9250, fn_00819250(Ptr) -> u32),
        entry!(0x0081_92a0, fn_008192a0(Ptr) -> Ptr),
        entry!(0x0081_9330, fn_00819330(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod n049_tests {
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
    fn fn_008192a0_constructs_the_fog_projectile() {
        let mut e = Engine::new();
        e.register(MAGIC_PROJECTILE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0xc8, 0xffff_ffff);
            e.mem.set_u32(a[0] + 0xcc, 0xffff_ffff);
            a[0].into_ret()
        });
        e.register(SOUND_HANDLE_CONSTRUCTOR, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xe0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_92a0, &[this]).u32(), this);
        assert_eq!(calls(&e, MAGIC_PROJECTILE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(calls(&e, SOUND_HANDLE_CONSTRUCTOR), vec![vec![this + 0xbc]]);
        assert_eq!(e.mem.u32(this), MAGIC_FOG_PROJECTILE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), MAGIC_FOG_PROJECTILE_VTABLE_18);
        assert_eq!(e.mem.u32(this + 0xc8), 0);
        assert_eq!(e.mem.u32(this + 0xcc), 0);
    }

    #[test]
    fn fn_00819330_deletes_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(MAGIC_FOG_PROJECTILE_DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_9330, &[0x500, 0]).u32(), 0x500);
        assert_eq!(e.call(0x0081_9330, &[0x500, 3]).u32(), 0x500);
        assert_eq!(calls(&e, MAGIC_FOG_PROJECTILE_DESTRUCTOR).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x500]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looks_up_the_member_and_answers_the_high_half() {
        let mut e = Engine::new();
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LOOKUP_0043B4F0, |e, a| {
            e.mem.set_u32(a[1], a[0].wrapping_mul(0x10));
            a[1].into_ret()
        });
        e.register(HIGH_HALF, |e, a| (e.mem.u32(a[0]) >> 16).into_ret());
        let this = e.mem.alloc(0x598);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_9250, &[this]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == LOOKUP_0043B4F0));

        e.mem.set_u32(this + 0x594, 0x0012_3450);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_9250, &[this]).u32(), 0x0123);
        let log = e.call_log.take().unwrap();
        let lookups: Vec<_> = log.iter().filter(|(a, _)| *a == LOOKUP_0043B4F0).collect();
        assert_eq!(lookups[0].1[0], 0x0012_3450);
        assert_eq!(log.iter().filter(|(a, _)| *a == FIRST_WORD).count(), 2);
    }
}
