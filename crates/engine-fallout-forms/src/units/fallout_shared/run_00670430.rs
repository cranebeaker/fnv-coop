//! `fallout shared/run_00670430` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (`00ec43fb`), the `TESForm` type descriptor
/// (`01183028`) and the `ActorValueInfo` one (`0119a554`,
/// `.?AVActorValueInfo@@`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_ACTOR_VALUE_INFO: u32 = 0x0119_a554;

fn cast_actor_value_info(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_ACTOR_VALUE_INFO, 0u32],
    )
    .u32()
}

// Translated from 00670430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies another actor value info (`RET 4`; the engine map gives it no
/// name): first the form's virtual slot 0x18, then, when `other` is an
/// `ActorValueInfo` (`__RTDynamicCast` from `TESForm`), the word at +0x38,
/// the string at +0x3C (`00501c10`), the words at +0x44, +0x4C, +0x50,
/// +0x54 and the count at +0x58, the first `count` (at most 0xF) words of
/// the array at +0x5C, the rest of its 0xF words set to 0x4D, and all
/// components (`004851b0`).
pub fn fn_00670430(e: &mut Engine, this: Ptr, other: Ptr) {
    let base = this.addr();
    e.vcall(base, 0x18, &[]);
    let cast = cast_actor_value_info(e, other);
    if cast == 0 {
        return;
    }
    let word = e.mem.u32(cast + 0x38);
    e.mem.set_u32(base + 0x38, word);
    e.call(0x0050_1c10, &args![base + 0x3c, cast + 0x3c]);
    for offset in [0x44, 0x4c, 0x50, 0x54, 0x58] {
        let word = e.mem.u32(cast + offset);
        e.mem.set_u32(base + offset, word);
    }
    let mut index = 0i32;
    while index < e.mem.i32(base + 0x58) && index < 0xf {
        let word = e.mem.u32(cast + 0x5c + 4 * index as u32);
        e.mem.set_u32(base + 0x5c + 4 * index as u32, word);
        index += 1;
    }
    while index < 0xf {
        e.mem.set_u32(base + 0x5c + 4 * index as u32, 0x4d);
        index += 1;
    }
    e.call(0x0048_51b0, &args![this, other]);
}

// Translated from 00670540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares with another actor value info (`RET 4`): true (they differ)
/// unless `other` is an `ActorValueInfo` whose components
/// (`TESForm::CompareAllComponents`, `00485270`) and string at +0x3C
/// (`00501c30`) are the same.
pub fn fn_00670540(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = cast_actor_value_info(e, other);
    if cast == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, cast]).u8() != 0 {
        return true;
    }
    e.call(0x0050_1c30, &args![this.addr() + 0x3c, cast + 0x3c])
        .u8()
        != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_0430, fn_00670430(Ptr, Ptr)),
        entry!(0x0067_0540, fn_00670540(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(0x0f00_1018, |_, _| Ret::default());
        let mut slots = vec![0u32; 7];
        slots[6] = 0x0f00_1018;
        e.put_vtable(0x0f00_0000, &slots);
        e
    }

    #[test]
    fn copy_takes_the_fields_and_fills_the_array() {
        let mut e = engine();
        e.register(0x0050_1c10, |_, _| Ret::default());
        e.register(0x0048_51b0, |_, _| Ret::default());
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this, 0x0f00_0000);
        let other = e.mem.alloc(0xa0);
        for i in 0..0x28 {
            e.mem.set_u32(other + 0x38 + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(other + 0x58, 3);
        e.mem.set_u32(this + 0x48, 0xdead);
        e.call_log = Some(vec![]);
        e.call(0x0067_0430, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0f00_1018, vec![this]));
        assert!(log.contains(&(0x0050_1c10, vec![this + 0x3c, other + 0x3c])));
        assert_eq!(log.last().unwrap(), &(0x0048_51b0, vec![this, other]));
        assert_eq!(e.mem.u32(this + 0x38), 0x100);
        assert_eq!(e.mem.u32(this + 0x48), 0xdead);
        assert_eq!(e.mem.u32(this + 0x54), 0x107);
        let array: Vec<u32> = (0..0xf).map(|i| e.mem.u32(this + 0x5c + 4 * i)).collect();
        let mut expected = vec![0x109, 0x10a, 0x10b];
        expected.extend([0x4d; 12]);
        assert_eq!(array, expected);
        // A count above 0xF copies only 0xF words.
        e.mem.set_u32(other + 0x58, 0x20);
        e.call(0x0067_0430, &args![this, other]);
        assert_eq!(e.mem.u32(this + 0x5c + 4 * 14), 0x109 + 14);
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_0430, &args![this, other]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }

    #[test]
    fn compare_checks_components_and_the_string() {
        let mut e = engine();
        e.register(0x0048_5270, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0050_1c30, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        assert!(!e.call(0x0067_0540, &args![this, other]).bool());
        e.mem.set_u32(this + 0x3c, 1);
        assert!(e.call(0x0067_0540, &args![this, other]).bool());
        e.mem.set_u32(this + 0x3c, 0);
        e.mem.set_u32(this + 4, 1);
        assert!(e.call(0x0067_0540, &args![this, other]).bool());
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        e.mem.set_u32(this + 4, 0);
        assert!(e.call(0x0067_0540, &args![this, other]).bool());
    }
}
