//! `fallout shared/run_0058db40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058db40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058db40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5e (0x78 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_2f6c`) is `BGSImpactData`, so this is the constructor of `BGSImpactData`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0058db40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_2f6c);
    e.mem.set_u32(base + 0x18, 0x0103_2f44);
    e.call(0x0059_3240, &args![base + 0x54]);
    e.call(0x004f_15a0, &args![this, 0x5eu32]);
    e.call(0x0058_dc90, &args![this]);
    this
}

// Translated from 0058dbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0058dc10` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058dbe0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_dc10, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
/// The record's format version (`00403570`, a `u16`).
const FILE_VERSION: u32 = 0x0040_3570;
/// `memset` (`00403d30`, cdecl, three words).
const MEMSET: u32 = 0x0040_3d30;
/// `TESForm::LoadData` (`004861f0`: file, buffer, size).
const FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// Swaps the words of the impact data's `Data` (`0058e3a0`, on +0x30).
const SWAP_DATA: u32 = 0x0058_e3a0;
/// The decal data at +0x54 (`DecalData`, Xbox PDB `DData`): `006815c0`
/// answers the address its fields are written at.
const DECAL_DATA: u32 = 0x54;
const DECAL_DATA_FIELDS: u32 = 0x0068_15c0;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_DODT: u32 = 0x5444_4f44;
const CHUNK_DNAM: u32 = 0x4d41_4e44;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_NAM1: u32 = 0x314d_414e;

// Translated from 0058dc10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058dc10 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `BGSImpactData` (the vtables `0058db40` stores):
/// puts back the vtables, destroys the decal data at +0x54 (`00483710`),
/// the model at +0x18 (`00489070`) and the `TESForm` base (`00483630`).
pub fn fn_0058dc10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_2f6c);
    e.mem.set_u32(base + 0x18, 0x0103_2f44);
    e.call(0x0048_3710, &args![base + DECAL_DATA]);
    e.call(0x0048_9070, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0058dc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSImpactData`'s data defaults (Xbox PDB `IMPACT_DATA_DATA`, PC +0x30):
/// cleared, then the effect duration from `0101622c`, orientation 2, the
/// angle threshold and placement radius from `0101e580` and `0101e57c`,
/// sound level 1, and the texture set and both sounds null.
pub fn fn_0058dc90(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(MEMSET, &args![base + 0x30, 0u32, 0x18u32]);
    let duration = e.global::<f32>(0x0101_622c);
    e.mem.set_f32(base + 0x30, x87_quiet(duration));
    e.mem.set_u32(base + 0x34, 2);
    let angle = e.global::<f32>(0x0101_e580);
    e.mem.set_f32(base + 0x38, x87_quiet(angle));
    let radius = e.global::<f32>(0x0101_e57c);
    e.mem.set_f32(base + 0x3c, x87_quiet(radius));
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x4c, 0);
    e.mem.set_u32(base + 0x50, 0);
    e.mem.set_u32(base + 0x40, 1);
}

// Translated from 0058de80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the impact data's flag byte (`Data.cFlags`, +0x44) is
/// set.
pub fn fn_0058de80(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x44) & 1 != 0
}

// Translated from 0058dea0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058dea0 - the /GS stack cookie check (00ec408c) is not translated.
/// `BGSImpactData`'s record reader (`RET 4`: the `TESFile`; the engine map
/// has no name): false unless the form type (all of `EAX`) is 0x5E;
/// otherwise `TESForm::LoadForm`, the data's words swapped first when the
/// file asks (`0058e3a0`), and each chunk: `EDID` (vtable +0x134), `OBND`
/// (vtable +0xE0), `MODL`/`MODT` (`TESModel::LoadModelChunk`), `DATA`
/// (format versions below 10: a 0x38-byte record swapped by `0058e250` and
/// spread into the data and the decal data; later: the 0x18 bytes in
/// place), `DODT` (the decal data, `00593450`), `DNAM` (`00673400`),
/// `SNAM` (`00589f50`) and `NAM1` (`0058e430`) with their form ids.
/// Returns true.
pub fn fn_0058dea0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(FILE_FORM_TYPE, &args![file]).u32() != 0x5e {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
        e.call(SWAP_DATA, &args![base + 0x30]);
    }
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_MODL | CHUNK_MODT => {
                let model = if this.is_null() { 0 } else { base + 0x18 };
                e.call(0x0048_92d0, &args![model, file]);
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_DATA => {
                let version = e.call(FILE_VERSION, &args![file]).u16();
                if i32::from(version) < 10 {
                    impact_data_load_old(e, this, file);
                } else {
                    e.call(MEMSET, &args![base + 0x30, 0u32, 0x18u32]);
                    e.call(FORM_LOAD_DATA, &args![this, file, base + 0x30, 0x18u32]);
                    if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                        e.call(SWAP_DATA, &args![base + 0x30]);
                    }
                }
            }
            CHUNK_DODT => {
                e.call(0x0059_3450, &args![base + DECAL_DATA, file]);
            }
            CHUNK_DNAM => {
                let id = read_form_id(e, file);
                e.call(0x0067_3400, &args![this, id]);
            }
            CHUNK_SNAM => {
                let id = read_form_id(e, file);
                e.call(0x0058_9f50, &args![this, id]);
            }
            CHUNK_NAM1 => {
                let id = read_form_id(e, file);
                fn_0058e430(e, this, id);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// A form id chunk read into a zeroed local (`004727f0`).
fn read_form_id(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(FILE_CHUNK_DATA, &args![file, slot]);
        e.mem.u32(slot.addr())
    })
}

/// `0058dea0`'s `DATA` before format version 10: the 0x38-byte record (on
/// the stack) spread into the data and the decal data. Floats go through
/// FLD/FSTP.
fn impact_data_load_old(e: &mut Engine, this: Ptr, file: Ptr) {
    let base = this.addr();
    let record = e.with_stack(0x38, |e, record| {
        e.call(MEMSET, &args![record, 0u32, 0x38u32]);
        e.call(FORM_LOAD_DATA, &args![this, file, record, 0x38u32]);
        if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
            e.call(0x0058_e250, &args![record]);
        }
        e.mem.bytes(record.addr(), 0x38)
    });
    let word = |at: usize| {
        u32::from_le_bytes([record[at], record[at + 1], record[at + 2], record[at + 3]])
    };
    let float = |at: usize| f32::from_bits(x87_quiet_bits(word(at)));
    e.call(MEMSET, &args![base + 0x30, 0u32, 0x18u32]);
    e.mem.set_f32(base + 0x30, float(0x04));
    e.mem.set_u32(base + 0x34, word(0x0c));
    e.mem.set_f32(base + 0x38, float(0x28));
    e.mem.set_f32(base + 0x3c, float(0x2c));
    e.mem.set_u32(base + 0x40, word(0x34));
    let decal = base + DECAL_DATA;
    for (field, at) in [
        (0x00u32, 0x00usize),
        (0x04, 0x14),
        (0x08, 0x1c),
        (0x0c, 0x20),
        (0x10, 0x18),
        (0x14, 0x24),
        (0x18, 0x08),
    ] {
        let fields = e.call(DECAL_DATA_FIELDS, &args![decal]).u32();
        e.mem.set_f32(fields + field, float(at));
    }
    let fields = e.call(DECAL_DATA_FIELDS, &args![decal]).u32();
    e.mem.set_u8(fields + 0x1c, record[0x11]);
    let fields = e.call(DECAL_DATA_FIELDS, &args![decal]).u32();
    e.mem.set_u8(fields + 0x1d, record[0x10]);
    let fields = e.call(DECAL_DATA_FIELDS, &args![decal]).u32();
    e.mem.set_u32(fields + 0x20, word(0x30));
}

// Translated from 0058e430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the impact data's second sound (`pSound2`, Xbox PDB, PC +0x50).
pub fn fn_0058e430(e: &mut Engine, this: Ptr, sound: u32) {
    e.mem.set_u32(this.addr() + 0x50, sound);
}

/// `__RTDynamicCast` (`00ec43fb`) from `.?AVTESForm@@` (`01183028`) to
/// `.?AVBGSImpactData@@` (`011862fc`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_IMPACT_DATA: u32 = 0x0118_62fc;
/// `TESForm::CopyAllComponents` and `CompareAllComponents` (`004851b0`,
/// `00485270`), the byte copy `00401460` (`cdecl`: destination, source,
/// size), `memcmp` (`00ec4835`), and the decal data's copy (`005932b0`)
/// and comparison (`005932e0`) (`thiscall`, one word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const COPY_BYTES: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
const DECAL_DATA_COPY: u32 = 0x0059_32b0;
const DECAL_DATA_DIFFERS: u32 = 0x0059_32e0;
/// `00401080` (`cdecl`: the address of a word and a byte; the loaders
/// call it on each word of a record they byte-swap).
const SWAP_WORD: u32 = 0x0040_1080;

/// `form` cast to `BGSImpactData`, or 0.
fn cast_to_impact_data(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_IMPACT_DATA, 0u32],
    )
    .u32()
}

// Translated from 0058dd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSImpactData`'s copy (by the RTTI it casts to; `thiscall`, one word,
/// `RET 4`): when `source` casts, `TESForm::CopyAllComponents` with the
/// cast form, its 0x18 bytes at +0x30 (`00401460`), its decal data at +0x54
/// (`005932b0`) and the words at +0x48, +0x4C and +0x50.
pub fn fn_0058dd10(e: &mut Engine, this: Ptr, source: u32) {
    let other = cast_to_impact_data(e, source);
    if other == 0 {
        return;
    }
    let at = this.addr();
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
    e.call(
        COPY_BYTES,
        &args![at + 0x30, other.wrapping_add(0x30), 0x18u32],
    );
    e.call(
        DECAL_DATA_COPY,
        &args![at + DECAL_DATA, other.wrapping_add(DECAL_DATA)],
    );
    for offset in [0x48, 0x4c, 0x50] {
        let word = e.mem.u32(other.wrapping_add(offset));
        e.mem.set_u32(at + offset, word);
    }
}

// Translated from 0058dda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSImpactData`'s comparison (`thiscall`, one word, `RET 4`): whether it
/// differs from `form`: true when it does not cast, when
/// `TESForm::CompareAllComponents` answers true, when `memcmp` finds the
/// 0x18 bytes at +0x30 different, when this form has decal data (`0058de80`
/// false) that `005932e0` finds different, or when the words at +0x48,
/// +0x4C or +0x50 differ.
pub fn fn_0058dda0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_to_impact_data(e, form);
    if other == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool() {
        return true;
    }
    let at = this.addr();
    let order = e
        .call(MEMCMP, &args![at + 0x30, other.wrapping_add(0x30), 0x18u32])
        .u32();
    if order != 0 {
        return true;
    }
    if !fn_0058de80(e, this)
        && e.call(
            DECAL_DATA_DIFFERS,
            &args![at + DECAL_DATA, other.wrapping_add(DECAL_DATA)],
        )
        .bool()
    {
        return true;
    }
    [0x48u32, 0x4c, 0x50]
        .iter()
        .any(|offset| e.mem.u32(at + offset) != e.mem.u32(other.wrapping_add(*offset)))
}

/// The word offsets `0058e250` swaps, in its order.
const RECORD_WORDS_0058E250: [u32; 13] = [
    0x00, 0x14, 0x1c, 0x20, 0x04, 0x08, 0x0c, 0x18, 0x24, 0x28, 0x2c, 0x30, 0x34,
];

// Translated from 0058e250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`): `00401080(address, 0)` on 13 words of the
/// record, in the order of `RECORD_WORDS_0058E250`.
pub fn fn_0058e250(e: &mut Engine, this: Ptr) {
    for offset in RECORD_WORDS_0058E250 {
        e.call(SWAP_WORD, &args![this.addr() + offset, 0u32]);
    }
}

// Translated from 0058e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`): `00401080(address, 0)` on the five words at
/// +0, +4, +8, +0xC and +0x10.
pub fn fn_0058e3a0(e: &mut Engine, this: Ptr) {
    for offset in [0x00, 0x04, 0x08, 0x0c, 0x10] {
        e.call(SWAP_WORD, &args![this.addr() + offset, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_db40, fn_0058db40(Ptr) -> Ptr),
        entry!(0x0058_dbe0, fn_0058dbe0(Ptr, u32) -> Ptr),
        entry!(0x0058_dc10, fn_0058dc10(Ptr)),
        entry!(0x0058_dc90, fn_0058dc90(Ptr)),
        entry!(0x0058_de80, fn_0058de80(Ptr) -> bool),
        entry!(0x0058_dea0, fn_0058dea0(Ptr, Ptr) -> bool),
        entry!(0x0058_e430, fn_0058e430(Ptr, u32)),
        entry!(0x0058_dd10, fn_0058dd10(Ptr, u32)),
        entry!(0x0058_dda0, fn_0058dda0(Ptr, u32) -> bool),
        entry!(0x0058_e250, fn_0058e250(Ptr)),
        entry!(0x0058_e3a0, fn_0058e3a0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0058db40_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_db40,
            size: 0x78,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x0059_3240, &[A::T(0x54)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5e)]),
                (0x0058_dc90, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_2f6c), (0x18, 4, 0x0103_2f44)],
        });
    }
    #[test]
    fn at_0058dbe0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_dc10, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_dbe0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_dbe0)
                .collect();
            let mut expected = vec![(0x0058_dc10, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod impact_data_tests {
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

    fn memset(e: &mut Engine) {
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
    }

    #[test]
    fn fn_0058dc10_destroys_the_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x78);
        quiet(&mut e, &[0x0048_3710, 0x0048_9070, 0x0048_3630]);
        e.call_log = Some(vec![]);
        e.call(0x0058_dc10, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_2f6c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_2f44);
        assert_eq!(calls(&e, 0x0048_3710), vec![vec![this + 0x54]]);
        assert_eq!(calls(&e, 0x0048_9070), vec![vec![this + 0x18]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);
    }

    #[test]
    fn fn_0058dc90_sets_the_defaults() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.set_global(0x0101_622c, 0.25f32);
        e.set_global(0x0101_e580, 15.0f32);
        e.set_global(0x0101_e57c, 16.0f32);
        memset(&mut e);
        let this = e.mem.alloc(0x78);
        for offset in (0x30..0x54).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call(0x0058_dc90, &args![this]);
        let words: Vec<u32> = (0x30..0x54)
            .step_by(4)
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            words,
            vec![
                0.25f32.to_bits(),
                2,
                15.0f32.to_bits(),
                16.0f32.to_bits(),
                1,
                0,
                0,
                0,
                0
            ]
        );
    }

    #[test]
    fn fn_0058de80_reads_bit_zero_of_the_flags() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x78);
        e.mem.set_u8(this + 0x44, 0xfe);
        assert!(!e.call(0x0058_de80, &args![this]).bool());
        e.mem.set_u8(this + 0x44, 1);
        assert!(e.call(0x0058_de80, &args![this]).bool());
    }

    #[test]
    fn fn_0058e430_sets_the_second_sound() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x78);
        e.call(0x0058_e430, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0x50), 0x1234);
    }

    /// A file whose chunks are `chunks` (type, data bytes), its format
    /// version `version` and endian swapping `swap`.
    fn file(
        e: &mut Engine,
        form_type: u32,
        chunks: Vec<(u32, Vec<u8>)>,
        version: u32,
        swap: bool,
    ) -> u32 {
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((chunks, 0usize)));
        e.register_double(FILE_FORM_TYPE, move |_, _| ret(form_type));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).map_or(0, |x| x.0))
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.write(a[1], &c.0[c.1].1);
            Ret::default()
        });
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA_SIZED, move |e, a| {
            let c = c.borrow();
            e.mem.write(a[1], &c.0[c.1].1);
            Ret::default()
        });
        let c = chunks;
        e.register_double(FORM_LOAD_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.write(a[2], &c.0[c.1].1);
            Ret::default()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        e.register_double(FILE_VERSION, move |_, _| ret(version));
        if swap {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        } else {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(0));
        }
        memset(e);
        quiet(
            e,
            &[
                0x0048_5110,
                0x0048_92d0,
                SWAP_DATA,
                0x0058_e250,
                0x0059_3450,
                0x0067_3400,
                0x0058_9f50,
            ],
        );
        file
    }

    /// A form whose vtable slots +0xE0 and +0x134 are doubles; the decal
    /// data fields answer at +0x54.
    fn impact(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x78);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        quiet(e, &[0x7000_00e0, 0x7000_0134]);
        e.register(DECAL_DATA_FIELDS, |_, a| ret(a[0]));
        this
    }

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    #[test]
    fn fn_0058dea0_reads_the_current_chunks() {
        let mut e = Engine::new();
        let this = impact(&mut e);
        let data = words(&[
            1.5f32.to_bits(),
            1,
            2.0f32.to_bits(),
            3.0f32.to_bits(),
            2,
            0x0000_0101,
        ]);
        let f = file(
            &mut e,
            0x5e,
            vec![
                (CHUNK_EDID, words(&[0x6f6c])),
                (CHUNK_OBND, vec![]),
                (CHUNK_MODL, vec![]),
                (CHUNK_MODT, vec![]),
                (CHUNK_DATA, data.clone()),
                (CHUNK_DODT, vec![]),
                (CHUNK_DNAM, words(&[0x11])),
                (CHUNK_SNAM, words(&[0x22])),
                (CHUNK_NAM1, words(&[0x33])),
            ],
            10,
            true,
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_dea0, &args![this, f]).bool());
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, f]]);
        assert_eq!(calls(&e, SWAP_DATA), vec![vec![this + 0x30]; 2]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, f]]);
        assert_eq!(calls(&e, 0x0048_92d0), vec![vec![this + 0x18, f]; 2]);
        assert_eq!(
            calls(&e, FORM_LOAD_DATA),
            vec![vec![this, f, this + 0x30, 0x18]]
        );
        assert_eq!(e.mem.bytes(this + 0x30, 0x18), data);
        assert_eq!(calls(&e, 0x0059_3450), vec![vec![this + 0x54, f]]);
        assert_eq!(calls(&e, 0x0067_3400), vec![vec![this, 0x11]]);
        assert_eq!(calls(&e, 0x0058_9f50), vec![vec![this, 0x22]]);
        assert_eq!(e.mem.u32(this + 0x50), 0x33);
    }

    #[test]
    fn fn_0058dea0_spreads_the_older_data_record() {
        let mut e = Engine::new();
        let this = impact(&mut e);
        // Record words: each the index of its word, as a float where the
        // code moves a float; +0x10/+0x11 two bytes; +0x04 a signalling NaN.
        let mut record = vec![0u32; 14];
        for (i, word) in record.iter_mut().enumerate() {
            *word = (i as f32 + 100.0).to_bits();
        }
        record[1] = 0x7f80_0001;
        record[3] = 7;
        record[4] = 0x0000_0201;
        record[12] = 0x55;
        record[13] = 4;
        let f = file(&mut e, 0x5e, vec![(CHUNK_DATA, words(&record))], 9, false);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_dea0, &args![this, f]).bool());
        let loaded = calls(&e, FORM_LOAD_DATA);
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0][3], 0x38);
        assert!(calls(&e, 0x0058_e250).is_empty());
        assert_eq!(e.mem.u32(this + 0x30), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x34), 7);
        assert_eq!(e.mem.f32(this + 0x38), 110.0);
        assert_eq!(e.mem.f32(this + 0x3c), 111.0);
        assert_eq!(e.mem.u32(this + 0x40), 4);
        let decal: Vec<f32> = (0..7).map(|i| e.mem.f32(this + 0x54 + 4 * i)).collect();
        assert_eq!(decal, vec![100.0, 105.0, 107.0, 108.0, 106.0, 109.0, 102.0]);
        assert_eq!(e.mem.u8(this + 0x54 + 0x1c), 2);
        assert_eq!(e.mem.u8(this + 0x54 + 0x1d), 1);
        assert_eq!(e.mem.u32(this + 0x54 + 0x20), 0x55);
    }

    #[test]
    fn fn_0058dea0_swaps_the_older_record_and_refuses_other_records() {
        let mut e = Engine::new();
        let this = impact(&mut e);
        let f = file(&mut e, 0x5e, vec![(CHUNK_DATA, vec![0; 0x38])], 3, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_dea0, &args![this, f]).bool());
        let loaded = calls(&e, FORM_LOAD_DATA);
        assert_eq!(calls(&e, 0x0058_e250), vec![vec![loaded[0][2]]]);
        assert_eq!(calls(&e, SWAP_DATA).len(), 1);

        let mut e = Engine::new();
        let this = impact(&mut e);
        let f = file(&mut e, 0x5f, vec![(CHUNK_DATA, vec![])], 10, false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0058_dea0, &args![this, f]).bool());
        assert!(calls(&e, 0x0048_5110).is_empty());
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

    fn setup(e: &mut Engine) {
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_IMPACT_DATA, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(COPY_BYTES, |_, _| Ret::default());
        e.register(DECAL_DATA_COPY, |_, _| Ret::default());
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        e.register(MEMCMP, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])),
            ..Ret::default()
        });
        e.register(DECAL_DATA_DIFFERS, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0] + 0x10) != e.mem.u32(a[1] + 0x10)),
            ..Ret::default()
        });
        e.register(DECAL_DATA_FIELDS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
    }

    #[test]
    fn fn_0058dd10_copies_impact_data() {
        let mut e = Engine::new();
        setup(&mut e);
        let this = e.mem.alloc(0x80);
        let other = e.mem.alloc(0x80);
        e.mem.set_u32(other + 0x48, 0x48);
        e.mem.set_u32(other + 0x4c, 0x4c);
        e.mem.set_u32(other + 0x50, 0x50);
        e.mem.set_u32(other + 0xc, 1);
        let targets = [COPY_ALL_COMPONENTS, COPY_BYTES, DECAL_DATA_COPY];
        e.call_log = Some(vec![]);
        e.call(0x0058_dd10, &[this, other]);
        assert!(calls_to(&mut e, &targets).is_empty());
        e.mem.set_u32(other + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_dd10, &[this, other]);
        assert_eq!(
            calls_to(&mut e, &targets),
            vec![
                (COPY_ALL_COMPONENTS, vec![this, other]),
                (COPY_BYTES, vec![this + 0x30, other + 0x30, 0x18]),
                (DECAL_DATA_COPY, vec![this + 0x54, other + 0x54]),
            ]
        );
        for offset in [0x48, 0x4c, 0x50] {
            assert_eq!(e.mem.u32(this + offset), offset);
        }
    }

    #[test]
    fn fn_0058dda0_compares_impact_data() {
        let mut e = Engine::new();
        setup(&mut e);
        let this = e.mem.alloc(0x80);
        let other = e.mem.alloc(0x80);
        e.mem.set_u32(other + 0xc, 1);
        assert!(e.call(0x0058_dda0, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 0);
        assert!(!e.call(0x0058_dda0, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0058_dda0, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 0);
        e.mem.set_u32(other + 0x30, 1);
        assert!(e.call(0x0058_dda0, &[this, other]).bool());
        e.mem.set_u32(other + 0x30, 0);
        // The decal data differs: only counted when this form has some.
        e.mem.set_u32(other + 0x64, 1);
        assert!(e.call(0x0058_dda0, &[this, other]).bool());
        e.mem.set_u8(this + 0x44, 1);
        assert!(!e.call(0x0058_dda0, &[this, other]).bool());
        e.mem.set_u32(other + 0x64, 0);
        for offset in [0x48, 0x4c, 0x50] {
            e.mem.set_u32(other + offset, 1);
            assert!(e.call(0x0058_dda0, &[this, other]).bool());
            e.mem.set_u32(other + offset, 0);
        }
    }

    #[test]
    fn fn_0058e250_and_0058e3a0_swap_the_words() {
        let mut e = Engine::new();
        e.register(SWAP_WORD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0058_e250, &[0x1000]);
        let log = calls_to(&mut e, &[SWAP_WORD]);
        let expected: Vec<(u32, Vec<u32>)> = RECORD_WORDS_0058E250
            .iter()
            .map(|o| (SWAP_WORD, vec![0x1000 + o, 0]))
            .collect();
        assert_eq!(log, expected);
        e.call_log = Some(vec![]);
        e.call(0x0058_e3a0, &[0x1000]);
        let log = calls_to(&mut e, &[SWAP_WORD]);
        let expected: Vec<(u32, Vec<u32>)> = [0u32, 4, 8, 0xc, 0x10]
            .iter()
            .map(|o| (SWAP_WORD, vec![0x1000 + o, 0]))
            .collect();
        assert_eq!(log, expected);
    }
}
