//! `(unplaced)/run_005f9b70` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f9b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at +0x12C (`thiscall`; no Xbox PDB name).
pub fn fn_005f9b70(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x12c))
}

/// `__RTDynamicCast` (`00ec43fb`) and the RTTI type descriptors `.?AVTESForm@@` and
/// `.?AVTESCreature@@`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_CREATURE: u32 = 0x0118_3a00;
/// `TESForm::CompareAllComponents` (`00485270`) and `memcmp` (`00ec4835`).
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const MEMCMP: u32 = 0x00ec_4835;
/// The creature getters [`fn_005f9b90`] compares: a byte (`005f9ad0`), three floats
/// (`00821640`, `00567470`, `00821660`, ST0), and words.
const GETTER_005F9AD0: u32 = 0x005f_9ad0;
const FLOAT_GETTERS: [u32; 3] = [0x0082_1640, 0x0056_7470, 0x0082_1660];
const GETTER_009C0070: u32 = 0x009c_0070;
const GETTER_005F9100: u32 = 0x005f_9100;
const OBJECT_005F91D0: u32 = 0x005f_91d0;
const GETTER_005F9B30: u32 = 0x005f_9b30;
const GETTER_005F9B50: u32 = 0x005f_9b50;
/// `00461580` (`ECX` = the component at +0x30, one stack word: a mask), `AL`.
const FLAG_00461580: u32 = 0x0046_1580;
/// `005f06f0` (`ECX` = an object, one stack word: another), `AL`: whether they differ.
const OBJECTS_DIFFER_005F06F0: u32 = 0x005f_06f0;

// Translated from 005f9b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCreature`'s `Compare` slot (+0x10c of its vtable `01048f5c`; unnamed in the engine
/// map; `ECX` and one stack word, `RET 4`): true (different) at the first difference, in
/// the exe's order: `other` is not a `TESCreature`; `TESForm::CompareAllComponents`; the 4
/// bytes at +0x12c against the other's [`fn_005f9b70`]; `005f9ad0`'s byte; the three
/// floats (this one's stored as a `double`, compared unordered-as-different with
/// `FUCOMPP`); slot +0x58 of the component at +0x30; this vtable's slots +0x188 and
/// +0x180; `009c0070`; then, when flag 0x100 of the component at +0x30 (`00461580`) is
/// set, the `005f91d0` objects (one null and not the other, or `005f06f0` answering
/// true), else `005f9100`; then `005f9b30` and `005f9b50`. False when all match.
pub fn fn_005f9b90(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let creature = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, TYPE_CREATURE, 0u32],
        )
        .u32();
    if creature == 0 {
        return true;
    }
    let base = this.addr();
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, creature]).u8() != 0 {
        return true;
    }
    let theirs = fn_005f9b70(e, Ptr::new(creature)).addr();
    if e.call(MEMCMP, &args![base + 0x12c, theirs, 4u32]).u32() != 0 {
        return true;
    }
    let mine = e.call(GETTER_005F9AD0, &args![this]).u8();
    if mine != e.call(GETTER_005F9AD0, &args![creature]).u8() {
        return true;
    }
    for getter in FLOAT_GETTERS {
        // `FSTP double` of this one's, `FLD double`, `FUCOMPP`, `TEST AH,0x44`, `JNP`.
        let mine = e.call(getter, &args![this]).f64();
        let theirs = e.call(getter, &args![creature]).f64();
        #[allow(clippy::float_cmp)]
        let same = mine == theirs;
        if !same {
            return true;
        }
    }
    let mine = e.vcall(base + 0x30, 0x58, &args![]).u32();
    if mine != e.vcall(creature + 0x30, 0x58, &args![]).u32() {
        return true;
    }
    for slot in [0x188u32, 0x180] {
        let mine = e.vcall(base, slot, &args![]).u32();
        if mine != e.vcall(creature, slot, &args![]).u32() {
            return true;
        }
    }
    let mine = e.call(GETTER_009C0070, &args![this]).u32();
    if mine != e.call(GETTER_009C0070, &args![creature]).u32() {
        return true;
    }
    if e.call(FLAG_00461580, &args![base + 0x30, 0x100u32]).u8() != 0 {
        let mine = e.call(OBJECT_005F91D0, &args![this]).u32();
        let theirs = e.call(OBJECT_005F91D0, &args![creature]).u32();
        if mine != 0 {
            if theirs == 0 {
                return true;
            }
            if e.call(OBJECTS_DIFFER_005F06F0, &args![mine, theirs]).u8() != 0 {
                return true;
            }
        } else if theirs != 0 {
            return true;
        }
    } else {
        let mine = e.call(GETTER_005F9100, &args![this]).u32();
        if mine != e.call(GETTER_005F9100, &args![creature]).u32() {
            return true;
        }
    }
    for getter in [GETTER_005F9B30, GETTER_005F9B50] {
        let mine = e.call(getter, &args![this]).u32();
        if mine != e.call(getter, &args![creature]).u32() {
            return true;
        }
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_9b70, fn_005f9b70(Ptr) -> Ptr),
        entry!(0x005f_9b90, fn_005f9b90(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_member_at_0x12c() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x005f_9b70, &args![0x1000u32]).u32(), 0x112c);
    }
}

#[cfg(test)]
mod compare_tests {
    use super::*;

    /// Two creatures; every getter answers `differ_on`'s value for the other creature when
    /// it is the getter named (else 0); floats answer `float` for both, NaN included.
    fn compare(differ_on: u32, float: f64, flag: bool) -> bool {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x140);
        let other = e.mem.alloc(0x140);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| 0u32.into_ret());
        e.register(MEMCMP, |_, _| 0u32.into_ret());
        let mut slots = vec![0u32; 0x63];
        slots[0x58 / 4] = 0x0060_0058;
        slots[0x180 / 4] = 0x0060_0180;
        slots[0x188 / 4] = 0x0060_0188;
        e.put_vtable(0x0070_0000, &slots);
        for form in [this, other] {
            e.mem.set_u32(form, 0x0070_0000);
            e.mem.set_u32(form + 0x30, 0x0070_0000);
        }
        for getter in [
            GETTER_005F9AD0,
            GETTER_009C0070,
            GETTER_005F9100,
            OBJECT_005F91D0,
            GETTER_005F9B30,
            GETTER_005F9B50,
            0x0060_0058,
            0x0060_0180,
            0x0060_0188,
        ] {
            e.register_double(getter, move |_, a| {
                let theirs = a[0] == other || a[0] == other + 0x30;
                u32::from(theirs && getter == differ_on).into_ret()
            });
        }
        for getter in FLOAT_GETTERS {
            e.register_double(getter, move |_, _| Ret {
                st0: float,
                ..Ret::default()
            });
        }
        e.register_double(FLAG_00461580, move |_, _| u32::from(flag).into_ret());
        e.register(OBJECTS_DIFFER_005F06F0, |_, _| 0u32.into_ret());
        e.call(0x005f_9b90, &args![this, other]).bool()
    }

    #[test]
    fn fn_005f9b90_finds_equal_creatures_equal() {
        assert!(!compare(0, 1.5, false));
        assert!(!compare(0, 1.5, true));
    }

    #[test]
    fn fn_005f9b90_stops_at_each_difference() {
        for getter in [
            GETTER_005F9AD0,
            0x0060_0058,
            0x0060_0188,
            0x0060_0180,
            GETTER_009C0070,
            GETTER_005F9100,
            GETTER_005F9B30,
            GETTER_005F9B50,
        ] {
            assert!(compare(getter, 1.5, false), "{getter:08x}");
        }
        // With flag 0x100 the objects count: one null and not the other differs.
        assert!(compare(OBJECT_005F91D0, 1.5, true));
        assert!(!compare(GETTER_005F9100, 1.5, true));
        // A NaN float compares unordered: different.
        assert!(compare(0, f64::NAN, false));
    }
}
