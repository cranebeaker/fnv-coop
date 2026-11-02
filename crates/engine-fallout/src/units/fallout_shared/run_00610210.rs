//! `fallout shared/run_00610210` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00610210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00610210 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0xc (0x524 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00610210(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.call(0x0048_cfe0, &args![base + 0x2c]);
    e.call(0x0048_ba00, &args![base + 0x40]);
    e.mem.set_u32(base, 0x0104_b4bc);
    e.mem.set_u32(base + 0x18, 0x0104_b4a4);
    e.mem.set_u32(base + 0x24, 0x0104_b48c);
    e.mem.set_u32(base + 0x2c, 0x0104_b46c);
    e.mem.set_u32(base + 0x40, 0x0104_b458);
    e.call(0x0047_ff70, &args![base + 0x74]);
    e.call(0x0047_ff70, &args![base + 0x80]);
    e.call(0x0096_a2d0, &args![base + 0x8c]);
    e.call(0x0096_a2d0, &args![base + 0xa8]);
    e.call(
        0x00ec_782f,
        &args![base + 0xb0, 0x18, 0x10, 0x0048_8f50, 0x0048_9070],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x230, 0xc, 0x10, 0x0048_e270, 0x0048_e2e0],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x2f0, 0xc, 0x6, 0x0048_e270, 0x0048_e2e0],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x338, 0x18, 0x6, 0x0048_8f50, 0x0048_9070],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x3c8, 0x18, 0x2, 0x0061_04a0, 0x0051_b950],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x3f8, 0x20, 0x4, 0x0044_9610, 0x0044_9680],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x478, 0x20, 0x4, 0x0044_9610, 0x0044_9680],
    );
    e.call(0x0040_37b0, &args![base + 0x50c]);
    e.call(0x0060_b9e0, &args![base + 0x514, 0x0, 0x1]);
    e.call(0x004f_15a0, &args![this, 0xc]);
    e.call(0x0061_06a0, &args![this]);
    this
}

/// `memset` (cdecl: destination, value, count).
const MEMSET: u32 = 0x0040_3d30;
/// The race's face coordinates for a sex (`005d9fb0`, `__thiscall`), the
/// default set (`BSFaceGenManager::GetFaceGenCoord`), the copy
/// (`BSFaceGenManager::CopyFaceGenCoord`, cdecl: from, to, 0, 0) and
/// `BSFaceGenManager::InitFaceGenCoord` (cdecl).
const RACE_FACE_COORD: u32 = 0x005d_9fb0;
const DEFAULT_FACE_COORD: u32 = 0x0065_21e0;
const COPY_FACE_COORD: u32 = 0x0065_29a0;
const INIT_FACE_COORD: u32 = 0x0065_21f0;
/// `TESSoundFile::SetSoundFile` (engine map; on a texture it sets the
/// texture's path), `__thiscall`, `RET 4`.
const SET_PATH: u32 = 0x0048_9100;
/// Default path tables (pointers in `.data`): head part models and
/// textures (8 per sex), body part textures and models (3 per sex), body
/// models (one per sex).
const HEAD_MODEL_PATHS: u32 = 0x0119_9fe8;
const HEAD_TEXTURE_PATHS: u32 = 0x0119_a028;
const BODY_TEXTURE_PATHS: u32 = 0x0119_a068;
const BODY_PART_PATHS: u32 = 0x0119_a080;
const BODY_MODEL_PATHS: u32 = 0x0119_9fbc;

/// `base + stride * a + size * b`, as the exe's `IMUL`/`LEA` arithmetic.
fn element(base: u32, stride: u32, a: u32, size: u32, b: u32) -> u32 {
    base.wrapping_add(stride.wrapping_mul(a))
        .wrapping_add(size.wrapping_mul(b))
}

// Translated from 006104a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the race's two body models (the array at +0x3c8 the
/// constructor `fn_00610210` builds with it): the `TESModel` constructor
/// (`00488f50`), then the vtable `0104b5f8`. Returns `this` (`ECX`, no
/// stack words). The engine map gives it no name.
pub fn fn_006104a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0048_8f50, &args![this]);
    e.mem.set_u32(this.addr(), 0x0104_b5f8);
    this
}

// Translated from 00610430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text of the string at `this + 0x50c` (`00559450`), or the empty
/// string at `01011584` when it has none (`ECX`, no stack words). The
/// engine map gives it no name.
pub fn fn_00610430(e: &mut Engine, this: Ptr) -> u32 {
    let text = this.addr() + 0x50c;
    if e.call(0x0055_9450, &args![text]).u32() != 0 {
        e.call(0x0055_9450, &args![text]).u32()
    } else {
        0x0101_1584
    }
}

// Translated from 00610ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ClearData` slot (+0x18, `TESForm`'s order in the Xbox PDB) of the
/// `TESRace` vtable (`0104b4bc`, its RTTI name `.?AVTESRace@@`): empties
/// the hair and eye lists (`00613850`, `00613950`), then calls virtual slot
/// +4 (`ClearDataComponent`) of every head part model and texture (8 per
/// sex), every body part texture and model (3 per sex) and both body
/// models. `ECX`, no stack words.
pub fn fn_00610ac0(e: &mut Engine, this: Ptr) {
    let race = this.addr();
    e.call(0x0061_3850, &args![this]);
    e.call(0x0061_3950, &args![this]);
    for sex in 0..2 {
        for part in 0..8 {
            e.vcall(element(race + 0xb0, 0xc0, sex, 0x18, part), 4, &args![]);
            e.vcall(element(race + 0x230, 0x60, sex, 0xc, part), 4, &args![]);
        }
    }
    for sex in 0..2 {
        for part in 0..3 {
            e.vcall(element(race + 0x2f0, 0x24, sex, 0xc, part), 4, &args![]);
            e.vcall(element(race + 0x338, 0x48, sex, 0x18, part), 4, &args![]);
        }
        e.vcall(element(race + 0x3c8, 0x18, sex, 0, 0), 4, &args![]);
    }
}

// Translated from 006106a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `InitializeData` slot (+0x14) of the `TESRace` vtable: clears the
/// 0x24 bytes of data at +0x50 and sets its seven skill bytes (every other
/// byte from +0x50) to 0xff, the four floats at +0x60..+0x6c to 1.0, the
/// default hair (+0x94, +0x98) to none, the voices to the defaults
/// (`[011cb298]`, `[011cb29c]`), the floats at +0xa0, +0xa4 to 0.0 and the
/// bytes at +0x9c, +0x9d to 0; copies the default face coordinates into
/// the race's set for each sex (`005d9fb0`, `CopyFaceGenCoord`) and
/// initializes it (`InitFaceGenCoord`); clears the word at +0x4f8; resets
/// every head part, body part and body model to its default path (slot 0,
/// then slot 0x18 or `TESSoundFile::SetSoundFile` with the path tables);
/// and clears the older and younger races (+0x504, +0x508). `ECX`, no
/// stack words.
pub fn fn_006106a0(e: &mut Engine, this: Ptr) {
    let race = this.addr();
    e.call(MEMSET, &args![race + 0x50, 0u32, 0x24u32]);
    for i in 0..7 {
        e.mem.set_u8(race + 0x50 + 2 * i, 0xff);
    }
    for i in 0..2 {
        // FLD1 / FSTP float.
        e.mem.set_u32(race + 0x68 + 4 * i, x87_store32(1.0));
        e.mem.set_u32(race + 0x60 + 4 * i, x87_store32(1.0));
    }
    e.mem.set_u32(race + 0x94, 0);
    e.mem.set_u32(race + 0x98, 0);
    let male = e.global::<u32>(0x011c_b298);
    e.mem.set_u32(race + 0x4fc, male);
    let female = e.global::<u32>(0x011c_b29c);
    e.mem.set_u32(race + 0x500, female);
    // FLDZ / FSTP float.
    e.mem.set_u32(race + 0xa0, x87_store32(0.0));
    e.mem.set_u32(race + 0xa4, x87_store32(0.0));
    e.mem.set_u8(race + 0x9c, 0);
    e.mem.set_u8(race + 0x9d, 0);
    for sex in 0..2u32 {
        let coord = e.call(RACE_FACE_COORD, &args![this, sex]).u32();
        let default = e.call(DEFAULT_FACE_COORD, &args![]).u32();
        e.call(COPY_FACE_COORD, &args![default, coord, 0u32, 0u32]);
        let coord = e.call(RACE_FACE_COORD, &args![this, sex]).u32();
        e.call(INIT_FACE_COORD, &args![coord]);
    }
    e.mem.set_u16(race + 0x4f8, 0);
    for sex in 0..2 {
        for part in 0..8 {
            let model = element(race + 0xb0, 0xc0, sex, 0x18, part);
            e.vcall(model, 0, &args![]);
            let path = e.mem.u32(element(HEAD_MODEL_PATHS, 0x20, sex, 4, part));
            e.vcall(model, 0x18, &args![path]);
            let texture = element(race + 0x230, 0x60, sex, 0xc, part);
            e.vcall(texture, 0, &args![]);
            let path = e.mem.u32(element(HEAD_TEXTURE_PATHS, 0x20, sex, 4, part));
            e.call(SET_PATH, &args![texture, path]);
        }
    }
    for sex in 0..2 {
        for part in 0..3 {
            let texture = element(race + 0x2f0, 0x24, sex, 0xc, part);
            e.vcall(texture, 0, &args![]);
            let path = e.mem.u32(element(BODY_TEXTURE_PATHS, 0xc, sex, 4, part));
            e.call(SET_PATH, &args![texture, path]);
            let model = element(race + 0x338, 0x48, sex, 0x18, part);
            e.vcall(model, 0, &args![]);
            let path = e.mem.u32(element(BODY_PART_PATHS, 0xc, sex, 4, part));
            e.vcall(model, 0x18, &args![path]);
        }
        let model = element(race + 0x3c8, 0x18, sex, 0, 0);
        e.vcall(model, 0, &args![]);
        let path = e.mem.u32(element(BODY_MODEL_PATHS, 4, sex, 0, 0));
        e.vcall(model, 0x18, &args![path]);
    }
    e.mem.set_u32(race + 0x504, 0);
    e.mem.set_u32(race + 0x508, 0);
}

// Translated from 006104c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006104c0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The `TESRace` destructor body (the scalar deleting destructor
/// `fn_00610470` runs it): stores the vtables back (`0104b4bc`, and the
/// components' at +0x18, +0x24, +0x2c, +0x40), clears the data
/// (`fn_00610ac0`), destroys +0x514 (`006013a0`) and the string at +0x50c
/// (`004037d0`), the arrays the constructor built (`eh vector destructor
/// iterator` on +0x478, +0x3f8, +0x3c8, +0x338, +0x2f0, +0x230, +0xb0 with
/// their element destructors), the lists at +0xa8 and +0x8c (`0046ffb0`),
/// the attributes at +0x80 and +0x74 (`0047ffa0`), the components at +0x40
/// (`0048ba70`), +0x2c (`0048d060`), +0x18 (`00591300`) and finally
/// `TESForm::~TESForm` (`00483630`). `ECX`, no stack words. The
/// exception-unwinding frame is not translated.
pub fn fn_006104c0(e: &mut Engine, this: Ptr) {
    let race = this.addr();
    e.mem.set_u32(race, 0x0104_b4bc);
    e.mem.set_u32(race + 0x18, 0x0104_b4a4);
    e.mem.set_u32(race + 0x24, 0x0104_b48c);
    e.mem.set_u32(race + 0x2c, 0x0104_b46c);
    e.mem.set_u32(race + 0x40, 0x0104_b458);
    fn_00610ac0(e, this);
    e.call(0x0060_13a0, &args![race + 0x514]);
    e.call(0x0040_37d0, &args![race + 0x50c]);
    let arrays: [(u32, u32, u32, u32); 7] = [
        (0x478, 0x20, 4, 0x0044_9680),
        (0x3f8, 0x20, 4, 0x0044_9680),
        (0x3c8, 0x18, 2, 0x0051_b950),
        (0x338, 0x18, 6, 0x0048_9070),
        (0x2f0, 0xc, 6, 0x0048_e2e0),
        (0x230, 0xc, 0x10, 0x0048_e2e0),
        (0xb0, 0x18, 0x10, 0x0048_9070),
    ];
    for (offset, size, count, destructor) in arrays {
        e.call(0x00ec_5fce, &args![race + offset, size, count, destructor]);
    }
    e.call(0x0046_ffb0, &args![race + 0xa8]);
    e.call(0x0046_ffb0, &args![race + 0x8c]);
    e.call(0x0047_ffa0, &args![race + 0x80]);
    e.call(0x0047_ffa0, &args![race + 0x74]);
    e.call(0x0048_ba70, &args![race + 0x40]);
    e.call(0x0048_d060, &args![race + 0x2c]);
    e.call(0x0059_1300, &args![race + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00610470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESRace` scalar deleting destructor (slot +0x10 of its vtable):
/// runs `fn_006104c0` on `this`, then frees `this` with `00401030` when
/// bit 0 of `flags` is set. Returns `this`. `__thiscall`, `RET 4`.
pub fn fn_00610470(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006104c0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_0210, fn_00610210(Ptr) -> Ptr),
        entry!(0x0061_04a0, fn_006104a0(Ptr) -> Ptr),
        entry!(0x0061_0430, fn_00610430(Ptr) -> u32),
        entry!(0x0061_0ac0, fn_00610ac0(Ptr)),
        entry!(0x0061_06a0, fn_006106a0(Ptr)),
        entry!(0x0061_04c0, fn_006104c0(Ptr)),
        entry!(0x0061_0470, fn_00610470(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00610210_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0061_0210,
            size: 0x524,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0048_cfe0, &[A::T(0x2c)]),
                (0x0048_ba00, &[A::T(0x40)]),
                (0x0047_ff70, &[A::T(0x74)]),
                (0x0047_ff70, &[A::T(0x80)]),
                (0x0096_a2d0, &[A::T(0x8c)]),
                (0x0096_a2d0, &[A::T(0xa8)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0xb0),
                        A::V(0x18),
                        A::V(0x10),
                        A::V(0x0048_8f50),
                        A::V(0x0048_9070),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x230),
                        A::V(0xc),
                        A::V(0x10),
                        A::V(0x0048_e270),
                        A::V(0x0048_e2e0),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x2f0),
                        A::V(0xc),
                        A::V(0x6),
                        A::V(0x0048_e270),
                        A::V(0x0048_e2e0),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x338),
                        A::V(0x18),
                        A::V(0x6),
                        A::V(0x0048_8f50),
                        A::V(0x0048_9070),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x3c8),
                        A::V(0x18),
                        A::V(0x2),
                        A::V(0x0061_04a0),
                        A::V(0x0051_b950),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x3f8),
                        A::V(0x20),
                        A::V(0x4),
                        A::V(0x0044_9610),
                        A::V(0x0044_9680),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x478),
                        A::V(0x20),
                        A::V(0x4),
                        A::V(0x0044_9610),
                        A::V(0x0044_9680),
                    ],
                ),
                (0x0040_37b0, &[A::T(0x50c)]),
                (0x0060_b9e0, &[A::T(0x514), A::V(0x0), A::V(0x1)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0xc)]),
                (0x0061_06a0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_b4bc),
                (0x18, 4, 0x0104_b4a4),
                (0x24, 4, 0x0104_b48c),
                (0x2c, 4, 0x0104_b46c),
                (0x40, 4, 0x0104_b458),
            ],
        });
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;

    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: u32 = 0x0300_0000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Every model and texture element of a race.
    fn elements(race: u32) -> Vec<u32> {
        let mut all = vec![];
        for sex in 0..2 {
            for part in 0..8 {
                all.push(element(race + 0xb0, 0xc0, sex, 0x18, part));
                all.push(element(race + 0x230, 0x60, sex, 0xc, part));
            }
        }
        for sex in 0..2 {
            for part in 0..3 {
                all.push(element(race + 0x2f0, 0x24, sex, 0xc, part));
                all.push(element(race + 0x338, 0x48, sex, 0x18, part));
            }
            all.push(element(race + 0x3c8, 0x18, sex, 0, 0));
        }
        all
    }

    fn race(callees: &[u32]) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(VTABLE, 0x40);
        for i in 0..0x10 {
            e.mem.set_u32(VTABLE + 4 * i, SLOTS + 4 * i);
            e.register(SLOTS + 4 * i, |_, _| Ret::default());
        }
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
        let race = e.mem.alloc(0x524);
        for at in elements(race) {
            e.mem.set_u32(at, VTABLE);
        }
        e.call_log = Some(vec![]);
        (e, race)
    }

    #[test]
    fn the_body_model_constructor_sets_its_vtable() {
        let (mut e, race) = race(&[0x0048_8f50]);
        let model = race + 0x3c8;
        assert_eq!(e.call(0x0061_04a0, &[model]).u32(), model);
        assert_eq!(e.mem.u32(model), 0x0104_b5f8);
        assert_eq!(calls_to(&e, 0x0048_8f50), vec![vec![model]]);
    }

    #[test]
    fn the_text_falls_back_to_the_empty_string() {
        let (mut e, race) = race(&[]);
        e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        assert_eq!(e.call(0x0061_0430, &[race]).u32(), 0x0101_1584);
        e.mem.set_u32(race + 0x50c, 0x7e47);
        assert_eq!(e.call(0x0061_0430, &[race]).u32(), 0x7e47);
    }

    #[test]
    fn clearing_clears_every_component() {
        let (mut e, race) = race(&[0x0061_3850, 0x0061_3950]);
        e.call(0x0061_0ac0, &[race]);
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(log[1], (0x0061_3850, vec![race]));
        assert_eq!(log[2], (0x0061_3950, vec![race]));
        let cleared: Vec<u32> = calls_to(&e, SLOTS + 4).into_iter().map(|w| w[0]).collect();
        assert_eq!(cleared, elements(race));
    }

    #[test]
    fn initializing_sets_the_defaults() {
        let (mut e, race) = race(&[MEMSET, COPY_FACE_COORD, INIT_FACE_COORD, SET_PATH]);
        e.register(RACE_FACE_COORD, |_, a| ret(0x0c00 + a[1]));
        e.register(DEFAULT_FACE_COORD, |_, _| ret(0x0def));
        e.map(0x011c_b298, 8);
        e.mem.set_u32(0x011c_b298, 0xaaa);
        e.mem.set_u32(0x011c_b29c, 0xbbb);
        e.map(0x0119_9fbc, 0x100);
        for i in 0..0x40 {
            e.mem.set_u32(0x0119_9fbc + 4 * i, 0x0100 + i);
        }
        for offset in [0x504, 0x508, 0x94, 0xa0] {
            e.mem.set_u32(race + offset, 0xffff_ffff);
        }
        e.mem.set_u16(race + 0x4f8, 0xffff);
        e.call(0x0061_06a0, &[race]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![race + 0x50, 0, 0x24]]);
        assert_eq!(e.mem.u8(race + 0x50), 0xff);
        assert_eq!(e.mem.u8(race + 0x5c), 0xff);
        assert_eq!(e.mem.f32(race + 0x60), 1.0);
        assert_eq!(e.mem.f32(race + 0x6c), 1.0);
        assert_eq!(e.mem.u32(race + 0x94), 0);
        assert_eq!(e.mem.u32(race + 0x4fc), 0xaaa);
        assert_eq!(e.mem.u32(race + 0x500), 0xbbb);
        assert_eq!(e.mem.u32(race + 0xa0), 0);
        assert_eq!(e.mem.u16(race + 0x4f8), 0);
        assert_eq!(e.mem.u32(race + 0x504), 0);
        assert_eq!(e.mem.u32(race + 0x508), 0);
        assert_eq!(
            calls_to(&e, COPY_FACE_COORD),
            vec![vec![0x0def, 0x0c00, 0, 0], vec![0x0def, 0x0c01, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, INIT_FACE_COORD),
            vec![vec![0x0c00], vec![0x0c01]]
        );
        let paths = calls_to(&e, SLOTS + 0x18);
        assert_eq!(paths.len(), 16 + 6 + 2);
        // Head part 1 of sex 1, body part 2 of sex 0, the female body.
        let table = |at: u32| 0x0100 + (at - 0x0119_9fbc) / 4;
        assert_eq!(
            paths[9],
            vec![
                element(race + 0xb0, 0xc0, 1, 0x18, 1),
                table(HEAD_MODEL_PATHS + 0x24)
            ]
        );
        assert_eq!(
            paths[18],
            vec![race + 0x338 + 2 * 0x18, table(BODY_PART_PATHS + 8)]
        );
        assert_eq!(
            paths[23],
            vec![race + 0x3c8 + 0x18, table(BODY_MODEL_PATHS + 4)]
        );
        let textures = calls_to(&e, SET_PATH);
        assert_eq!(textures.len(), 16 + 6);
        assert_eq!(textures[16], vec![race + 0x2f0, table(BODY_TEXTURE_PATHS)]);
        assert_eq!(calls_to(&e, SLOTS).len(), 16 + 16 + 6 + 6 + 2);
    }

    #[test]
    fn the_destructor_tears_down_in_order_and_deletes_on_flag_one() {
        let callees = [
            0x0061_3850,
            0x0061_3950,
            0x0060_13a0,
            0x0040_37d0,
            0x00ec_5fce,
            0x0046_ffb0,
            0x0047_ffa0,
            0x0048_ba70,
            0x0048_d060,
            0x0059_1300,
            0x0048_3630,
            0x0040_1030,
        ];
        for flags in [0u32, 1] {
            let (mut e, race) = race(&callees);
            assert_eq!(e.call(0x0061_0470, &[race, flags]).u32(), race);
            assert_eq!(e.mem.u32(race), 0x0104_b4bc);
            assert_eq!(e.mem.u32(race + 0x40), 0x0104_b458);
            let order: Vec<u32> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .map(|c| c.0)
                .filter(|a| callees.contains(a))
                .collect();
            let mut expected = vec![0x0061_3850, 0x0061_3950, 0x0060_13a0, 0x0040_37d0];
            expected.extend([0x00ec_5fce; 7]);
            expected.extend([
                0x0046_ffb0,
                0x0046_ffb0,
                0x0047_ffa0,
                0x0047_ffa0,
                0x0048_ba70,
                0x0048_d060,
                0x0059_1300,
                0x0048_3630,
            ]);
            if flags == 1 {
                expected.push(0x0040_1030);
            }
            assert_eq!(order, expected);
            let arrays = calls_to(&e, 0x00ec_5fce);
            assert_eq!(arrays[0], vec![race + 0x478, 0x20, 4, 0x0044_9680]);
            assert_eq!(arrays[2], vec![race + 0x3c8, 0x18, 2, 0x0051_b950]);
            assert_eq!(arrays[6], vec![race + 0xb0, 0x18, 0x10, 0x0048_9070]);
            assert_eq!(calls_to(&e, 0x0060_13a0), vec![vec![race + 0x514]]);
            assert_eq!(
                calls_to(&e, 0x0046_ffb0),
                vec![vec![race + 0xa8], vec![race + 0x8c]]
            );
        }
    }
}
