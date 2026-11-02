//! `BSShader/run_00ba02b0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::ni_pointer_clear;
use crate::units::bsshader::geometrydecalshader::shader_inline::{
    release_reference, MEMORY_ALLOCATE, MEMORY_MANAGER,
};

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays), the `NiTArray` set-size
/// `00a90180` (`thiscall` on the array at +0x28, one word) and this effect's vtable
/// (`010b7ec8`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const VTABLE_010B7EC8: u32 = 0x010b_7ec8;
/// The global that keeps the effect (`011ffff8`) and the byte it clears (`011f4ac0`).
const EFFECT_011FFFF8: u32 = 0x011f_fff8;
const FLAG_011F4AC0: u32 = 0x011f_4ac0;
/// The `float` constants it copies (values read in Ghidra: 0.1, 0.6, 0.985, 0.01, 0.4, 0.05).
const CONSTANT_0101E2BC: u32 = 0x0101_e2bc;
const CONSTANT_01018180: u32 = 0x0101_8180;
const CONSTANT_010231A4: u32 = 0x0102_31a4;
const CONSTANT_01013EA4: u32 = 0x0101_3ea4;
const CONSTANT_010231A8: u32 = 0x0102_31a8;
const CONSTANT_010181BC: u32 = 0x0101_81bc;
/// The `float` globals it sets (`011ffff4`, `01200004` to `01200020`).
const PARAMETER_011FFFF4: u32 = 0x011f_fff4;
const FLOATS_01200004: u32 = 0x0120_0004;

// Translated from 00ba02f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba02b0`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010b7ec8`, clears the `NiPointer` word +0x58, sizes the array at +0x28 to 2 (`00a90180`),
/// keeps the effect in `011ffff8` and clears the byte `011f4ac0`. Then, through the FPU (each
/// constant loaded once, a signalling NaN quieted): 0.0 at +0x5c and +0x60, the `float` at
/// `0101e2bc` at +0x64 and +0x68, 1.0 at +0x6c, 0.0 at +0x70 and `011ffff4`, the `float`s at
/// `0101e2bc`, `01018180`, `010231a4`, `01013ea4`, `010231a8`, `01018180`, `010231a4` and
/// `010181bc` at `01200004` to `01200020`. Finally the `NiPointer` at +0x58 is cleared (its
/// object released when set). Answers `this`.
// Translation incomplete: 00ba02f0 - the C++ exception frame (FS:[0] chain, unwind state 1 for the base and the pointer) is not translated.
pub fn fn_00ba02f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B7EC8);
    e.mem.set_u32(base + 0x58, 0);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 2u32]);
    e.set_global(EFFECT_011FFFF8, base);
    e.set_global(FLAG_011F4AC0, 0u8);
    let loaded = |e: &Engine, constant: u32| x87_quiet_bits(e.global::<u32>(constant));
    let tenth = loaded(e, CONSTANT_0101E2BC);
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x64, tenth);
    e.mem.set_u32(base + 0x68, tenth);
    e.mem.set_u32(base + 0x6c, 0x3f80_0000);
    e.mem.set_u32(base + 0x70, 0);
    e.set_global(PARAMETER_011FFFF4, 0u32);
    let second = loaded(e, CONSTANT_01018180);
    let third = loaded(e, CONSTANT_010231A4);
    let words = [
        tenth,
        second,
        third,
        loaded(e, CONSTANT_01013EA4),
        loaded(e, CONSTANT_010231A8),
        second,
        third,
        loaded(e, CONSTANT_010181BC),
    ];
    for (index, word) in words.iter().enumerate() {
        e.set_global(FLOATS_01200004 + 4 * index as u32, *word);
    }
    ni_pointer_clear(e, base + 0x58);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_02f0, fn_00ba02f0(Ptr) -> Ptr),
        entry!(0x00ba_02b0, fn_00ba02b0(Ptr)),
        entry!(0x00ba_0400, fn_00ba0400(Ptr)),
        entry!(0x00ba_04c0, fn_00ba04c0(Ptr, u32, u32)),
    ]
}

/// `ShadowSceneLight::RemoveTriGeom` (engine map, `00ba01a0`, `thiscall` on the light: the
/// geometry, the list node).
const REMOVE_TRI_GEOM_00BA01A0: u32 = 0x00ba_01a0;

// Translated from 00ba02b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba02b0`): a shadow scene light's geometry list emptied,
/// `thiscall`, no stack words. Walks the list at +0x138 (each node: next +0, item +8; the next
/// node read before the call) and removes each non-null item with its node
/// (`ShadowSceneLight::RemoveTriGeom`); then clears the list head.
pub fn fn_00ba02b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut node = e.mem.u32(base + 0x138);
    while node != 0 {
        let item = e.mem.u32(node + 8);
        let next = e.mem.u32(node);
        if item != 0 {
            e.call(REMOVE_TRI_GEOM_00BA01A0, &args![base, item, node]);
        }
        node = next;
    }
    e.mem.set_u32(base + 0x138, 0);
}

/// The array setter `00ba3aa0` (`thiscall` on an `NiTPrimitiveArray`: index, the address of
/// the word) and `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;

/// Element `index` of the `NiTPrimitiveArray` at `array` set to `value` through `00ba3aa0`
/// (the word in a local).
fn set_at(e: &mut Engine, array: u32, index: u32, value: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![array, index, local.addr()]);
    });
}

// Translated from 00ba0400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba0400 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 for the pointer and the base) is not translated.
/// Unnamed in the engine map (`run_00ba02b0`): the destructor of the effect `00ba02f0` builds,
/// `thiscall`, no stack words. Stores the vtable `010b7ec8`; each of the 9 effect parameters
/// (+0x1c elements) that is set is deleted through its slot +0 with 1 (the element tested
/// twice) and its element set to null; the `NiPointer` at +0x58 releases its object (not
/// cleared); then `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00ba0400(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B7EC8);
    for index in 0..9u32 {
        let element = e.mem.u32(base + 0x1c).wrapping_add(4 * index);
        if e.mem.u32(element) != 0 {
            let param = e.mem.u32(element);
            if param != 0 {
                e.vcall(param, 0, &args![1u32]);
            }
            set_at(e, base + 0x18, index, 0);
        }
    }
    let pointer = e.mem.u32(base + 0x58);
    if pointer != 0 {
        release_reference(e, pointer);
    }
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

/// The effect-array sizer `00ba3f90` (unnamed; `thiscall`, one word),
/// `ImageSpaceShaderParam`'s constructor (`00b8a9e0`), `InitConstantMap` (`00b8ab10`, two
/// words) and `00b8a7c0` (unnamed, `thiscall`, no stack words), and the effect setter
/// `00ba3c70` (unit `run_00ba3aa0`).
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const PARAM_SET_UP_00B8A7C0: u32 = 0x00b8_a7c0;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;

// Translated from 00ba04c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba04c0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 8 for the parameter allocations) is not translated.
/// Unnamed in the engine map (`run_00ba02b0`): the `Setup` slot of the effect `00ba02f0`
/// builds, `thiscall`, two words (the manager; a word it does not read). Sizes the arrays to 9
/// (`00ba3f90`) and `pTextures` to 2; then nine parameters (0x88 bytes each from the memory
/// manager; the constructor only on a block) with `InitConstantMap(2, 6)` and the byte +0x55
/// cleared, `00b8a7c0` for parameters 0, 1, 6, 7 and 8; parameter `i` is stored at `i` and
/// given with the manager's effect at +0x1a4 + 4 `i` to effect `i` (output 0).
pub fn fn_00ba04c0(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 9u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 2u32]);
    for index in 0..9u32 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
            .u32();
        let param = if block != 0 {
            e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
        } else {
            0
        };
        e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 2u32, 6u32]);
        e.mem.set_u8(param + 0x55, 0);
        if matches!(index, 0 | 1 | 6 | 7 | 8) {
            e.call(PARAM_SET_UP_00B8A7C0, &args![param]);
        }
        set_at(e, base + 0x18, index, param);
        let effect = e.mem.u32(e.mem.u32(manager + 8) + 0x1a4 + 4 * index);
        e.call(
            SET_EFFECT_00BA3C70,
            &args![base, index, effect, param, 0u32],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ba02f0_builds_the_effect_and_its_globals() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_00A90180, |_, a| a[0].into_ret());
        for page in [
            0x0101_e000u32,
            0x0101_8000,
            0x0102_3000,
            0x0101_3000,
            0x011f_f000,
            0x0120_0000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        let constants = [
            (CONSTANT_0101E2BC, 0.1f32.to_bits()),
            (CONSTANT_01018180, 0x7f80_0001),
            (CONSTANT_010231A4, 0.985f32.to_bits()),
            (CONSTANT_01013EA4, 0.01f32.to_bits()),
            (CONSTANT_010231A8, 0.4f32.to_bits()),
            (CONSTANT_010181BC, 0.05f32.to_bits()),
        ];
        for (address, bits) in constants {
            e.set_global(address, bits);
        }
        e.set_global(FLAG_011F4AC0, 7u8);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x74);
        e.mem.write(this, &[0xcc; 0x74]);
        assert_eq!(e.call(0x00ba_02f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B7EC8);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        let tenth = 0.1f32.to_bits();
        let words: Vec<u32> = (0x5cu32..=0x70)
            .step_by(4)
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(words, vec![0, 0, tenth, tenth, 0x3f80_0000, 0]);
        assert_eq!(e.global::<u32>(EFFECT_011FFFF8), this);
        assert_eq!(e.global::<u8>(FLAG_011F4AC0), 0);
        assert_eq!(e.global::<u32>(PARAMETER_011FFFF4), 0);
        let globals: Vec<u32> = (0..8)
            .map(|i| e.global::<u32>(FLOATS_01200004 + 4 * i))
            .collect();
        assert_eq!(
            globals,
            vec![
                tenth,
                0x7fc0_0001,
                0.985f32.to_bits(),
                0.01f32.to_bits(),
                0.4f32.to_bits(),
                0x7fc0_0001,
                0.985f32.to_bits(),
                0.05f32.to_bits()
            ]
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 2]),
            ]
        );
    }
}

#[cfg(test)]
mod light_and_setup_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00ba02b0_removes_every_item_and_clears_the_list() {
        let mut e = rig();
        e.register(REMOVE_TRI_GEOM_00BA01A0, |_, _| Ret::default());
        let this = e.mem.alloc(0x140);
        let nodes: Vec<u32> = (0..3).map(|_| e.mem.alloc(0xc)).collect();
        e.mem.set_u32(nodes[0], nodes[1]);
        e.mem.set_u32(nodes[1], nodes[2]);
        e.mem.set_u32(nodes[0] + 8, 0x100);
        e.mem.set_u32(nodes[2] + 8, 0x300);
        e.mem.set_u32(this + 0x138, nodes[0]);
        e.call(0x00ba_02b0, &args![this]);
        assert_eq!(
            calls(&e, REMOVE_TRI_GEOM_00BA01A0),
            vec![vec![this, 0x100, nodes[0]], vec![this, 0x300, nodes[2]]]
        );
        assert_eq!(e.mem.u32(this + 0x138), 0);
    }

    #[test]
    fn fn_00ba0400_deletes_the_parameters_and_releases_the_pointer() {
        let mut e = rig();
        for callee in [ARRAY_SET_AT_00BA3AA0, EFFECT_DESTRUCT_00BA3D30, 0x0e72_0000] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(0x0e73_0000, &[0x0e72_0000]);
        let this = e.mem.alloc(0x74);
        let list = e.mem.alloc(0x24);
        let param = e.mem.alloc(8);
        e.mem.set_u32(param, 0x0e73_0000);
        e.mem.set_u32(list + 4 * 8, param);
        e.mem.set_u32(this + 0x1c, list);
        let pointer = object(&mut e, 1);
        e.mem.set_u32(this + 0x58, pointer);
        e.call(0x00ba_0400, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010B7EC8);
        assert_eq!(calls(&e, 0x0e72_0000), vec![vec![param, 1]]);
        assert_eq!(calls(&e, ARRAY_SET_AT_00BA3AA0)[0][..2], [this + 0x18, 8]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![pointer]]);
        assert_eq!(e.mem.u32(this + 0x58), pointer);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
    }

    #[test]
    fn fn_00ba04c0_sets_up_nine_parameters() {
        let mut e = rig();
        e.map(0x0e74_0000, 0x1000);
        let mut next = 0x0e74_0000u32;
        e.register_double(PARAM_CONSTRUCT_00B8A9E0, move |_, _| {
            next += 0x100;
            next.into_ret()
        });
        for callee in [
            SIZE_ARRAYS_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            PARAM_SET_UP_00B8A7C0,
            ARRAY_SET_AT_00BA3AA0,
            SET_EFFECT_00BA3C70,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x74);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x1c8);
        for index in 0..9u32 {
            e.mem
                .set_u32(effects + 0x1a4 + 4 * index, 0x0e75_0000 + index);
        }
        e.mem.set_u32(manager + 8, effects);
        e.call(0x00ba_04c0, &args![this, manager, 0u32]);
        let param = |i: u32| 0x0e74_0100 + 0x100 * i;
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 9]]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 2]]
        );
        let expected: Vec<Vec<u32>> = (0..9u32)
            .map(|i| vec![this, i, 0x0e75_0000 + i, param(i), 0])
            .collect();
        assert_eq!(calls(&e, SET_EFFECT_00BA3C70), expected);
        let maps: Vec<Vec<u32>> = (0..9u32).map(|i| vec![param(i), 2, 6]).collect();
        assert_eq!(calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10), maps);
        let set_up: Vec<Vec<u32>> = [0u32, 1, 6, 7, 8].iter().map(|i| vec![param(*i)]).collect();
        assert_eq!(calls(&e, PARAM_SET_UP_00B8A7C0), set_up);
        let indexes: Vec<u32> = calls(&e, ARRAY_SET_AT_00BA3AA0)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(indexes, (0..9).collect::<Vec<u32>>());
    }
}
