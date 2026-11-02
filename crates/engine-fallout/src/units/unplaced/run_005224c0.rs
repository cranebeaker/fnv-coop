//! `(unplaced)/run_005224c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0046e8c0` (`tesdatahandler.cpp`; thiscall on `this`, a `bool` in AL).
const CHECK_0046E8C0: u32 = 0x0046_e8c0;
/// Reads the word at its `this` (`00559450`, thiscall).
const READ_POINTER: u32 = 0x0055_9450;
/// `004fb7a0` (`bgsexplosion.cpp`; thiscall on the object at +0x24c, one
/// word).
const CALLEE_004FB7A0: u32 = 0x004f_b7a0;
/// The fields read (offsets as the code reads them).
const OBJECT_24C: u32 = 0x24c;
const HOLDER_35C: u32 = 0x35c;
const BYTE_11F: u32 = 0x11f;

// Translated from 005224c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x22c of `this` (unnamed in the engine map).
pub fn fn_005224c0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x22c)
}

// Translated from 00522500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x234 of `this` (unnamed in the engine map).
pub fn fn_00522500(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x234)
}

// Translated from 00522520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x238 of `this` (unnamed in the engine map).
pub fn fn_00522520(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x238)
}

// Translated from 00522540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x23c of `this` (unnamed in the engine map).
pub fn fn_00522540(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x23c)
}

// Translated from 00522ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004fb7a0(object, value)` on the object at +0x24c when there is one,
/// else 0 (thiscall, `RET 4`; unnamed in the engine map).
pub fn fn_00522ba0(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let object = e.mem.u32(this.addr() + OBJECT_24C);
    if object == 0 {
        return 0;
    }
    e.call(CALLEE_004FB7A0, &args![object, value]).u32()
}

// Translated from 00522c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `0046e8c0(this)` answers true, the pointer held at +0x35c (read by
/// `00559450`); else 0 (unnamed in the engine map).
pub fn fn_00522c00(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(CHECK_0046E8C0, &args![this]).u32() & 0xff == 0 {
        return 0;
    }
    e.call(READ_POINTER, &args![this.addr() + HOLDER_35C]).u32()
}

// Translated from 00522c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `-1` unless `0046e8c0(this)` answers true; then the byte at +0x11f plus
/// 0x19, at most 0x1f (a signed comparison with 0x20; unnamed in the
/// engine map).
pub fn fn_00522c30(e: &mut Engine, this: Ptr) -> u32 {
    let mut value = u32::MAX;
    if e.call(CHECK_0046E8C0, &args![this]).u32() & 0xff != 0 {
        value = u32::from(e.mem.u8(this.addr() + BYTE_11F)) + 0x19;
        if value as i32 >= 0x20 {
            value = 0x1f;
        }
    }
    value
}

// Translated from 005224e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x230 of `this` (unnamed in the engine map; called by `PlayWeaponBlock`,
/// `00522020` and [`fn_00522590`]).
pub fn fn_005224e0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x230)
}

// Translated from 00522560 (decompiled, FalloutNV.exe 1.4.0.525)
/// 2 when `fixed` is nonzero, else the word at +0x364 of `this` (unnamed in the engine map;
/// `ECX` and one byte word, `RET 4`).
pub fn fn_00522560(e: &mut Engine, this: Ptr, fixed: u8) -> u32 {
    if fixed != 0 {
        2
    } else {
        e.mem.u32(this.addr() + 0x364)
    }
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference) and the RTTI type descriptors `.?AVTESForm@@` and `.?AVTESObjectWEAP@@`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_WEAPON: u32 = 0x0118_3998;
/// `TESForm::CompareAllComponents` (Xbox PDB, `00485270`, `ECX`, one stack word), `AL`.
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `00501c90` (`ECX` = this one's holder, two stack words: the other's and 1): nonzero
/// when they differ.
const HOLDERS_DIFFER: u32 = 0x0050_1c90;
/// `006e4fb0` (`ECX` = the other weapon): the address of its 0xcc bytes compared with
/// this one's +0xf4.
const DATA_ADDRESS: u32 = 0x006e_4fb0;
/// The CRT's `memcmp` (`00ec4835`, `cdecl`).
const MEMCMP: u32 = 0x00ec_4835;

/// The components [`fn_00522590`] compares through their vtable slot +0xc, in order.
const COMPARED_COMPONENTS: [u32; 10] = [
    0x1d0, 0x200, 0x270, 0x290, 0x2b0, 0x2d0, 0x2f0, 0x310, 0x330, 0x1e8,
];
/// The `ECX`-only getters compared before and after the byte compares (outside this file).
const GETTERS_FIRST: [u32; 3] = [0x0051_f550, 0x008d_8ac0, 0x008d_8b00];

/// Whether getter `getter` (a direct call when it is in this file, else by address) answers
/// the same word for both weapons (`this` asked first).
fn same_word(e: &mut Engine, this: u32, other: u32, getter: u32) -> bool {
    let mine = word_of(e, this, getter);
    mine == word_of(e, other, getter)
}

fn word_of(e: &mut Engine, object: u32, getter: u32) -> u32 {
    let object = Ptr::new(object);
    match getter {
        0x0052_24c0 => fn_005224c0(e, object),
        0x0052_24e0 => fn_005224e0(e, object),
        0x0052_2500 => fn_00522500(e, object),
        0x0052_2520 => fn_00522520(e, object),
        0x0052_2540 => fn_00522540(e, object),
        _ => e.call(getter, &args![object]).u32(),
    }
}

// Translated from 00522590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP`'s `Compare` slot (the vtable slot at `0102c628`; unnamed in the engine
/// map; `ECX` and one stack word, `RET 4`): true (different) at the first difference, in
/// the exe's order: `other` is not a `TESObjectWEAP`; `TESForm::CompareAllComponents`; the
/// ten components' slot +0xc (+0x1d0, +0x200, +0x270 to +0x330, +0x1e8); the getters
/// `0051f550`, `008d8ac0`, `008d8b00`; `004ab500(i)` for 1 to 7 and `004bd570(i)` for 1, 2,
/// 4; the holders at +0x35c and +0x368 (`00501c90` with 1); `memcmp` of +0xf4 with
/// `006e4fb0`'s 0xcc bytes, of +0x1c0 (0x10) and +0x370 (0x14); `005215a0(0)`,
/// `00522440(0)`, `00522470(0)`; the getters `008bffa0`, [`fn_005224c0`], [`fn_005224e0`],
/// [`fn_00522500`], [`fn_00522520`], [`fn_00522540`], `00521630`, `008d8a60`, `005224a0`;
/// and the word at +0x364 against the other's [`fn_00522560`]`(0)`. False when all match.
pub fn fn_00522590(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let weapon = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, TYPE_WEAPON, 0u32],
        )
        .u32();
    if weapon == 0 {
        return true;
    }
    let base = this.addr();
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, weapon]).u8() != 0 {
        return true;
    }
    for offset in COMPARED_COMPONENTS {
        if e.vcall(base + offset, 0xc, &args![weapon + offset]).u8() != 0 {
            return true;
        }
    }
    for getter in GETTERS_FIRST {
        if !same_word(e, base, weapon, getter) {
            return true;
        }
    }
    for (getter, indices) in [
        (0x004a_b500u32, &[1u32, 2, 3, 4, 5, 6, 7][..]),
        (0x004b_d570, &[1, 2, 4][..]),
    ] {
        for index in indices {
            let mine = e.call(getter, &args![base, *index]).u32();
            if mine != e.call(getter, &args![weapon, *index]).u32() {
                return true;
            }
        }
    }
    for offset in [0x35cu32, 0x368] {
        if e.call(HOLDERS_DIFFER, &args![base + offset, weapon + offset, 1u32])
            .u32()
            != 0
        {
            return true;
        }
    }
    let data = e.call(DATA_ADDRESS, &args![weapon]).u32();
    if e.call(MEMCMP, &args![base + 0xf4, data, 0xccu32]).u32() != 0 {
        return true;
    }
    for (offset, size) in [(0x1c0u32, 0x10u32), (0x370, 0x14)] {
        if e.call(MEMCMP, &args![base + offset, weapon + offset, size])
            .u32()
            != 0
        {
            return true;
        }
    }
    for getter in [0x0052_15a0u32, 0x0052_2440, 0x0052_2470] {
        let mine = e.call(getter, &args![base, 0u32]).u32();
        if mine != e.call(getter, &args![weapon, 0u32]).u32() {
            return true;
        }
    }
    for getter in [
        0x008b_ffa0u32,
        0x0052_24c0,
        0x0052_24e0,
        0x0052_2500,
        0x0052_2520,
        0x0052_2540,
        0x0052_1630,
        0x008d_8a60,
        0x0052_24a0,
    ] {
        if !same_word(e, base, weapon, getter) {
            return true;
        }
    }
    let condition = fn_00522560(e, Ptr::new(weapon), 0);
    e.mem.u32(base + 0x364) != condition
}

// Translated from 00522be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object held at +0x368 of `this` (`00559450`; unnamed in the engine map; called by
/// `007eb920` and `UpdateQueuedActionList`).
pub fn fn_00522be0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(READ_POINTER, &args![this.addr() + 0x368]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_24e0, fn_005224e0(Ptr) -> u32),
        entry!(0x0052_2560, fn_00522560(Ptr, u8) -> u32),
        entry!(0x0052_2590, fn_00522590(Ptr, Ptr) -> bool),
        entry!(0x0052_2be0, fn_00522be0(Ptr) -> u32),
        entry!(0x0052_24c0, fn_005224c0(Ptr) -> u32),
        entry!(0x0052_2500, fn_00522500(Ptr) -> u32),
        entry!(0x0052_2520, fn_00522520(Ptr) -> u32),
        entry!(0x0052_2540, fn_00522540(Ptr) -> u32),
        entry!(0x0052_2ba0, fn_00522ba0(Ptr, u32) -> u32),
        entry!(0x0052_2c00, fn_00522c00(Ptr) -> u32),
        entry!(0x0052_2c30, fn_00522c30(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_getters_read_their_offsets() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x240);
        for offset in [0x22cu32, 0x234, 0x238, 0x23c] {
            e.mem.set_u32(this + offset, 0x1000 + offset);
        }
        assert_eq!(e.call(0x0052_24c0, &args![this]).u32(), 0x122c);
        assert_eq!(e.call(0x0052_2500, &args![this]).u32(), 0x1234);
        assert_eq!(e.call(0x0052_2520, &args![this]).u32(), 0x1238);
        assert_eq!(e.call(0x0052_2540, &args![this]).u32(), 0x123c);
    }

    #[test]
    fn fn_00522ba0_asks_the_object_when_there_is_one() {
        let mut e = Engine::new();
        e.register(CALLEE_004FB7A0, |_, a| (a[0] ^ a[1]).into_ret());
        let this = e.mem.alloc(0x250);
        assert_eq!(e.call(0x0052_2ba0, &args![this, 5u32]).u32(), 0);
        e.mem.set_u32(this + OBJECT_24C, 0x0700_0000);
        assert_eq!(e.call(0x0052_2ba0, &args![this, 5u32]).u32(), 0x0700_0005);
    }

    #[test]
    fn fn_00522c00_reads_the_holder_only_when_the_check_passes() {
        let mut e = Engine::new();
        e.register(CHECK_0046E8C0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(READ_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0x360);
        e.mem.set_u32(this + HOLDER_35C, 0x0abc_0000);
        e.mem.set_u32(this, 0xffff_ff00);
        assert_eq!(e.call(0x0052_2c00, &args![this]).u32(), 0);
        e.mem.set_u32(this, 1);
        assert_eq!(e.call(0x0052_2c00, &args![this]).u32(), 0x0abc_0000);
    }

    #[test]
    fn fn_00522c30_adds_0x19_and_caps_at_0x1f() {
        let mut e = Engine::new();
        e.register(CHECK_0046E8C0, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0x120);
        assert_eq!(e.call(0x0052_2c30, &args![this]).u32(), u32::MAX);
        e.mem.set_u32(this, 1);
        for (byte, expected) in [(0u8, 0x19u32), (6, 0x1f), (7, 0x1f), (0xff, 0x1f)] {
            e.mem.set_u8(this + BYTE_11F, byte);
            assert_eq!(e.call(0x0052_2c30, &args![this]).u32(), expected);
        }
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    #[test]
    fn fn_005224e0_and_00522560_read_their_words() {
        let mut e = Engine::new();
        let weapon = e.mem.alloc(0x370);
        e.mem.set_u32(weapon + 0x230, 0x1234);
        e.mem.set_u32(weapon + 0x364, 5);
        assert_eq!(e.call(0x0052_24e0, &args![weapon]).u32(), 0x1234);
        assert_eq!(e.call(0x0052_2560, &args![weapon, 0x100u32]).u32(), 5);
        assert_eq!(e.call(0x0052_2560, &args![weapon, 1u32]).u32(), 2);
    }

    #[test]
    fn fn_00522be0_reads_the_holder_at_0x368() {
        let mut e = Engine::new();
        e.register(READ_POINTER, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x0052_2be0, &args![0x1000u32]).u32(), 0x1369);
    }

    const COMPONENT_COMPARE: u32 = 0x0060_000c;

    /// Two weapons with equal everything; `differ` names the callee whose answer differs
    /// for the second weapon (or makes a compare answer true), 0 for none.
    fn compare(differ: u32) -> (bool, Log) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x400);
        let other = e.mem.alloc(0x400);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.put_vtable(0x0070_0000, &[0, 0, 0, COMPONENT_COMPARE]);
        for offset in COMPARED_COMPONENTS {
            e.mem.set_u32(this + offset, 0x0070_0000);
        }
        let mut callees = vec![
            COMPARE_ALL_COMPONENTS,
            COMPONENT_COMPARE,
            HOLDERS_DIFFER,
            MEMCMP,
            0x004a_b500,
            0x004b_d570,
            0x0052_15a0,
            0x0052_2440,
            0x0052_2470,
            0x008b_ffa0,
            0x0052_1630,
            0x008d_8a60,
            0x0052_24a0,
        ];
        callees.extend(GETTERS_FIRST);
        for callee in callees {
            let flags = matches!(
                callee,
                COMPARE_ALL_COMPONENTS | COMPONENT_COMPARE | HOLDERS_DIFFER | MEMCMP
            );
            e.register_double(callee, move |_, a| {
                if flags {
                    u32::from(callee == differ).into_ret()
                } else if callee == differ && a[0] == other {
                    1u32.into_ret()
                } else {
                    0u32.into_ret()
                }
            });
        }
        e.register(DATA_ADDRESS, |_, a| (a[0] + 0xf4).into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(0x0052_2590, &args![this, other]).bool();
        (r, e.call_log.take().unwrap())
    }

    #[test]
    fn fn_00522590_finds_equal_weapons_equal() {
        let (r, log) = compare(0);
        assert!(!r);
        let count = |a: u32| log.iter().filter(|(c, _)| *c == a).count();
        assert_eq!(count(COMPONENT_COMPARE), 10);
        assert_eq!(count(0x004a_b500), 14);
        assert_eq!(count(0x004b_d570), 6);
        assert_eq!(count(MEMCMP), 3);
        assert_eq!(count(HOLDERS_DIFFER), 2);
    }

    #[test]
    fn fn_00522590_stops_at_each_difference() {
        for callee in [
            COMPARE_ALL_COMPONENTS,
            COMPONENT_COMPARE,
            0x0051_f550,
            0x004a_b500,
            0x004b_d570,
            HOLDERS_DIFFER,
            MEMCMP,
            0x0052_2470,
            0x0052_24a0,
        ] {
            assert!(compare(callee).0, "{callee:08x}");
        }
    }

    #[test]
    fn fn_00522590_compares_the_local_words() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| 0u32.into_ret());
        e.register(COMPONENT_COMPARE, |_, _| 0u32.into_ret());
        e.register(HOLDERS_DIFFER, |_, _| 0u32.into_ret());
        e.register(MEMCMP, |_, _| 0u32.into_ret());
        e.register(DATA_ADDRESS, |_, a| (a[0] + 0xf4).into_ret());
        for callee in [
            0x004a_b500u32,
            0x004b_d570,
            0x0052_15a0,
            0x0052_2440,
            0x0052_2470,
            0x008b_ffa0,
            0x0052_1630,
            0x008d_8a60,
            0x0052_24a0,
            0x0051_f550,
            0x008d_8ac0,
            0x008d_8b00,
        ] {
            e.register(callee, |_, _| 0u32.into_ret());
        }
        e.put_vtable(0x0070_0000, &[0, 0, 0, COMPONENT_COMPARE]);
        let this = e.mem.alloc(0x400);
        let other = e.mem.alloc(0x400);
        for offset in COMPARED_COMPONENTS {
            e.mem.set_u32(this + offset, 0x0070_0000);
        }
        assert!(!e.call(0x0052_2590, &args![this, other]).bool());
        e.mem.set_u32(other + 0x230, 9);
        assert!(e.call(0x0052_2590, &args![this, other]).bool());
        e.mem.set_u32(other + 0x230, 0);
        e.mem.set_u32(this + 0x364, 2);
        assert!(e.call(0x0052_2590, &args![this, other]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.mem.set_u32(this + 0x364, 0);
        assert!(e.call(0x0052_2590, &args![this, other]).bool());
    }
}
