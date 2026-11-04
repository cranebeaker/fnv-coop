//! `(unplaced)/run_00522020` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00522440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x244 of `this` when `second` is non-zero, else the one at
/// +0x220 (thiscall, `RET 4`; unnamed in the engine map).
pub fn fn_00522440(e: &mut Engine, this: Ptr, second: u8) -> u32 {
    let offset = if second != 0 { 0x244 } else { 0x220 };
    e.mem.u32(this.addr() + offset)
}

// Translated from 00522470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x248 of `this` when `second` is non-zero, else the one at
/// +0x224 (thiscall, `RET 4`; unnamed in the engine map).
pub fn fn_00522470(e: &mut Engine, this: Ptr, second: u8) -> u32 {
    let offset = if second != 0 { 0x248 } else { 0x224 };
    e.mem.u32(this.addr() + offset)
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference) and the RTTI type descriptors `.?AVTESForm@@` and `.?AVTESObjectWEAP@@`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_WEAPON: u32 = 0x0118_3998;
/// `TESForm::CopyAllComponents` (Xbox PDB, `004851b0`, `ECX`, one stack word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
/// `00559450` (`ECX`): the object a pointer holder holds; `004037f0` (`ECX` = a holder,
/// two stack words: the object and 0) sets it.
const HOLDER_OBJECT: u32 = 0x0055_9450;
const HOLDER_SET: u32 = 0x0040_37f0;
/// `00401460` (`cdecl`: destination, source, size): copies bytes.
const COPY_BYTES: u32 = 0x0040_1460;
/// `00401170` (`ECX`): the form type byte at +4. `004f15a0` (`ECX`, one stack word) sets it.
const FORM_TYPE: u32 = 0x0040_1170;
const SET_FORM_TYPE: u32 = 0x004f_15a0;

/// The components [`fn_00522020`] copies through their own vtable slot +8 (`ECX` = this
/// one's component, one stack word: the source's), by offset, in two runs.
const COMPONENTS_FIRST: [u32; 2] = [0x200, 0x1e8];
const COMPONENTS_SECOND: [u32; 7] = [0x270, 0x290, 0x2b0, 0x2d0, 0x2f0, 0x310, 0x330];
/// Getter/setter pairs (the getter on the source, `ECX` only; the setter on `this` with
/// its `EAX`), in the exe's order: the first run after the components, the second after
/// the byte copies and the indexed getters.
const PAIRS_FIRST: [(u32, u32); 3] = [
    (0x0051_f550, 0x0051_f570),
    (0x008d_8ac0, 0x008d_8ae0),
    (0x008d_8b00, 0x0052_1710),
];
const PAIRS_SECOND: [(u32, u32); 9] = [
    (0x008b_ffa0, 0x0052_1650),
    (0x0052_24e0, 0x0052_1690),
    (0x0052_24c0, 0x0052_1670),
    (0x0052_2500, 0x0052_16b0),
    (0x0052_2520, 0x0052_16d0),
    (0x0052_2540, 0x0052_16f0),
    (0x0052_1630, 0x0052_1610),
    (0x008d_8a60, 0x008d_8a80),
    (0x0052_24a0, 0x008d_8aa0),
];

fn copy_pair(e: &mut Engine, this: u32, weapon: u32, (getter, setter): (u32, u32)) {
    let value = e.call(getter, &args![weapon]).u32();
    e.call(setter, &args![this, value]);
}

// Translated from 00522020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP`'s `Copy` slot (the vtable slot at `0102c624`; unnamed in the engine map,
/// whose names for several getters and setters come from folded code; `ECX` and one stack
/// word, `RET 4`). When `source` casts (`__RTDynamicCast` from `TESForm`) to a
/// `TESObjectWEAP`, copies in the exe's order: `TESForm::CopyAllComponents` from `source`;
/// the component at +0x1d0 (its vtable slot +8); the holders at +0x35c and +0x368
/// (`004037f0` with the source's `00559450` object and 0); the components at +0x200 and
/// +0x1e8; three getter/setter pairs; seven indexed values (`004ab500(i)` into
/// `005214d0(i, ..)` for 1 to 7) and three more (`004bd570(i)` into `00521480(i, ..)` for
/// 1, 2, 4); the components at +0x270 to +0x330; the bytes at +0xf4 (0xcc), +0x1c0 (0x10)
/// and +0x370 (0x14) (`00401460`); `005215a0(0)` into `005215d0`, [`fn_00522440`]`(0)` into
/// `005215f0` and [`fn_00522470`]`(0)` into `005c8a30`; nine more pairs; `00522560(0)` into
/// the word at +0x364; and the form type byte (`00401170` into `004f15a0`).
pub fn fn_00522020(e: &mut Engine, this: Ptr, source: Ptr) {
    let weapon = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, TYPE_TES_FORM, TYPE_WEAPON, 0u32],
        )
        .u32();
    if weapon == 0 {
        return;
    }
    let base = this.addr();
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
    e.vcall(base + 0x1d0, 8, &args![weapon + 0x1d0]);
    for offset in [0x35cu32, 0x368] {
        let object = e.call(HOLDER_OBJECT, &args![weapon + offset]).u32();
        e.call(HOLDER_SET, &args![base + offset, object, 0u32]);
    }
    for offset in COMPONENTS_FIRST {
        e.vcall(base + offset, 8, &args![weapon + offset]);
    }
    for pair in PAIRS_FIRST {
        copy_pair(e, base, weapon, pair);
    }
    for index in 1..=7u32 {
        let value = e.call(0x004a_b500, &args![weapon, index]).u32();
        e.call(0x0052_14d0, &args![base, index, value]);
    }
    for index in [1u32, 2, 4] {
        let value = e.call(0x004b_d570, &args![weapon, index]).u32();
        e.call(0x0052_1480, &args![base, index, value]);
    }
    for offset in COMPONENTS_SECOND {
        e.vcall(base + offset, 8, &args![weapon + offset]);
    }
    for (offset, size) in [(0xf4u32, 0xccu32), (0x1c0, 0x10), (0x370, 0x14)] {
        e.call(COPY_BYTES, &args![base + offset, weapon + offset, size]);
    }
    let value = e.call(0x0052_15a0, &args![weapon, 0u32]).u32();
    e.call(0x0052_15d0, &args![base, value]);
    let value = fn_00522440(e, Ptr::new(weapon), 0);
    e.call(0x0052_15f0, &args![base, value]);
    let value = fn_00522470(e, Ptr::new(weapon), 0);
    e.call(0x005c_8a30, &args![base, value]);
    for pair in PAIRS_SECOND {
        copy_pair(e, base, weapon, pair);
    }
    let value = e.call(0x0052_2560, &args![weapon, 0u32]).u32();
    e.mem.set_u32(base + 0x364, value);
    let kind = e.call(FORM_TYPE, &args![weapon]).u32();
    e.call(SET_FORM_TYPE, &args![this, kind]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_2020, fn_00522020(Ptr, Ptr)),
        entry!(0x0052_2440, fn_00522440(Ptr, u8) -> u32),
        entry!(0x0052_2470, fn_00522470(Ptr, u8) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getters_pick_their_word_by_the_byte_argument() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x250);
        for offset in [0x220u32, 0x224, 0x244, 0x248] {
            e.mem.set_u32(this + offset, offset);
        }
        assert_eq!(e.call(0x0052_2440, &args![this, 0u32]).u32(), 0x220);
        assert_eq!(e.call(0x0052_2440, &args![this, 1u32]).u32(), 0x244);
        // Only the low byte of the argument word counts.
        assert_eq!(e.call(0x0052_2440, &args![this, 0x100u32]).u32(), 0x220);
        assert_eq!(e.call(0x0052_2470, &args![this, 0u32]).u32(), 0x224);
        assert_eq!(e.call(0x0052_2470, &args![this, 0x80u32]).u32(), 0x248);
    }
}

#[cfg(test)]
mod copy_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    const COMPONENT_COPY: u32 = 0x0060_0008;

    fn engine(cast: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(
            DYNAMIC_CAST,
            if cast {
                |_, a| (a[0] + 0x1000).into_ret()
            } else {
                |_, _| 0u32.into_ret()
            },
        );
        let mut callees = vec![
            COPY_ALL_COMPONENTS,
            HOLDER_OBJECT,
            HOLDER_SET,
            COPY_BYTES,
            FORM_TYPE,
            SET_FORM_TYPE,
            COMPONENT_COPY,
            0x004a_b500,
            0x0052_14d0,
            0x004b_d570,
            0x0052_1480,
            0x0052_15a0,
            0x0052_15d0,
            0x0052_15f0,
            0x005c_8a30,
            0x0052_2560,
        ];
        for (getter, setter) in PAIRS_FIRST.iter().chain(PAIRS_SECOND.iter()) {
            callees.push(*getter);
            callees.push(*setter);
        }
        for callee in callees {
            // Each answers its own address's low half plus its first word's low byte.
            e.register_double(callee, move |_, a| {
                ((callee & 0xffff) << 8 | (a[0] & 0xff)).into_ret()
            });
        }
        e.put_vtable(0x0070_0000, &[0, 0, COMPONENT_COPY]);
        let this = e.mem.alloc(0x400);
        let source = e.mem.alloc(0x2000);
        for offset in [
            0x1d0u32, 0x200, 0x1e8, 0x270, 0x290, 0x2b0, 0x2d0, 0x2f0, 0x310, 0x330,
        ] {
            e.mem.set_u32(this + offset, 0x0070_0000);
        }
        e.call_log = Some(vec![]);
        (e, this, source)
    }

    #[test]
    fn fn_00522020_copies_every_field_of_a_weapon() {
        let (mut e, this, source) = engine(true);
        let weapon = source + 0x1000;
        e.mem.set_u32(weapon + 0x220, 0x2200);
        e.mem.set_u32(weapon + 0x224, 0x2240);
        e.call(0x0052_2020, &args![this, source]);
        let log: Log = e.call_log.take().unwrap();
        let log: Vec<_> = log.into_iter().filter(|(a, _)| *a != 0x0052_2020).collect();
        assert_eq!(log[1], (COPY_ALL_COMPONENTS, vec![this, source]));
        assert_eq!(log[2], (COMPONENT_COPY, vec![this + 0x1d0, weapon + 0x1d0]));
        assert_eq!(log[3], (HOLDER_OBJECT, vec![weapon + 0x35c]));
        assert_eq!(log[4].0, HOLDER_SET);
        assert_eq!(log[4].1[0], this + 0x35c);
        assert_eq!(log[4].1[2], 0);
        let components: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == COMPONENT_COPY)
            .map(|(_, w)| w[0] - this)
            .collect();
        assert_eq!(
            components,
            vec![0x1d0, 0x200, 0x1e8, 0x270, 0x290, 0x2b0, 0x2d0, 0x2f0, 0x310, 0x330]
        );
        let indexed: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x0052_14d0)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(indexed, vec![1, 2, 3, 4, 5, 6, 7]);
        let indexed: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x0052_1480)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(indexed, vec![1, 2, 4]);
        let bytes: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == COPY_BYTES)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            bytes,
            vec![
                vec![this + 0xf4, weapon + 0xf4, 0xcc],
                vec![this + 0x1c0, weapon + 0x1c0, 0x10],
                vec![this + 0x370, weapon + 0x370, 0x14],
            ]
        );
        assert!(log.contains(&(0x0052_15f0, vec![this, 0x2200])));
        assert!(log.contains(&(0x005c_8a30, vec![this, 0x2240])));
        for (getter, setter) in PAIRS_FIRST.iter().chain(PAIRS_SECOND.iter()) {
            let value = (getter & 0xffff) << 8 | (weapon & 0xff);
            assert!(log.contains(&(*setter, vec![this, value])), "{setter:08x}");
        }
        assert_eq!(e.mem.u32(this + 0x364), 0x2560 << 8 | (weapon & 0xff));
        assert_eq!(
            log.last().unwrap(),
            &(SET_FORM_TYPE, vec![this, 0x1170 << 8 | (weapon & 0xff)])
        );
    }

    #[test]
    fn fn_00522020_ignores_other_forms() {
        let (mut e, this, source) = engine(false);
        e.call(0x0052_2020, &args![this, source]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
