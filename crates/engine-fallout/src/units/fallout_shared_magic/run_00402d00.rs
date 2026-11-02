//! `fallout shared/magic/run_00402d00` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// The `BSStringT<char>` constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// This class's vtable.
const VTABLE: u32 = 0x0101_1d54;

// Translated from 00402d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESFullName` (the engine map gives it no name; in the
/// Xbox PDB `TESFullName` is a `BaseFormComponent` followed by a `BSStringT`
/// at +4, and `TESQuest` constructs it at +0x30): runs the base constructor,
/// sets the vtable and constructs the string at `+4`. Returns `this`.
pub fn fn_00402d00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(STRING_CONSTRUCT, &args![this.byte_add(4)]);
    this
}

// Translated from 00402ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map gives it no name. `thiscall`, no stack arguments: sets the
/// fields of an object at `+0xb8` to -1, `+0xbc` to the byte 0, `+0xc0` to 0,
/// `+0xc4` to the `float` 0.0 and `+0xc8` to 0, then calls `00483710` on it.
/// Returns nothing.
pub fn fn_00402ea0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0xb8, 0xffff_ffff);
    e.mem.set_u8(base + 0xbc, 0);
    e.mem.set_u32(base + 0xc0, 0);
    e.mem.set_f32(base + 0xc4, 0.0);
    e.mem.set_u32(base + 0xc8, 0);
    e.call(0x0048_3710, &args![this]);
}

/// The vtables `fn_00402d50` stores back, by offset.
const DESTRUCTOR_VTABLES: [(u32, u32); 11] = [
    (0x00, 0x0101_1964),
    (0x30, 0x0101_191c),
    (0x3c, 0x0101_1900),
    (0x4c, 0x0101_18dc),
    (0x6c, 0x0101_18bc),
    (0x78, 0x0101_18a8),
    (0x88, 0x0101_1894),
    (0x94, 0x0101_1880),
    (0x9c, 0x0101_186c),
    (0xa4, 0x0101_1858),
    (0xac, 0x0101_1844),
];

// Translated from 00402d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00402d50 - the exception-unwinding frame (FS:[0]) is not translated.
/// Destructor body (the engine map gives it no name; the class is unconfirmed): stores
/// its eleven vtable pointers back, then runs the member destructors in reverse order:
/// `fn_00402ef0` on `this`, `00402c40` on `this + 0xcc`, `0047b520` on `+0xac`,
/// `00475380` on `+0xa4`, `0048eaf0` on `+0x94`, `0047a1a0` on `+0x78`, `00402c40` on
/// `+0x6c`, `0048a490` on `+0x4c` and the base destructor `00402bc0` on `this`.
pub fn fn_00402d50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, vtable) in DESTRUCTOR_VTABLES {
        e.mem.set_u32(base + offset, vtable);
    }
    fn_00402ef0(e, this);
    e.call(0x0040_2c40, &args![base + 0xcc]);
    e.call(0x0047_b520, &args![base + 0xac]);
    e.call(0x0047_5380, &args![base + 0xa4]);
    e.call(0x0048_eaf0, &args![base + 0x94]);
    e.call(0x0047_a1a0, &args![base + 0x78]);
    e.call(0x0040_2c40, &args![base + 0x6c]);
    e.call(0x0048_a490, &args![base + 0x4c]);
    e.call(0x0040_2bc0, &args![this]);
}

// Translated from 00402ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra shows a library-match name): runs `00406340`
/// (`effectitemlist.cpp`) on `this + 0x3c`, then `00483710` on `this`.
pub fn fn_00402ef0(e: &mut Engine, this: Ptr) {
    e.call(0x0040_6340, &args![this.addr() + 0x3c]);
    e.call(0x0048_3710, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00402d00, fn_00402d00(Ptr) -> Ptr),
        entry!(0x00402ea0, fn_00402ea0(Ptr)),
        entry!(0x0040_2d50, fn_00402d50(Ptr)),
        entry!(0x0040_2ef0, fn_00402ef0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_base_then_the_vtable_then_the_string() {
        let mut e = Engine::new();
        e.register(BASE_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let back = e.call(0x0040_2d00, &args![object]).u32();
        assert_eq!(back, object);
        assert_eq!(e.mem.u32(object), VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (BASE_COMPONENT_CONSTRUCT, vec![object]),
                (STRING_CONSTRUCT, vec![object + 4]),
            ]
        );
    }
}

#[cfg(test)]
mod start_value_tests {
    use super::*;

    #[test]
    fn fn_00402ea0_sets_the_start_values_and_calls_00483710() {
        let mut e = Engine::new();
        e.register(0x0048_3710, |_, _| Ret::default());
        let this = e.mem.alloc(0xcc);
        e.mem.write(this, &[0xcd; 0xcc]);
        e.call_log = Some(vec![]);
        e.call(0x0040_2ea0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0048_3710, vec![this])]);
        let mut expected = vec![0xcd_u8; 0xcc];
        expected[0xb8..0xbc].copy_from_slice(&[0xff; 4]);
        expected[0xbc] = 0;
        expected[0xc0..0xcc].fill(0);
        assert_eq!(e.mem.bytes(this, 0xcc), expected);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;

    #[test]
    fn fn_00402d50_restores_vtables_and_destroys_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd0);
        for callee in [
            0x0040_6340u32,
            0x0048_3710,
            0x0040_2c40,
            0x0047_b520,
            0x0047_5380,
            0x0048_eaf0,
            0x0047_a1a0,
            0x0048_a490,
            0x0040_2bc0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0040_2d50, &args![this]);
        let log: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x0040_2d50)
            .collect();
        assert_eq!(
            log,
            vec![
                (0x0040_6340, vec![this + 0x3c]),
                (0x0048_3710, vec![this]),
                (0x0040_2c40, vec![this + 0xcc]),
                (0x0047_b520, vec![this + 0xac]),
                (0x0047_5380, vec![this + 0xa4]),
                (0x0048_eaf0, vec![this + 0x94]),
                (0x0047_a1a0, vec![this + 0x78]),
                (0x0040_2c40, vec![this + 0x6c]),
                (0x0048_a490, vec![this + 0x4c]),
                (0x0040_2bc0, vec![this]),
            ]
        );
        for (offset, vtable) in DESTRUCTOR_VTABLES {
            assert_eq!(e.mem.u32(this + offset), vtable);
        }
    }

    #[test]
    fn fn_00402ef0_destroys_the_list_then_the_base() {
        let mut e = Engine::new();
        e.register(0x0040_6340, |_, _| Ret::default());
        e.register(0x0048_3710, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_2ef0, &args![0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(0x0040_6340, vec![0x703c]), (0x0048_3710, vec![0x7000])]
        );
    }
}
