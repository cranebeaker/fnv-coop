//! `(unplaced)/run_00632f90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word `Setting` base constructor (`00404920`), this group's setting
/// vtable and its collection (`00632e20`).
const SETTING_CONSTRUCT_WORD: u32 = 0x0040_4920;
const SETTING_VTABLE: u32 = 0x0104_e6e8;
const COLLECTION: u32 = 0x0063_2e20;
/// Slot of a settings collection's vtable that adds a setting.
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 00633000 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a word setting of the `00632e20` collection
/// (`thiscall`, `RET 8`): the word base constructor (`00404920`), the vtable
/// `0104e6e8`, then adds itself to the collection (vtable slot +4). Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_00633000(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(SETTING_CONSTRUCT_WORD, &args![this, name, value]);
    e.mem.set_u32(this.addr(), SETTING_VTABLE);
    let collection = e.call(COLLECTION, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

/// The collection's vtable slot `00632f90` calls with the setting.
const COLLECTION_SLOT_8: u32 = 0x8;
/// `00404950` (`thiscall`): the base destructor the setting destructor ends
/// with.
const SETTING_BASE_DESTRUCT: u32 = 0x0040_4950;

// Translated from 00632f90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00632f90 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The destructor of a setting of the `00632e20` collection (unnamed in the
/// engine map; `thiscall`, plain `RET`): stores the vtable `0104e6e8` (RTTI
/// `SettingT<BlendSettingCollection>`), calls the collection's virtual slot
/// `+8` with `this`, then runs the base destructor `00404950`.
pub fn fn_00632f90(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SETTING_VTABLE);
    let collection = e.call(COLLECTION, &[]).u32();
    e.vcall(collection, COLLECTION_SLOT_8, &args![this]);
    e.call(SETTING_BASE_DESTRUCT, &args![this]);
}

/// `0063f830` (`SkyObject::SkyObject`, Xbox PDB name in the engine map).
const SKY_OBJECT_CONSTRUCT: u32 = 0x0063_f830;
/// The vtable `0104eb68` (RTTI `.?AVAtmosphere@@`).
const ATMOSPHERE_VTABLE: u32 = 0x0104_eb68;
/// `00633c90` and `0066b0d0` (`thiscall`, one word each, unnamed in the
/// engine map): run with 0 on each of the four pointer members.
const MEMBER_CONSTRUCT: u32 = 0x0063_3c90;
const MEMBER_SET: u32 = 0x0066_b0d0;

// Translated from 00633080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00633080 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// An `Atmosphere` constructor (unnamed in the engine map; `thiscall`, plain
/// `RET`): the `SkyObject` constructor `0063f830`, the vtable `0104eb68`,
/// `00633c90` with 0 on `+0x8`, `+0xc`, `+0x10`, `+0x14`, then `0066b0d0`
/// with 0 on `+0x8`, `+0x14`, `+0x10`, `+0xc` (that order), and the byte at
/// `+0x18` set to 1. Returns `this`.
pub fn fn_00633080(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SKY_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), ATMOSPHERE_VTABLE);
    for offset in [0x8u32, 0xc, 0x10, 0x14] {
        e.call(
            MEMBER_CONSTRUCT,
            &args![this.addr().wrapping_add(offset), 0u32],
        );
    }
    for offset in [0x8u32, 0x14, 0x10, 0xc] {
        e.call(MEMBER_SET, &args![this.addr().wrapping_add(offset), 0u32]);
    }
    e.mem.set_u8(this.addr().wrapping_add(0x18), 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00633000, fn_00633000(Ptr, u32, u32) -> Ptr),
        entry!(0x0063_2f90, fn_00632f90(Ptr)),
        entry!(0x0063_3080, fn_00633080(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_setting_joins_its_collection() {
        let mut e = Engine::new();
        let c = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(c, 0x0090_0000);
        e.register_double(COLLECTION, move |_, _| c.into_ret());
        e.register(0x0090_1000, |_, _| Ret::default());
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x00633000, &[s, 0x20, 9]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SETTING_CONSTRUCT_WORD, vec![s, 0x20, 9]));
        assert_eq!(e.mem.u32(s), SETTING_VTABLE);
        assert_eq!(log.last().unwrap(), &(0x0090_1000, vec![c, s]));
    }
}

#[cfg(test)]
mod b015_tests {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()
    }

    #[test]
    fn setting_destructor_hands_itself_to_slot_8_then_runs_the_base() {
        let mut e = Engine::new();
        let c = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0, 0x0090_1000]);
        e.mem.set_u32(c, 0x0090_0000);
        e.register_double(COLLECTION, move |_, _| c.into_ret());
        e.register(0x0090_1000, |_, _| Ret::default());
        e.register(SETTING_BASE_DESTRUCT, |_, _| Ret::default());
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x0063_2f90, &[s]);
        let log: Vec<_> = calls(&mut e)
            .into_iter()
            .filter(|c| c.0 != 0x0063_2f90)
            .collect();
        assert_eq!(e.mem.u32(s), SETTING_VTABLE);
        assert_eq!(
            log,
            vec![
                (COLLECTION, vec![]),
                (0x0090_1000, vec![c, s]),
                (SETTING_BASE_DESTRUCT, vec![s]),
            ]
        );
    }

    #[test]
    fn atmosphere_constructor_sets_up_its_four_members() {
        let mut e = Engine::new();
        for a in [SKY_OBJECT_CONSTRUCT, MEMBER_CONSTRUCT, MEMBER_SET] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_3080, &[this]).u32(), this);
        let log: Vec<_> = calls(&mut e)
            .into_iter()
            .filter(|c| c.0 != 0x0063_3080)
            .collect();
        let mut expected = vec![(SKY_OBJECT_CONSTRUCT, vec![this])];
        for offset in [0x8, 0xc, 0x10, 0x14] {
            expected.push((MEMBER_CONSTRUCT, vec![this + offset, 0]));
        }
        for offset in [0x8, 0x14, 0x10, 0xc] {
            expected.push((MEMBER_SET, vec![this + offset, 0]));
        }
        assert_eq!(log, expected);
        assert_eq!(e.mem.u32(this), ATMOSPHERE_VTABLE);
        assert_eq!(e.mem.u8(this + 0x18), 1);
    }
}
