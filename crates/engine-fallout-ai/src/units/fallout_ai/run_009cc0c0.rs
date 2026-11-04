//! `fallout/ai/run_009cc0c0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009cc220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01090c04` (RTTI `.?AVCombatProcedureAttackMelee@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009cc220(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_0c04);
    e.call(0x009c_a6c0, &args![this]);
}

/// `009ca680` (unnamed): the base constructor (`ECX` only).
const BASE_CONSTRUCT_009CA680: u32 = 0x009c_a680;
/// The vtable `01090c04` (RTTI `.?AVCombatProcedureAttackMelee@@`).
const ATTACK_MELEE_VTABLE: u32 = 0x0109_0c04;
/// `00906f60` (unnamed): stores 0.0 in the two floats at `this` (`ECX` only).
const CLEAR_TWO_FLOATS: u32 = 0x0090_6f60;
/// `004300a0` (unnamed): stores 0.0 in the float at `this` (`ECX` only).
const CLEAR_FLOAT: u32 = 0x0043_00a0;
/// `009cff10` (unnamed, `combatprocedureattackmelee.cpp`): constructs the member at
/// `+0x64` (`ECX` only).
const MEMBER_CONSTRUCT_009CFF10: u32 = 0x009c_ff10;
/// `-1.0` (`float`, `0x01012054`) and `FLT_MAX` (`float`, `0x01090bc0`).
const MINUS_ONE: u32 = 0x0101_2054;
const FLOAT_MAX: u32 = 0x0109_0bc0;
/// `00435de0` (unnamed, `extradataobjects.cpp`): `thiscall`, one float (`RET 4`); stores
/// it at `this`.
const SET_TIME_STAMP: u32 = 0x0043_5de0;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 009cc0c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009cc0c0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`ECX` only): the constructor of the object with the vtable
/// `01090c04` (RTTI `.?AVCombatProcedureAttackMelee@@`): the base constructor `009ca680`,
/// the vtable, the float pairs at `+0x20` to `+0x48` (`00906f60`), the float at `+0x60`
/// (`004300a0`), the member at `+0x64` (`009cff10`); then `+4` 0, `+0x1c` 0xff, `+0x10` 0,
/// `+0x14` and `+0x18` 3, `+0x54` 0x100, the byte at `+0x50` 0, the floats at `+0x58` and
/// `+0x5c` -1.0, and `+0x60` the time stamp of `-FLT_MAX` (`00435de0` on a stack local).
/// Returns `this`.
pub fn fn_009cc0c0(e: &mut Engine, this: Ptr) -> Ptr {
    let p = this.addr();
    e.call(BASE_CONSTRUCT_009CA680, &args![this]);
    e.mem.set_u32(p, ATTACK_MELEE_VTABLE);
    for offset in [0x20u32, 0x28, 0x30, 0x38, 0x40, 0x48] {
        e.call(CLEAR_TWO_FLOATS, &args![p + offset]);
    }
    e.call(CLEAR_FLOAT, &args![p + 0x60]);
    e.call(MEMBER_CONSTRUCT_009CFF10, &args![p + 0x64]);
    e.mem.set_u32(p + 0x4, 0);
    e.mem.set_u32(p + 0x1c, 0xff);
    e.mem.set_u32(p + 0x10, 0);
    e.mem.set_u32(p + 0x14, 3);
    e.mem.set_u32(p + 0x18, 3);
    e.mem.set_u32(p + 0x54, 0x100);
    e.mem.set_u8(p + 0x50, 0);
    let minus_one = x87_quiet_bits(e.mem.u32(MINUS_ONE));
    e.mem.set_u32(p + 0x58, minus_one);
    let minus_one = x87_quiet_bits(e.mem.u32(MINUS_ONE));
    e.mem.set_u32(p + 0x5c, minus_one);
    let negated = x87_store32(-x87_load32(e.mem.u32(FLOAT_MAX)));
    let stamp = e.with_stack(4, |e, local| {
        e.call(SET_TIME_STAMP, &args![local, negated]);
        e.mem.u32(local.addr())
    });
    e.mem.set_u32(p + 0x60, stamp);
    this
}

// Translated from 009cc1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor; the
/// destructor [`fn_009cc220`], then `operator delete` on `this` for bit 0 of `flags`.
/// Returns `this`.
pub fn fn_009cc1f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009cc220(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009c_c220, fn_009cc220(Ptr)),
        entry!(0x009c_c0c0, fn_009cc0c0(Ptr) -> Ptr),
        entry!(0x009c_c1f0, fn_009cc1f0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009cc220_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x009c_a6c0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009c_c220, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_0c04);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_0c04,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

    #[test]
    fn fn_009cc0c0_constructs_the_procedure() {
        let mut e = Engine::new();
        e.map(MINUS_ONE & !0xfff, 0x1000);
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.map(FLOAT_MAX & !0xfff, 0x1000);
        e.mem.set_f32(FLOAT_MAX, f32::MAX);
        for addr in [
            BASE_CONSTRUCT_009CA680,
            CLEAR_TWO_FLOATS,
            CLEAR_FLOAT,
            MEMBER_CONSTRUCT_009CFF10,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SET_TIME_STAMP, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = e.mem.alloc(0x70);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009c_c0c0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let cleared: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == CLEAR_TWO_FLOATS)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(cleared, vec![0x20, 0x28, 0x30, 0x38, 0x40, 0x48]);
        assert!(log.contains(&(CLEAR_FLOAT, vec![this + 0x60])));
        assert!(log.contains(&(MEMBER_CONSTRUCT_009CFF10, vec![this + 0x64])));
        assert_eq!(e.mem.u32(this), ATTACK_MELEE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x1c), 0xff);
        assert_eq!(e.mem.u32(this + 0x14), 3);
        assert_eq!(e.mem.u32(this + 0x18), 3);
        assert_eq!(e.mem.u32(this + 0x54), 0x100);
        assert_eq!(e.mem.f32(this + 0x58), -1.0);
        assert_eq!(e.mem.f32(this + 0x5c), -1.0);
        assert_eq!(e.mem.f32(this + 0x60), -f32::MAX);
    }

    #[test]
    fn fn_009cc1f0_destroys_and_frees_for_bit_0() {
        let mut e = Engine::new();
        e.register(0x009c_a6c0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x70);
        for flags in [0u32, 1] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009c_c1f0, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let frees = log.iter().filter(|c| c.0 == OPERATOR_DELETE).count();
            assert_eq!(frees, flags as usize);
        }
        assert_eq!(e.mem.u32(this), ATTACK_MELEE_VTABLE);
    }
}
