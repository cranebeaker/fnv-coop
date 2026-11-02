//! `fallout shared/run_0051b5f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051b6e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051b6e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form
/// type 0x25 (0x94 bytes); `ECX` is the object. The engine map gives it no
/// name (a `range-sub` function), so the class is unconfirmed. It runs the
/// base and component constructors by address, stores the vtable pointers,
/// registers the form type (`004f15a0`), copies six floats from globals
/// (`010184f4`, `0101712c`, `0102bc30`, `01016248`, `01018238` into
/// `+0x6c..+0x7c`), sets `+0x84` and `+0x88` to `1.0`, `+0x80` to `0x80`, and
/// copies the words at `011f4980` and `011f4984` into `+0x8c` and `+0x90`.
/// The exception-unwinding frame is not translated.
pub fn fn_0051b6e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f7a0, &args![this]);
    e.call(0x0051_b930, &args![base + 0x30]);
    e.call(0x0051_b970, &args![base + 0x48]);
    e.call(0x0047_5350, &args![base + 0x54]);
    e.mem.set_u32(base, 0x0102_bc94);
    e.mem.set_u32(base + 0x30, 0x0102_bc6c);
    e.mem.set_u32(base + 0x48, 0x0102_bc4c);
    e.mem.set_u32(base + 0x54, 0x0102_bc38);
    e.call(0x0047_4790, &args![base + 0x5c, 0u32, 1u32]);
    e.call(0x0068_15c0, &args![base + 0x8c]);
    e.call(0x004f_15a0, &args![this, 0x25u32]);
    for (offset, global) in [
        (0x6c, 0x0101_84f4),
        (0x70, 0x0101_712c),
        (0x74, 0x0102_bc30),
        (0x78, 0x0101_6248),
        (0x7c, 0x0101_8238),
    ] {
        let value = e.global::<f32>(global);
        e.mem.set_f32(base + offset, value);
    }
    e.mem.set_f32(base + 0x84, 1.0);
    e.mem.set_f32(base + 0x88, 1.0);
    e.mem.set_u32(base + 0x80, 0x80);
    let first = e.global::<u32>(0x011f_4980);
    e.mem.set_u32(base + 0x8c, first);
    let second = e.global::<u32>(0x011f_4984);
    e.mem.set_u32(base + 0x90, second);
    this
}

// Translated from 0051b990 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00402c40`, which forwards `this` to `0048e2e0`
/// (a destructor with an exception frame), and returns nothing. Parameters: one word,
/// `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `00402c40` ends right after that call and `0048e2e0` ends with its frame unlink; neither
/// loads a result and Ghidra types both void.
pub fn fn_0051b990(e: &mut Engine, this: Ptr) {
    e.call(0x0040_2c40, &args![this]);
}

// Translated from 0051b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the component at `+0x30` of the form `0051b6e0` builds
/// (unnamed in the engine map, class unconfirmed). `ECX` is the object: runs the
/// base constructor `00488f50`, stores the virtual table `0102be70` and returns
/// `this`.
pub fn fn_0051b930(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0048_8f50, &args![this]);
    e.mem.set_u32(this.addr(), 0x0102_be70);
    this
}

// Translated from 0051b970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the component at `+0x48` of the form `0051b6e0` builds
/// (unnamed in the engine map, class unconfirmed). `ECX` is the object: runs the
/// base constructor `0051b9b0`, stores the virtual table `0102be94` and returns
/// `this`.
pub fn fn_0051b970(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0051_b9b0, &args![this]);
    e.mem.set_u32(this.addr(), 0x0102_be94);
    this
}

// Translated from 0051b860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall` getter: the `float` at `+0x70` of `this`, returned
/// in `ST0` (`FLD dword`, so a signalling NaN comes out quiet).
pub fn fn_0051b860(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x70)))
}

/// `004013e0` (`ECX`): whether form flag 8 is set (`AL`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `00484ab0` (`ECX`, one stack word): sets or clears form flag 8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `BGSDestructibleObjectForm::InitItem` (engine map, `00478be0`, `ECX`, one stack word: the
/// owning form).
const DESTRUCTIBLE_INIT_ITEM: u32 = 0x0047_8be0;
/// `00658930` (`ECX`): the 16-bit count at +0xa, zero-extended (here the seed array's size).
const ARRAY_COUNT_00658930: u32 = 0x0065_8930;
/// `00877a30` (`ECX` = an `NiTArray`, one stack word): the address of element `index`.
const ARRAY_ELEMENT_00877A30: u32 = 0x0087_7a30;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 0051b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: a constructor (`ECX`, no stack words) that runs the base constructor
/// `0048e270` on `this`, stores the vtable `0102beb4` (RTTI `.?AVTESTexture1024@@`) and
/// answers `this`.
pub fn fn_0051b9b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0048_e270, &args![this]);
    e.mem.set_u32(this.addr(), 0x0102_beb4);
    this
}

// Translated from 0051b840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, called from many classes (a folded setter): `ECX` and one stack float
/// (`RET 4`), stored at `+0x6c` (`FLD`/`FSTP`, so a signalling NaN is stored quiet).
pub fn fn_0051b840(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_f32(this.addr().wrapping_add(0x6c), x87_quiet(value));
}

// Translated from 0051b8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, called from many classes (a folded setter): `ECX` and one stack float
/// (`RET 4`), stored at `+0x84` (`FLD`/`FSTP`, so a signalling NaN is stored quiet).
pub fn fn_0051b8c0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_f32(this.addr().wrapping_add(0x84), x87_quiet(value));
}

// Translated from 0051b8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, called from many classes (a folded getter): the `float` at `+0x88`
/// of `this` (`ECX`), returned in `ST0` (`FLD dword`, so a signalling NaN comes out quiet).
pub fn fn_0051b8e0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x88)))
}

// Translated from 0051b950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: hands `this` (`ECX`) to `00489070` (the `TESModelTree` component's
/// destructor in `0051b9d0`) and returns nothing.
pub fn fn_0051b950(e: &mut Engine, this: Ptr) {
    e.call(0x0048_9070, &args![this]);
}

// Translated from 0051b9d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051b9d0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef4aa4, state variable) is not translated
/// The `TESObjectTREE` destructor (unnamed in the map; `ECX`, no stack words): stores the
/// form's four vtables (`0102bc94` at +0, `0102bc6c` at +0x30, `0102bc4c` at +0x48,
/// `0102bc38` at +0x54, all RTTI `.?AVTESObjectTREE@@`), then destroys the members and bases
/// last first: `00483710` on `this`, `00473f00` on the seed array (+0x5c), `00475380` on
/// +0x54, [`fn_0051b990`] on +0x48, [`fn_0051b950`] on +0x30, and `0050f7f0` on `this`.
pub fn fn_0051b9d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_bc94);
    e.mem.set_u32(base + 0x30, 0x0102_bc6c);
    e.mem.set_u32(base + 0x48, 0x0102_bc4c);
    e.mem.set_u32(base + 0x54, 0x0102_bc38);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0047_3f00, &args![base + 0x5c]);
    e.call(0x0047_5380, &args![base + 0x54]);
    fn_0051b990(e, Ptr::new(base + 0x48));
    fn_0051b950(e, Ptr::new(base + 0x30));
    e.call(0x0050_f7f0, &args![this]);
}

// Translated from 0051b900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESObjectTREE` scalar deleting destructor (its vtable `0102bc94`, slot +0x10; `ECX`
/// and one stack word, `RET 4`): the destructor [`fn_0051b9d0`], then `operator delete` of
/// `this` when bit 0 of `flags` is set; answers `this`.
pub fn fn_0051b900(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0051b9d0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0051bb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectTREE`'s vtable slot +0x88, the `InitItem` slot of the Xbox PDB's `TESForm`
/// (`ECX`): unless form flag 8 is set (`004013e0`), the destructible component (+0x54)
/// resolves (`BGSDestructibleObjectForm::InitItem` with the tree) and form flag 8 is set
/// (`00484ab0` with 1).
pub fn fn_0051bb80(e: &mut Engine, this: Ptr) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    e.call(DESTRUCTIBLE_INIT_ITEM, &args![this.addr() + 0x54, this]);
    e.call(SET_FORM_FLAG_8, &args![this, 1u32]);
}

// Translated from 0051ba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectTREE`'s vtable slot +0x184 (`ECX`, one stack word, `RET 4`; its body matches the
/// Xbox PDB's `GetIndexForSeed`, unconfirmed): the first index of the seed array (+0x5c, its
/// count re-read through `00658930` each step) whose element is `seed`, as a byte; 0xff when
/// none is.
pub fn fn_0051ba90(e: &mut Engine, this: Ptr, seed: u32) -> u8 {
    let seeds = this.addr() + 0x5c;
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT_00658930, &args![seeds]).u32() {
        let element = e.call(ARRAY_ELEMENT_00877A30, &args![seeds, index]).u32();
        if e.mem.u32(element) == seed {
            return index as u8;
        }
        index += 1;
    }
    0xff
}

// Translated from 0051baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectTREE`'s vtable slot +0x188 (`ECX`, one stack word, `RET 4`; its body matches the
/// Xbox PDB's `GetSeedAtIndex`, unconfirmed): for index 0xff or an empty seed array (+0x5c,
/// `00658930`), the answer of vtable slot +0x190; otherwise the seed at the index (the index
/// taken modulo the count when it is not below it, as a byte), read through `00877a30`.
pub fn fn_0051baf0(e: &mut Engine, this: Ptr, index: u8) -> u32 {
    let seeds = this.addr() + 0x5c;
    if index == 0xff || e.call(ARRAY_COUNT_00658930, &args![seeds]).u32() == 0 {
        return e.vcall(this.addr(), 0x190, &args![]).u32();
    }
    let mut index = index;
    if u32::from(index) >= e.call(ARRAY_COUNT_00658930, &args![seeds]).u32() {
        let count = e.call(ARRAY_COUNT_00658930, &args![seeds]).u32();
        index = (u32::from(index) % count) as u8;
    }
    let element = e
        .call(ARRAY_ELEMENT_00877A30, &args![seeds, u32::from(index)])
        .u32();
    e.mem.u32(element)
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) from `.?AVTESForm@@`
/// (`01183028`) to `.?AVTESObjectSTAT@@` (`01186568`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_STAT: u32 = 0x0118_6568;
/// `TESForm::CopyAllComponents` and `CompareAllComponents` (`004851b0`,
/// `00485270`, engine map; `thiscall`, one word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `_ftol2` (`00ec62c0`): the value in `ST0` truncated to an integer.
const FTOL2: u32 = 0x00ec_62c0;

/// `form` cast to `TESObjectSTAT`, or 0.
fn cast_to_stat(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_STAT, 0u32],
    )
    .u32()
}

// Translated from 0051b5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`; a copy): when `source` casts
/// to `TESObjectSTAT`, copies the word at +0x54 from the cast form, runs
/// `TESForm::CopyAllComponents` with the cast form and copies the byte at
/// +0x50.
pub fn fn_0051b5f0(e: &mut Engine, this: Ptr, source: u32) {
    let other = cast_to_stat(e, source);
    if other == 0 {
        return;
    }
    let at = this.addr();
    let word = e.mem.u32(other.wrapping_add(0x54));
    e.mem.set_u32(at + 0x54, word);
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
    let byte = e.mem.u8(other.wrapping_add(0x50));
    e.mem.set_u8(at + 0x50, byte);
}

// Translated from 0051b650 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`): whether the form differs
/// from `form`: true when it does not cast to `TESObjectSTAT`, when
/// `TESForm::CompareAllComponents` answers true, or when the bytes at +0x50
/// or the words at +0x54 differ.
pub fn fn_0051b650(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_to_stat(e, form);
    if other == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool() {
        return true;
    }
    let at = this.addr();
    if e.mem.u8(at + 0x50) as i8 != e.mem.u8(other.wrapping_add(0x50)) as i8 {
        return true;
    }
    e.mem.u32(at + 0x54) != e.mem.u32(other.wrapping_add(0x54))
}

// Translated from 0051b6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to the
/// deleting destructor `0051b1d0` (`run_0051af50.rs`), passing its stack
/// word on; returns what it returns.
pub fn fn_0051b6d0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    let base = this.addr().wrapping_sub(0x30);
    e.call(0x0051_b1d0, &args![base, flags]).u32()
}

// Translated from 0051b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`): answers 0x400.
pub fn fn_0051b830(_e: &mut Engine, _this: Ptr) -> u32 {
    0x400
}

// Translated from 0051b880 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`): the integer at +0x80 loaded into `ST0`
/// (`FILD`, exact).
pub fn fn_0051b880(e: &mut Engine, this: Ptr) -> f64 {
    f64::from(e.mem.u32(this.addr() + 0x80) as i32)
}

// Translated from 0051b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one `float`, `RET 4`): stores `value`
/// truncated by `_ftol2` (`FLD`, then `00ec62c0`) at +0x80.
pub fn fn_0051b8a0(e: &mut Engine, this: Ptr, value: f32) {
    let loaded = x87_load32(value.to_bits());
    let whole = e.call(FTOL2, &args![loaded]).u32();
    e.mem.set_u32(this.addr() + 0x80, whole);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_b9b0, fn_0051b9b0(Ptr) -> Ptr),
        entry!(0x0051_b840, fn_0051b840(Ptr, f32)),
        entry!(0x0051_b8c0, fn_0051b8c0(Ptr, f32)),
        entry!(0x0051_b8e0, fn_0051b8e0(Ptr) -> f32),
        entry!(0x0051_b950, fn_0051b950(Ptr)),
        entry!(0x0051_b9d0, fn_0051b9d0(Ptr)),
        entry!(0x0051_b900, fn_0051b900(Ptr, u32) -> Ptr),
        entry!(0x0051_bb80, fn_0051bb80(Ptr)),
        entry!(0x0051_ba90, fn_0051ba90(Ptr, u32) -> u8),
        entry!(0x0051_baf0, fn_0051baf0(Ptr, u8) -> u32),
        entry!(0x0051_b860, fn_0051b860(Ptr) -> f32),
        entry!(0x0051_b990, fn_0051b990(Ptr)),
        entry!(0x0051_b6e0, fn_0051b6e0(Ptr) -> Ptr),
        entry!(0x0051_b930, fn_0051b930(Ptr) -> Ptr),
        entry!(0x0051_b970, fn_0051b970(Ptr) -> Ptr),
        entry!(0x0051_b5f0, fn_0051b5f0(Ptr, u32)),
        entry!(0x0051_b650, fn_0051b650(Ptr, u32) -> bool),
        entry!(0x0051_b6d0, fn_0051b6d0(Ptr, u32) -> u32),
        entry!(0x0051_b830, fn_0051b830(Ptr) -> u32),
        entry!(0x0051_b880, fn_0051b880(Ptr) -> f64),
        entry!(0x0051_b8a0, fn_0051b8a0(Ptr, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0051b6e0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_b6e0,
            size: 0x94,
            globals: &[
                (0x0101_84f4, 0x1000000),
                (0x0101_712c, 0x1000001),
                (0x0102_bc30, 0x1000002),
                (0x0101_6248, 0x1000003),
                (0x0101_8238, 0x1000004),
                (0x011f_4980, 0x1000005),
                (0x011f_4984, 0x1000006),
            ],
            returns: &[],
            calls: &[
                (0x0050_f7a0, &[A::T(0x0)]),
                (0x0051_b930, &[A::T(0x30)]),
                (0x0051_b970, &[A::T(0x48)]),
                (0x0047_5350, &[A::T(0x54)]),
                (0x0047_4790, &[A::T(0x5c), A::V(0x0), A::V(0x1)]),
                (0x0068_15c0, &[A::T(0x8c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x25)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_bc94),
                (0x30, 4, 0x0102_bc6c),
                (0x48, 4, 0x0102_bc4c),
                (0x54, 4, 0x0102_bc38),
                (0x6c, 4, 0x1000000),
                (0x70, 4, 0x1000001),
                (0x74, 4, 0x1000002),
                (0x78, 4, 0x1000003),
                (0x7c, 4, 0x1000004),
                (0x84, 4, 0x3f80_0000),
                (0x88, 4, 0x3f80_0000),
                (0x80, 4, 0x80),
                (0x8c, 4, 0x1000005),
                (0x90, 4, 0x1000006),
            ],
        });
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0051b990_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0040_2c40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_b990, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0040_2c40), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod component_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0051b930_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_b930,
            size: 0x18,
            globals: &[],
            returns: &[],
            calls: &[(0x0048_8f50, &[A::T(0)])],
            stores: &[(0, 4, 0x0102_be70)],
        });
    }

    #[test]
    fn fn_0051b970_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_b970,
            size: 0x0c,
            globals: &[],
            returns: &[],
            calls: &[(0x0051_b9b0, &[A::T(0)])],
            stores: &[(0, 4, 0x0102_be94)],
        });
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_0051b860_answers_the_float_at_offset_0x70() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x6c, 1.0);
        e.mem.set_f32(this + 0x70, -3.5);
        e.mem.set_f32(this + 0x74, 2.0);
        assert_eq!(e.call(0x0051_b860, &args![this]).f32(), -3.5);
        // A signalling NaN is returned quiet.
        e.mem.set_u32(this + 0x70, 0x7f80_0001);
        assert_eq!(
            e.call(0x0051_b860, &args![this]).f32().to_bits(),
            0x7fc0_0001
        );
    }
}

#[cfg(test)]
mod tree_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_0051b9b0_runs_the_base_then_stores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.register(0x0048_e270, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_b9b0, &args![this]).u32(), this);
        assert_eq!(calls(&e, 0x0048_e270), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x0102_beb4);
    }

    #[test]
    fn fn_0051b840_0051b8c0_0051b8e0_store_and_read_floats() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        e.call(0x0051_b840, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this + 0x6c), 2.5);
        e.call(0x0051_b8c0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x84), 0x7fc0_0001);
        e.call(0x0051_b840, &args![this, 0xff80_0002u32]);
        assert_eq!(e.mem.u32(this + 0x6c), 0xffc0_0002);
        e.mem.set_f32(this + 0x88, -7.25);
        assert_eq!(e.call(0x0051_b8e0, &args![this]).f32(), -7.25);
        e.mem.set_u32(this + 0x88, 0x7f80_0003);
        assert_eq!(
            e.call(0x0051_b8e0, &args![this]).f32().to_bits(),
            0x7fc0_0003
        );
        assert_eq!(e.mem.u32(this + 0x84), 0x7fc0_0001);
    }

    #[test]
    fn fn_0051b950_forwards_this() {
        let mut e = Engine::new();
        e.register(0x0048_9070, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_b950, &args![0x5000u32]);
        assert_eq!(calls(&e, 0x0048_9070), vec![vec![0x5000]]);
    }

    fn destructor_rig() -> (Engine, u32) {
        let mut e = Engine::new();
        for addr in [
            0x0048_3710,
            0x0047_3f00,
            0x0047_5380,
            0x0040_2c40,
            0x0048_9070,
            0x0050_f7f0,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x94);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_0051b9d0_destroys_last_first() {
        let (mut e, this) = destructor_rig();
        e.call(0x0051_b9d0, &args![this]);
        let order: Vec<(u32, u32)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| (c.0, c.1[0]))
            .collect();
        assert_eq!(
            order,
            vec![
                (0x0048_3710, this),
                (0x0047_3f00, this + 0x5c),
                (0x0047_5380, this + 0x54),
                (0x0040_2c40, this + 0x48),
                (0x0048_9070, this + 0x30),
                (0x0050_f7f0, this),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_bc94);
        assert_eq!(e.mem.u32(this + 0x30), 0x0102_bc6c);
        assert_eq!(e.mem.u32(this + 0x48), 0x0102_bc4c);
        assert_eq!(e.mem.u32(this + 0x54), 0x0102_bc38);
    }

    #[test]
    fn fn_0051b900_deletes_only_on_bit_0() {
        let (mut e, this) = destructor_rig();
        assert_eq!(e.call(0x0051_b900, &args![this, 2u32]).u32(), this);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0051_b900, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0050_f7f0).len(), 2);
    }

    #[test]
    fn fn_0051bb80_resolves_once() {
        let mut e = Engine::new();
        e.register(FORM_FLAG_8_SET, |_, _| false.into_ret());
        e.register(DESTRUCTIBLE_INIT_ITEM, |_, _| Ret::default());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_bb80, &args![0x5000u32]);
        assert_eq!(
            calls(&e, DESTRUCTIBLE_INIT_ITEM),
            vec![vec![0x5054, 0x5000]]
        );
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![0x5000, 1]]);
        e.register(FORM_FLAG_8_SET, |_, _| true.into_ret());
        e.call(0x0051_bb80, &args![0x5000u32]);
        assert_eq!(calls(&e, DESTRUCTIBLE_INIT_ITEM).len(), 1);
    }

    /// A tree whose seed array (+0x5c: base pointer at +0x5c + 4, count at +0x5c + 0xa) holds
    /// `seeds`; vtable slot +0x190 answers 0x777.
    fn seed_rig(seeds: &[u32]) -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x94);
        let base = e.mem.alloc(4 * seeds.len() as u32 + 4);
        for (i, seed) in seeds.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *seed);
        }
        e.mem.set_u32(this + 0x60, base);
        e.mem.set_u16(this + 0x66, seeds.len() as u16);
        e.register(ARRAY_COUNT_00658930, |e, a| {
            u32::from(e.mem.u16(a[0] + 0xa)).into_ret()
        });
        e.register(ARRAY_ELEMENT_00877A30, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 4).into_ret()
        });
        e.put_vtable(0x0f00_9000, &[0; 0x65]);
        e.mem.set_u32(0x0f00_9190, 0x0f00_9400);
        e.register(0x0f00_9400, |_, _| 0x777u32.into_ret());
        e.mem.set_u32(this, 0x0f00_9000);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_0051ba90_finds_the_first_index_of_a_seed() {
        let (mut e, this) = seed_rig(&[10, 20, 30, 20]);
        assert_eq!(e.call(0x0051_ba90, &args![this, 20u32]).u32() & 0xff, 1);
        assert_eq!(e.call(0x0051_ba90, &args![this, 30u32]).u32() & 0xff, 2);
        assert_eq!(e.call(0x0051_ba90, &args![this, 99u32]).u32() & 0xff, 0xff);
    }

    #[test]
    fn fn_0051baf0_answers_the_seed_at_an_index() {
        let (mut e, this) = seed_rig(&[10, 20, 30]);
        assert_eq!(e.call(0x0051_baf0, &args![this, 2u32]).u32(), 30);
        // 7 % 3 = 1.
        assert_eq!(e.call(0x0051_baf0, &args![this, 7u32]).u32(), 20);
        assert_eq!(e.call(0x0051_baf0, &args![this, 0xffu32]).u32(), 0x777);
        let (mut e, this) = seed_rig(&[]);
        assert_eq!(e.call(0x0051_baf0, &args![this, 0u32]).u32(), 0x777);
        assert!(calls(&e, ARRAY_ELEMENT_00877A30).is_empty());
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    fn calls_to(e: &mut Engine, targets: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect()
    }

    fn cast(e: &mut Engine) {
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_TES_OBJECT_STAT, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn fn_0051b5f0_copies_from_a_static() {
        let mut e = Engine::new();
        cast(&mut e);
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        e.mem.set_u32(other + 0x54, 0x5454);
        e.mem.set_u8(other + 0x50, 0x50);
        e.mem.set_u32(other + 0xc, 1);
        e.call(0x0051_b5f0, &[this, other]);
        assert_eq!(e.mem.u32(this + 0x54), 0);
        e.mem.set_u32(other + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0051_b5f0, &[this, other]);
        assert_eq!(
            calls_to(&mut e, &[COPY_ALL_COMPONENTS]),
            vec![(COPY_ALL_COMPONENTS, vec![this, other])]
        );
        assert_eq!(e.mem.u32(this + 0x54), 0x5454);
        assert_eq!(e.mem.u8(this + 0x50), 0x50);
    }

    #[test]
    fn fn_0051b650_compares_byte_and_word() {
        let mut e = Engine::new();
        cast(&mut e);
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        e.mem.set_u32(other + 0xc, 1);
        assert!(e.call(0x0051_b650, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 0);
        assert!(!e.call(0x0051_b650, &[this, other]).bool());
        e.mem.set_u8(other + 0x50, 0x80);
        assert!(e.call(0x0051_b650, &[this, other]).bool());
        e.mem.set_u8(this + 0x50, 0x80);
        e.mem.set_u32(other + 0x54, 1);
        assert!(e.call(0x0051_b650, &[this, other]).bool());
        e.mem.set_u32(other + 0x54, 0);
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0051_b650, &[this, other]).bool());
    }

    #[test]
    fn fn_0051b6d0_moves_this_back_to_the_destructor() {
        let mut e = Engine::new();
        e.register(0x0051_b1d0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_b6d0, &[0x2030, 1]).u32(), 0x2000);
        assert_eq!(
            calls_to(&mut e, &[0x0051_b1d0]),
            vec![(0x0051_b1d0, vec![0x2000, 1])]
        );
    }

    #[test]
    fn fn_0051b830_b880_b8a0_the_count() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        assert_eq!(e.call(0x0051_b830, &[this]).u32(), 0x400);
        e.mem.set_u32(this + 0x80, 0xffff_fffe);
        assert_eq!(e.call(0x0051_b880, &[this]).f64(), -2.0);
        e.mem.set_u32(this + 0x80, 0x7fff_ffff);
        assert_eq!(e.call(0x0051_b880, &[this]).f64(), 2_147_483_647.0);
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
        e.call(0x0051_b8a0, &[this, (-3.75f32).to_bits()]);
        assert_eq!(e.mem.u32(this + 0x80), (-3i32) as u32);
    }
}
