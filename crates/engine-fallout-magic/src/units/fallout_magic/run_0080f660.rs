//! `fallout/magic/run_0080f660` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Returns a pointer to a record inside the object (`this + 0x24`; the engine
/// map files it under `fallout shared/extradataobjects.cpp`).
const OBJECT_RECORD_POINTER: u32 = 0x0043_0830;

// Translated from 0080f790 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: `__fastcall` with one stack word it never reads. Asks
/// `00430830` for the record pointer of `this` and returns the `float` at
/// `+8` of that record (loaded into `ST0`, so a signalling NaN comes back
/// quiet).
pub fn fn_0080f790(e: &mut Engine, this: Ptr, _unused_1: u32) -> f32 {
    let record = e.call(OBJECT_RECORD_POINTER, &args![this]).u32();
    x87_quiet(e.mem.f32(record + 8))
}

// Translated from 0080f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: stores the byte argument at `this + 0x87` (`RET 4`).
pub fn fn_0080f7b0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x87, value);
}

// Translated from 0080f7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: `__thiscall` with two stack words it never reads
/// (`RET 8`); returns `true` (`MOV AL,1`).
pub fn fn_0080f7d0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> bool {
    true
}

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The `LockEffect` constructor `0080f390` (`thiscall`, three words).
const LOCK_EFFECT_CONSTRUCTOR: u32 = 0x0080_f390;
/// The `MagicProjectile` base constructor `0081dbb0` (unnamed in the engine map;
/// `thiscall`, no stack words).
const MAGIC_PROJECTILE_CONSTRUCTOR: u32 = 0x0081_dbb0;
/// `MagicBallProjectile`'s two vtables (`0107a554` at +0, `0107a548` at +0x18; RTTI
/// `.?AVMagicBallProjectile@@`).
const MAGIC_BALL_PROJECTILE_VTABLE: u32 = 0x0107_a554;
const MAGIC_BALL_PROJECTILE_VTABLE_18: u32 = 0x0107_a548;
/// `0041a250` (`thiscall`, no stack words): constructs the sound handle member.
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
/// The destructor body `0080fb30` (`magicballprojectile.cpp` in the engine map).
const MAGIC_BALL_PROJECTILE_DESTRUCTOR: u32 = 0x0080_fb30;
/// Three words at `011f426c` that `0080f760` copies out (a point; which one is not
/// confirmed here).
const POINT_011F426C: u32 = 0x011f_426c;

// Translated from 0080f660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080f660 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x48-byte `LockEffect`
/// (`00401000`) and constructs it with the three words (`0080f390`); returns it, or null
/// when the allocation fails.
pub fn fn_0080f660(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x48u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        LOCK_EFFECT_CONSTRUCTOR,
        &args![block, caster, spell, effect],
    )
    .u32()
}

// Translated from 0080f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080f6e0 - the FS:[0] exception frame (destroying the MagicProjectile base when a member constructor throws) is not translated
/// The `MagicBallProjectile` constructor (by the vtables it stores; unnamed in the engine
/// map; `thiscall`, plain `RET`): the `MagicProjectile` constructor `0081dbb0`, the two
/// vtables (+0 and +0x18), the sound handle at +0xB4 (`0041a250`; `Sound`, Xbox PDB +0xC4)
/// and the word at +0xC0 cleared (`pLight`, Xbox PDB +0xD0). Returns `this`.
pub fn fn_0080f6e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MAGIC_PROJECTILE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, MAGIC_BALL_PROJECTILE_VTABLE);
    e.mem.set_u32(base + 0x18, MAGIC_BALL_PROJECTILE_VTABLE_18);
    e.call(SOUND_HANDLE_CONSTRUCTOR, &args![base + 0xb4]);
    e.mem.set_u32(base + 0xc0, 0);
    this
}

// Translated from 0080f760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; `this` is not read): copies the three
/// words at `011f426c` to `out` and returns `out`.
pub fn fn_0080f760(e: &mut Engine, _this: Ptr, out: u32) -> u32 {
    for i in 0..3 {
        let word = e.global::<u32>(POINT_011F426C + 4 * i);
        e.mem.set_u32(out + 4 * i, word);
    }
    out
}

// Translated from 0080f7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor (`thiscall`, `RET 4`): the destructor body `0080fb30`,
/// then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0080f7e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAGIC_BALL_PROJECTILE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_f660, fn_0080f660(u32, u32, u32) -> u32),
        entry!(0x0080_f6e0, fn_0080f6e0(Ptr) -> Ptr),
        entry!(0x0080_f760, fn_0080f760(Ptr, u32) -> u32),
        entry!(0x0080_f7e0, fn_0080f7e0(Ptr, u32) -> Ptr),
        entry!(0x0080f790, fn_0080f790(Ptr, u32) -> f32),
        entry!(0x0080_f7b0, fn_0080f7b0(Ptr, u8)),
        entry!(0x0080_f7d0, fn_0080f7d0(Ptr, u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_float_at_offset_8_of_the_record() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x10);
        e.mem.set_f32(record + 8, 2.5);
        e.register_double(OBJECT_RECORD_POINTER, move |_, a| {
            assert_eq!(a[0], 0x1234);
            Ret {
                eax: record,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        let value = e.call(0x0080f790, &args![0x1234u32, 0u32]).f32();
        assert_eq!(value, 2.5);
        // A signalling NaN is quieted by the FLD.
        e.mem.set_u32(record + 8, 0x7f80_0001);
        let nan = e.call(0x0080f790, &args![0x1234u32, 0u32]).f32();
        assert!(nan.is_nan());
    }

    #[test]
    fn fn_0080f7b0_stores_the_byte_at_0x87() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        e.mem.set_u8(this + 0x86, 0x11);
        e.mem.set_u8(this + 0x88, 0x22);
        e.call(0x0080_f7b0, &args![this, 0xffff_ff5au32]);
        assert_eq!(e.mem.u8(this + 0x87), 0x5a);
        assert_eq!(e.mem.u8(this + 0x86), 0x11);
        assert_eq!(e.mem.u8(this + 0x88), 0x22);
    }

    #[test]
    fn fn_0080f7d0_returns_true() {
        let mut e = Engine::new();
        assert!(e.call(0x0080_f7d0, &args![0x1000u32, 0u32, 0u32]).bool());
    }
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
    fn fn_0080f660_allocates_and_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(LOCK_EFFECT_CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_f660, &[7, 8, 9]).u32(), 0x5001);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x48]]);
        assert_eq!(
            calls(&e, LOCK_EFFECT_CONSTRUCTOR),
            vec![vec![0x5000, 7, 8, 9]]
        );
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_f660, &[7, 8, 9]).u32(), 0);
        assert!(calls(&e, LOCK_EFFECT_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn fn_0080f6e0_constructs_the_ball_projectile() {
        let mut e = Engine::new();
        e.register(MAGIC_PROJECTILE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0xc0, 0xffff_ffff);
            a[0].into_ret()
        });
        e.register(SOUND_HANDLE_CONSTRUCTOR, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xd0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_f6e0, &[this]).u32(), this);
        assert_eq!(calls(&e, MAGIC_PROJECTILE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(calls(&e, SOUND_HANDLE_CONSTRUCTOR), vec![vec![this + 0xb4]]);
        assert_eq!(e.mem.u32(this), MAGIC_BALL_PROJECTILE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), MAGIC_BALL_PROJECTILE_VTABLE_18);
        assert_eq!(e.mem.u32(this + 0xc0), 0);
    }

    #[test]
    fn fn_0080f760_copies_the_point() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.set_global::<f32>(POINT_011F426C, 1.5);
        e.set_global::<f32>(POINT_011F426C + 4, -2.0);
        e.set_global::<u32>(POINT_011F426C + 8, 0x7f80_0001);
        let out = e.mem.alloc(0xc);
        assert_eq!(e.call(0x0080_f760, &[0x1234, out]).u32(), out);
        assert_eq!(e.mem.f32(out), 1.5);
        assert_eq!(e.mem.f32(out + 4), -2.0);
        // Integer moves: a signalling NaN pattern is copied unchanged.
        assert_eq!(e.mem.u32(out + 8), 0x7f80_0001);
    }

    #[test]
    fn fn_0080f7e0_deletes_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(MAGIC_BALL_PROJECTILE_DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_f7e0, &[0x500, 2]).u32(), 0x500);
        assert_eq!(e.call(0x0080_f7e0, &[0x500, 1]).u32(), 0x500);
        assert_eq!(calls(&e, MAGIC_BALL_PROJECTILE_DESTRUCTOR).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x500]]);
    }
}
