//! `fallout shared/run_005a7400` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a77b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a77b0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6b (0x28 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_69dc`) is `TESRecipeCategory`, so this is the constructor of `TESRecipeCategory`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005a77b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_69dc);
    e.mem.set_u32(base + 0x18, 0x0103_69c8);
    e.call(0x004f_15a0, &args![this, 0x6bu32]);
    e.call(0x005a_7840, &args![this, 0x1u32]);
    this
}

// Translated from 005a7c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a7c50 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6a (0x5c bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_6b2c`) is `TESRecipe`, so this is the constructor of `TESRecipe`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005a7c50(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_6b2c);
    e.mem.set_u32(base + 0x18, 0x0103_6b18);
    e.call(0x0068_0890, &args![base + 0x34]);
    e.call(0x0096_a2d0, &args![base + 0x3c]);
    e.call(0x0096_a2d0, &args![base + 0x44]);
    e.call(0x0096_a2d0, &args![base + 0x4c]);
    e.call(0x004f_15a0, &args![this, 0x6au32]);
    this
}

// Translated from 005a7870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005a78a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005a7870(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005a_78a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005a7d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005a7d40` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005a7d10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005a_7d40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005a7840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bits of `flags` (a byte) in the recipe category's flag byte
/// at +0x24 (`TESRecipeCategory`, its constructor `005a77b0` clears bit 0).
pub fn fn_005a7840(e: &mut Engine, this: Ptr, flags: u8) {
    let value = e.mem.u8(this.addr() + 0x24);
    e.mem.set_u8(this.addr() + 0x24, value & !flags);
}

// Translated from 005a78a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a78a0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `TESRecipeCategory` (the vtables `005a77b0` stores):
/// puts back the vtables, destroys the full name at +0x18 (`00591300`) and
/// the `TESForm` base (`00483630`).
pub fn fn_005a78a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_69dc);
    e.mem.set_u32(base + 0x18, 0x0103_69c8);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005a7910 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a7910 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESRecipeCategory`'s record reader (`RET 4`: the `TESFile`; the engine
/// map has no name): false unless the form type byte is 0x6B; otherwise
/// `TESForm::LoadForm`, `00484ab0(0)` and each chunk: `EDID` (vtable
/// +0x134), `OBND` (vtable +0xE0), `FULL` (+0x18, `00487050`) and `DATA`
/// (one byte at +0x24, `TESForm::LoadData`, then `00483710` on it when the
/// file asks for swapped words). Returns true.
pub fn fn_005a7910(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(0x0047_2660, &args![file]).u8();
    if form_type != 0x6b {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            // OBND
            0x444e_424f => {
                e.vcall(base, 0xe0, &args![file]);
            }
            // DATA
            0x4154_4144 => {
                e.call(0x0048_61f0, &args![this, file, base + 0x24, 1u32]);
                if e.call(0x0040_1680, &args![file]).bool() {
                    e.call(0x0048_3710, &args![base + 0x24]);
                }
            }
            // EDID
            0x4449_4445 => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(0x0047_2890, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            // FULL
            0x4c4c_5546 => {
                let full = if this.is_null() { 0 } else { base + 0x18 };
                e.call(0x0048_7050, &args![full, file]);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 005a7be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a recipe's 8-byte entry (the record reader `005a8110`
/// makes them): `00537e90(0)` on it. Returns `this`.
pub fn fn_005a7be0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0053_7e90, &args![this, 0u32]);
    this
}

// Translated from 005a7d40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a7d40 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `TESRecipe` (the vtables `005a7c50` stores): puts
/// back the vtables, clears the recipe (`005a80a0`), destroys the lists at
/// +0x4C, +0x44 and +0x3C (`0046ffb0`), the conditions at +0x34
/// (`006808b0`), the full name at +0x18 (`00591300`) and the `TESForm`
/// base (`00483630`).
pub fn fn_005a7d40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_6b2c);
    e.mem.set_u32(base + 0x18, 0x0103_6b18);
    e.call(0x005a_80a0, &args![this]);
    for list in [0x4cu32, 0x44, 0x3c] {
        e.call(0x0046_ffb0, &args![base + list]);
    }
    e.call(0x0068_08b0, &args![base + 0x34]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl), the source type descriptor and
/// the targets: `TESLoadScreenType`'s (`01186110`, by its RTTI name) and `TESRecipeCategory`'s
/// (`01186490`, by its RTTI name) as these functions pass them.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE: u32 = 0x0118_3028;
const LOAD_SCREEN_TYPE: u32 = 0x0118_6110;
const CATEGORY_TYPE: u32 = 0x0118_6490;
/// Whether the file's words are swapped (`00401500`, no arguments).
const SWAPPED: u32 = 0x0040_1500;
/// `TESForm::StartForm`, `TESForm::__SaveData`, `TESForm::CloseForm`,
/// `TESForm::CopyAllComponents`, `TESForm::CompareAllComponents`.
const START_FORM: u32 = 0x0048_55a0;
const SAVE_DATA: u32 = 0x0048_5e20;
const CLOSE_FORM: u32 = 0x0048_5680;
const COPY_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_COMPONENTS: u32 = 0x0048_5270;
/// The copy and compare helpers (`00401460`, `_memcmp` `00ec4835`; cdecl:
/// two addresses and a size).
const COPY_BYTES: u32 = 0x0040_1460;
const COMPARE_BYTES: u32 = 0x00ec_4835;

// Translated from 005a7400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the 22 words of `TESLoadScreenType`'s 0x58-byte data at +0x18 (`this`; the engine map
/// has no name): `00401080(word, 0)` (cdecl) on each, in the exe's order
/// +0x28, +0x10, +4, +8, +0x14, +0xC, +0x1C, +0x20, +0x24, +0x38, +0x2C,
/// +0x30, +0x3C, +0x34, +0x44, +0x48, +0x4C, +0x50, +0, +0x18, +0x40,
/// +0x54.
pub fn fn_005a7400(e: &mut Engine, this: Ptr) {
    for offset in [
        0x28u32, 0x10, 4, 8, 0x14, 0xc, 0x1c, 0x20, 0x24, 0x38, 0x2c, 0x30, 0x3c, 0x34, 0x44, 0x48,
        0x4c, 0x50, 0, 0x18, 0x40, 0x54,
    ] {
        e.call(0x0040_1080, &args![this.addr() + offset, 0u32]);
    }
}

// Translated from 005a7630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLoadScreenType`'s record writer (no stack words; the engine map has no
/// name): `TESForm::StartForm`, the 0x58 bytes at +0x18 as
/// `TESForm::__SaveData(+0x18, 0x58)`, swapped by `005a7400` before and
/// after when `00401500` answers true, then `TESForm::CloseForm`.
pub fn fn_005a7630(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    if e.call(SWAPPED, &args![]).bool() {
        fn_005a7400(e, Ptr::new(base + 0x18));
    }
    e.call(SAVE_DATA, &args![this, base + 0x18, 0x58u32]);
    if e.call(SWAPPED, &args![]).bool() {
        fn_005a7400(e, Ptr::new(base + 0x18));
    }
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 005a7690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLoadScreenType`'s copy (`RET 4`: the source form; the engine map has no
/// name): the source through `__RTDynamicCast(source, 0, 01183028,
/// 01186110, 0)`; nothing without it, otherwise the 0x58 bytes at +0x18
/// (`00401460`) and `TESForm::CopyAllComponents` given the uncast source.
pub fn fn_005a7690(e: &mut Engine, this: Ptr, source: u32) {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, LOAD_SCREEN_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(COPY_BYTES, &args![base + 0x18, other + 0x18, 0x58u32]);
    e.call(COPY_COMPONENTS, &args![this, source]);
}

// Translated from 005a76f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLoadScreenType`'s compare (`RET 4`: the other form; true when they differ;
/// the engine map has no name): true when the cast as in `005a7690`
/// fails or `_memcmp` of the 0x58 bytes at +0x18 (the other's through
/// `00500940`) is not 0, otherwise `TESForm::CompareAllComponents`.
pub fn fn_005a76f0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, LOAD_SCREEN_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    let data = e.call(0x0050_0940, &args![other]).u32();
    if e.call(COMPARE_BYTES, &args![base + 0x18, data, 0x58u32])
        .u32()
        != 0
    {
        return true;
    }
    e.call(COMPARE_COMPONENTS, &args![this, other]).bool()
}

// Translated from 005a7770 (decompiled, FalloutNV.exe 1.4.0.525)
/// A 16-bit sum starting at 0 with one term, `00484bf0(word)` (`RET 4`;
/// the engine map has no name): that call's 16-bit result.
pub fn fn_005a7770(e: &mut Engine, this: Ptr, word: u32) -> u16 {
    let sum = 0u16;
    let term = e.call(0x0048_4bf0, &args![this, word]).u16();
    sum.wrapping_add(term)
}

// Translated from 005a7a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRecipeCategory`'s record writer (no stack words; the engine map has
/// no name): `TESForm::StartForm`, `TESFullName::Save` on +0x18, the flag
/// byte at +0x24 as `TESForm::__SaveData(+0x24, 1)`, with `00483710` run on
/// it before and after when `00401500` answers true, then
/// `TESForm::CloseForm`.
pub fn fn_005a7a90(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(0x0048_7010, &args![base + 0x18]);
    if e.call(SWAPPED, &args![]).bool() {
        e.call(0x0048_3710, &args![base + 0x24]);
    }
    e.call(SAVE_DATA, &args![this, base + 0x24, 1u32]);
    if e.call(SWAPPED, &args![]).bool() {
        e.call(0x0048_3710, &args![base + 0x24]);
    }
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 005a7b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRecipeCategory`'s copy (`RET 4`: the source form; the engine map
/// has no name): the source through `__RTDynamicCast(source, 0, 01183028,
/// 01186490, 0)`; nothing without it, otherwise the byte at +0x24
/// (`00401460`) and `TESForm::CopyAllComponents` given the uncast source.
pub fn fn_005a7b00(e: &mut Engine, this: Ptr, source: u32) {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, CATEGORY_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(COPY_BYTES, &args![base + 0x24, other + 0x24, 1u32]);
    e.call(COPY_COMPONENTS, &args![this, source]);
}

// Translated from 005a7b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRecipeCategory`'s compare (`RET 4`: the other form; true when they
/// differ; the engine map has no name): true when the cast as in
/// `005a7b00` fails or `_memcmp` of the byte at +0x24 (the other's through
/// `00430830`) is not 0, otherwise `TESForm::CompareAllComponents`.
pub fn fn_005a7b60(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, CATEGORY_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    let data = e.call(0x0043_0830, &args![other]).u32();
    if e.call(COMPARE_BYTES, &args![base + 0x24, data, 1u32]).u32() != 0 {
        return true;
    }
    e.call(COMPARE_COMPONENTS, &args![this, other]).bool()
}

// Translated from 005a7c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two objects differ (`RET 4`: the other; the engine map has no
/// name): true when `00559450` or `00726070` (both thiscall, no stack
/// words) answers differently for them.
pub fn fn_005a7c00(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let mine = e.call(0x0055_9450, &args![this]).u32();
    if mine != e.call(0x0055_9450, &args![other]).u32() {
        return true;
    }
    let mine = e.call(0x0072_6070, &args![this]).u32();
    mine != e.call(0x0072_6070, &args![other]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a_7400, fn_005a7400(Ptr)),
        entry!(0x005a_7630, fn_005a7630(Ptr)),
        entry!(0x005a_7690, fn_005a7690(Ptr, u32)),
        entry!(0x005a_76f0, fn_005a76f0(Ptr, u32) -> bool),
        entry!(0x005a_7770, fn_005a7770(Ptr, u32) -> u16),
        entry!(0x005a_7a90, fn_005a7a90(Ptr)),
        entry!(0x005a_7b00, fn_005a7b00(Ptr, u32)),
        entry!(0x005a_7b60, fn_005a7b60(Ptr, u32) -> bool),
        entry!(0x005a_7c00, fn_005a7c00(Ptr, u32) -> bool),
        entry!(0x005a_77b0, fn_005a77b0(Ptr) -> Ptr),
        entry!(0x005a_7c50, fn_005a7c50(Ptr) -> Ptr),
        entry!(0x005a_7870, fn_005a7870(Ptr, u32) -> Ptr),
        entry!(0x005a_7d10, fn_005a7d10(Ptr, u32) -> Ptr),
        entry!(0x005a_7840, fn_005a7840(Ptr, u8)),
        entry!(0x005a_78a0, fn_005a78a0(Ptr)),
        entry!(0x005a_7910, fn_005a7910(Ptr, Ptr) -> bool),
        entry!(0x005a_7be0, fn_005a7be0(Ptr) -> Ptr),
        entry!(0x005a_7d40, fn_005a7d40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005a77b0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005a_77b0,
            size: 0x28,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6b)]),
                (0x005a_7840, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[(0x0, 4, 0x0103_69dc), (0x18, 4, 0x0103_69c8)],
        });
    }

    #[test]
    fn fn_005a7c50_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005a_7c50,
            size: 0x5c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0068_0890, &[A::T(0x34)]),
                (0x0096_a2d0, &[A::T(0x3c)]),
                (0x0096_a2d0, &[A::T(0x44)]),
                (0x0096_a2d0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6a)]),
            ],
            stores: &[(0x0, 4, 0x0103_6b2c), (0x18, 4, 0x0103_6b18)],
        });
    }
    #[test]
    fn at_005a7870_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005a_78a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005a_7870, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005a_7870)
                .collect();
            let mut expected = vec![(0x005a_78a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005a7d10_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005a_7d40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005a_7d10, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005a_7d10)
                .collect();
            let mut expected = vec![(0x005a_7d40, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod recipe_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_005a7840_clears_the_flag_bits() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        e.mem.set_u8(this + 0x24, 0xff);
        e.call(0x005a_7840, &args![this, 0x0105u32]);
        assert_eq!(e.mem.u8(this + 0x24), 0xfa);
    }

    #[test]
    fn the_destructors_destroy_their_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        quiet(
            &mut e,
            &[
                0x0059_1300,
                0x0048_3630,
                0x005a_80a0,
                0x0046_ffb0,
                0x0068_08b0,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x005a_78a0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_69dc);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_69c8);
        assert_eq!(calls(&e, 0x0059_1300), vec![vec![this + 0x18]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);

        e.call_log = Some(vec![]);
        e.call(0x005a_7d40, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_6b2c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_6b18);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x005a_7d40)
            .collect();
        assert_eq!(
            order,
            vec![
                (0x005a_80a0, vec![this]),
                (0x0046_ffb0, vec![this + 0x4c]),
                (0x0046_ffb0, vec![this + 0x44]),
                (0x0046_ffb0, vec![this + 0x3c]),
                (0x0068_08b0, vec![this + 0x34]),
                (0x0059_1300, vec![this + 0x18]),
                (0x0048_3630, vec![this]),
            ]
        );
    }

    #[test]
    fn fn_005a7be0_starts_the_entry() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0053_7e90]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_7be0, &args![0x4000u32]).u32(), 0x4000);
        assert_eq!(calls(&e, 0x0053_7e90), vec![vec![0x4000, 0]]);
    }

    fn category(form_type: u32, swap: bool) -> (Engine, u32, u32, bool) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((
            vec![0x4449_4445u32, 0x444e_424f, 0x4c4c_5546, 0x4154_4144],
            0usize,
        )));
        e.register_double(0x0047_2660, move |_, _| ret(form_type));
        let c = chunks.clone();
        e.register_double(0x0047_26b0, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).copied().unwrap_or(0))
        });
        let c = chunks;
        e.register_double(0x0047_26f0, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        e.register(0x0040_1660, |_, _| ret(8));
        if swap {
            e.register(0x0040_1680, |_, _| ret(1));
        } else {
            e.register(0x0040_1680, |_, _| ret(0));
        }
        quiet(
            &mut e,
            &[
                0x7000_00e0,
                0x7000_0134,
                0x0047_2890,
                0x0048_5110,
                0x0048_4ab0,
                0x0048_61f0,
                0x0048_3710,
                0x0048_7050,
            ],
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x005a_7910, &args![this, file]).bool();
        (e, this, file, result)
    }

    #[test]
    fn fn_005a7910_reads_the_category() {
        let (e, this, file, result) = category(0x6b, true);
        assert!(result);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x24, 1]]
        );
        assert_eq!(calls(&e, 0x0048_3710), vec![vec![this + 0x24]]);

        let (e, _, _, result) = category(0x6b, false);
        assert!(result);
        assert!(calls(&e, 0x0048_3710).is_empty());

        let (e, _, _, result) = category(0x6a, false);
        assert!(!result);
        assert!(calls(&e, 0x0048_5110).is_empty());
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_005a7400_swaps_the_words_in_order() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0040_1080]);
        let data = e.mem.alloc(0x58);
        e.call_log = Some(vec![]);
        e.call(0x005a_7400, &args![data]);
        let offsets: Vec<u32> = calls(&e, 0x0040_1080)
            .iter()
            .map(|c| {
                assert_eq!(c[1], 0);
                c[0] - data
            })
            .collect();
        assert_eq!(
            offsets,
            vec![
                0x28, 0x10, 4, 8, 0x14, 0xc, 0x1c, 0x20, 0x24, 0x38, 0x2c, 0x30, 0x3c, 0x34, 0x44,
                0x48, 0x4c, 0x50, 0, 0x18, 0x40, 0x54
            ]
        );
    }

    #[test]
    fn load_screen_type_and_category_writers_swap_only_when_asked() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                START_FORM,
                SAVE_DATA,
                CLOSE_FORM,
                0x0040_1080,
                0x0048_7010,
                0x0048_3710,
            ],
        );
        let this = e.mem.alloc(0x5c);
        for swap in [0u32, 1] {
            e.register_double(SWAPPED, move |_, _| ret(swap));
            e.call_log = Some(vec![]);
            e.call(0x005a_7630, &args![this]);
            assert_eq!(calls(&e, START_FORM), vec![vec![this]]);
            assert_eq!(calls(&e, SAVE_DATA), vec![vec![this, this + 0x18, 0x58]]);
            assert_eq!(calls(&e, 0x0040_1080).len(), 44 * swap as usize);
            assert_eq!(calls(&e, CLOSE_FORM), vec![vec![this]]);

            e.call_log = Some(vec![]);
            e.call(0x005a_7a90, &args![this]);
            assert_eq!(calls(&e, 0x0048_7010), vec![vec![this + 0x18]]);
            assert_eq!(calls(&e, SAVE_DATA), vec![vec![this, this + 0x24, 1]]);
            let swaps = calls(&e, 0x0048_3710);
            assert_eq!(swaps.len(), 2 * swap as usize);
            assert!(swaps.iter().all(|c| c == &vec![this + 0x24]));
            assert_eq!(calls(&e, CLOSE_FORM), vec![vec![this]]);
        }
    }

    #[test]
    fn load_screen_type_and_category_copies() {
        let mut e = Engine::new();
        quiet(&mut e, &[COPY_BYTES, COPY_COMPONENTS]);
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 1 { 0 } else { a[0] + 0x100 })
        });
        let this = e.mem.alloc(0x5c);
        for (addr, target, offset, size) in [
            (0x005a_7690u32, LOAD_SCREEN_TYPE, 0x18u32, 0x58u32),
            (0x005a_7b00, CATEGORY_TYPE, 0x24, 1),
        ] {
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, 0x4000u32]);
            assert_eq!(
                calls(&e, DYNAMIC_CAST),
                vec![vec![0x4000, 0, CAST_SOURCE, target, 0]]
            );
            assert_eq!(
                calls(&e, COPY_BYTES),
                vec![vec![this + offset, 0x4100 + offset, size]]
            );
            assert_eq!(calls(&e, COPY_COMPONENTS), vec![vec![this, 0x4000]]);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, 1u32]);
            assert!(calls(&e, COPY_BYTES).is_empty());
        }
    }

    #[test]
    fn load_screen_type_and_category_compares() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 1 { 0 } else { a[0] + 0x100 })
        });
        e.register(0x0050_0940, |_, a| ret(a[0] + 0x18));
        e.register(0x0043_0830, |_, a| ret(a[0] + 0x24));
        let this = e.mem.alloc(0x5c);
        for (addr, offset, size) in [(0x005a_76f0u32, 0x18u32, 0x58u32), (0x005a_7b60, 0x24, 1)] {
            for (order, components, differs) in [(0u32, 0u32, false), (5, 0, true), (0, 1, true)] {
                e.register_double(COMPARE_BYTES, move |_, _| ret(order));
                e.register_double(COMPARE_COMPONENTS, move |_, _| ret(components));
                e.call_log = Some(vec![]);
                assert_eq!(e.call(addr, &args![this, 0x4000u32]).bool(), differs);
                assert_eq!(
                    calls(&e, COMPARE_BYTES),
                    vec![vec![this + offset, 0x4100 + offset, size]]
                );
            }
            assert!(e.call(addr, &args![this, 1u32]).bool());
        }
    }

    #[test]
    fn fn_005a7770_returns_the_16_bit_term() {
        let mut e = Engine::new();
        e.register(0x0048_4bf0, |_, a| ret(0xffff_0000 | a[1]));
        let this = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_7770, &args![this, 4u32]).u32() & 0xffff, 4);
        assert_eq!(calls(&e, 0x0048_4bf0), vec![vec![this, 4]]);
    }

    #[test]
    fn fn_005a7c00_compares_the_two_answers() {
        let mut e = Engine::new();
        e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        assert!(!e.call(0x005a_7c00, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 1);
        assert!(e.call(0x005a_7c00, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 0);
        e.mem.set_u32(other, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_7c00, &args![this, other]).bool());
        assert!(calls(&e, 0x0072_6070).is_empty());
    }
}
