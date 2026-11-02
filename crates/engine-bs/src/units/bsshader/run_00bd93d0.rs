//! `BSShader/run_00bd93d0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and this effect's vtable
/// (`010bc560`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const VTABLE_010BC560: u32 = 0x010b_c560;

// Translated from 00bd93d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd93d0`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010bc560` and clears the 16-byte block at +0x58 (the bytes +0x58 to +0x5a, then the words
/// +0x5c to +0x64). Answers `this`.
pub fn fn_00bd93d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC560);
    for offset in [0x58u32, 0x59, 0x5a] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0x5cu32, 0x60, 0x64] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// The destructor `00bd9160` (engine map: `ImageSpaceEffectRefraction::~ImageSpaceEffectRefraction`,
/// `thiscall`) and `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the manager, one
/// word).
const DESTRUCT_00BD9160: u32 = 0x00bd_9160;
const MEMORY_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;

// Translated from 00bd9400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd93d0`): a scalar deleting destructor, `thiscall`, one
/// word: the destructor `00bd9160`, then the memory freed (`MemoryManager::Deallocate`) when
/// bit 0 of `flags` is set. Answers `this`.
pub fn fn_00bd9400(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_00BD9160, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The effect-array sizer `00ba3f90` (`thiscall`, one word), the `NiTArray` set-size
/// (`00a90180`), `ImageSpaceShaderParam`'s constructor (`00b8a9e0`), `InitConstantMap`
/// (`00b8ab10`, two words) and `SetPixelConstant` (`00b8a790`, five words), the array setter
/// `00ba3aa0`, and the input and effect setters `00ba3b50` and `00ba3c70` (unit
/// `run_00ba3aa0`).
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const PARAM_SET_PIXEL_CONSTANT_00B8A790: u32 = 0x00b8_a790;
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const SET_INPUTS_00BA3B50: u32 = 0x00ba_3b50;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;
/// The four `float`s the parameter's constant 0 takes (`011fa0c0` to `011fa0cc`).
const CONSTANT_011FA0C0: u32 = 0x011f_a0c0;

// Translated from 00bd9420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bd9420 - the C++ exception frame (FS:[0] chain, unwind state 0 for the parameter allocation) is not translated.
/// Unnamed in the engine map (`run_00bd93d0`): the effect's `Setup` slot, `thiscall`, two words
/// (the manager whose +8 holds the effects; a word it does not read). Sizes the effect's
/// arrays to 1 (`00ba3f90`) and `pTextures` to 3; makes one `ImageSpaceShaderParam` (0x88 bytes
/// from the memory manager; the constructor runs only on a block) with `InitConstantMap(0, 1)`
/// and pixel constant 0 from the four `float`s at `011fa0c0` (each through `FLD`/`FSTP`, a
/// signalling NaN quieted); stores it as parameter 0, sets input 0 to textures 1 and 2 (filter
/// modes 0) and effect 0 to the manager's effect at +0xa4 with the parameter.
pub fn fn_00bd9420(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 1u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 3u32]);
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
        .u32();
    let param = if block != 0 {
        e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
    } else {
        0
    };
    e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 0u32, 1u32]);
    let constant: Vec<u32> = (0..4u32)
        .map(|i| x87_quiet_bits(e.global::<u32>(CONSTANT_011FA0C0 + 4 * i)))
        .collect();
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![
            param,
            0u32,
            constant[0],
            constant[1],
            constant[2],
            constant[3]
        ],
    );
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), param);
        e.call(
            ARRAY_SET_AT_00BA3AA0,
            &args![base + 0x18, 0u32, local.addr()],
        );
    });
    e.call(
        SET_INPUTS_00BA3B50,
        &args![base, 0u32, 1u32, 2u32, 0u32, 0u32],
    );
    let effect = e.mem.u32(e.mem.u32(manager + 8) + 0xa4);
    e.call(SET_EFFECT_00BA3C70, &args![base, 0u32, effect, param, 0u32]);
}

/// `BSShaderProperty::RenderPassArray::Add` (engine map, `00ba9ee0`, `thiscall` on the array,
/// eight words).
const RENDER_PASS_ADD_00BA9EE0: u32 = 0x00ba_9ee0;

// Translated from 00bd9540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd93d0`), `thiscall`, nine words: picks a pass number from
/// the four bytes `a`, `b`, `c` and `d` (without `a`: 0x5b / 0x5a for `b` with / without `c`;
/// else 0x59 with `c`, else 0x60 / 0x58 with / without `d`; with `a`: 0x5f / 0x5e for `b` with /
/// without `c`, else 0x5d / 0x5c), and when `add` is 1 adds it to the render pass array at
/// +0x3c (`Add(geometry, pass, 1, 1, word, 0, 0, 0)`), else increments the `u16` at `count`.
/// Always clears the byte at `done` and answers 0.
pub fn fn_00bd9540(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    word: u32,
    count: u32,
    add: u8,
    done: u32,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
) -> u32 {
    let pass: u32 = if a == 0 {
        if b != 0 {
            if c != 0 {
                0x5b
            } else {
                0x5a
            }
        } else if c != 0 {
            0x59
        } else if d != 0 {
            0x60
        } else {
            0x58
        }
    } else if b != 0 {
        if c != 0 {
            0x5f
        } else {
            0x5e
        }
    } else if c != 0 {
        0x5d
    } else {
        0x5c
    };
    if add == 1 {
        let array = e.mem.u32(this.addr() + 0x3c);
        e.call(
            RENDER_PASS_ADD_00BA9EE0,
            &args![array, geometry, pass, 1u32, 1u32, word, 0u32, 0u32, 0u32],
        );
    } else {
        let value = e.mem.u16(count).wrapping_add(1);
        e.mem.set_u16(count, value);
    }
    e.mem.set_u8(done, 0);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_93d0, fn_00bd93d0(Ptr) -> Ptr),
        entry!(0x00bd_9400, fn_00bd9400(Ptr, u32) -> Ptr),
        entry!(0x00bd_9420, fn_00bd9420(Ptr, u32, u32)),
        entry!(
            0x00bd_9540,
            fn_00bd9540(Ptr, u32, u32, u32, u8, u32, u8, u8, u8, u8) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bd93d0_builds_the_effect() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x68);
        e.mem.write(this, &[0xcc; 0x68]);
        assert_eq!(e.call(0x00bd_93d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC560);
        assert_eq!(e.mem.bytes(this + 0x58, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0x5b), 0xcc);
        assert_eq!(e.mem.bytes(this + 0x5c, 0xc), vec![0; 0xc]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(EFFECT_CONSTRUCT_00BA4080, vec![this])]
        );
    }
}

#[cfg(test)]
mod setup_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00bd9400_destroys_and_frees_on_bit_0() {
        let mut e = rig();
        e.register(DESTRUCT_00BD9160, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE_00AA4060, |_, _| Ret::default());
        assert_eq!(e.call(0x00bd_9400, &args![0x1000u32, 0u32]).u32(), 0x1000);
        assert!(calls(&e, MEMORY_DEALLOCATE_00AA4060).is_empty());
        assert_eq!(e.call(0x00bd_9400, &args![0x1000u32, 1u32]).u32(), 0x1000);
        assert_eq!(
            calls(&e, DESTRUCT_00BD9160),
            vec![vec![0x1000], vec![0x1000]]
        );
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, 0x1000]]
        );
    }

    #[test]
    fn fn_00bd9420_sets_up_one_parameter() {
        let mut e = rig();
        e.map(0x011f_a000, 0x1000);
        e.set_global(CONSTANT_011FA0C0, 1.0f32);
        e.set_global(CONSTANT_011FA0C0 + 4, 0x7f80_0001u32);
        e.set_global(CONSTANT_011FA0C0 + 8, 3.0f32);
        e.set_global(CONSTANT_011FA0C0 + 0xc, 4.0f32);
        e.register(PARAM_CONSTRUCT_00B8A9E0, |_, _| 0x0e90_0000u32.into_ret());
        type Stored = std::rc::Rc<std::cell::RefCell<Vec<u32>>>;
        let stored: Stored = Default::default();
        let log = stored.clone();
        e.register_double(ARRAY_SET_AT_00BA3AA0, move |e, a| {
            log.borrow_mut().extend([a[0], a[1], e.mem.u32(a[2])]);
            Ret::default()
        });
        for callee in [
            SIZE_ARRAYS_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            PARAM_SET_PIXEL_CONSTANT_00B8A790,
            SET_INPUTS_00BA3B50,
            SET_EFFECT_00BA3C70,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x58);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0xa8);
        e.mem.set_u32(effects + 0xa4, 0x0e91_0000);
        e.mem.set_u32(manager + 8, effects);
        e.call(0x00bd_9420, &args![this, manager, 0u32]);
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 1]]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 3]]
        );
        let param = 0x0e90_0000;
        assert_eq!(
            calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10),
            vec![vec![param, 0, 1]]
        );
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![vec![
                param,
                0,
                1.0f32.to_bits(),
                0x7fc0_0001,
                3.0f32.to_bits(),
                4.0f32.to_bits()
            ]]
        );
        assert_eq!(*stored.borrow(), vec![this + 0x18, 0, param]);
        assert_eq!(
            calls(&e, SET_INPUTS_00BA3B50),
            vec![vec![this, 0, 1, 2, 0, 0]]
        );
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![vec![this, 0, 0x0e91_0000, param, 0]]
        );
    }

    /// The pass number `00bd9540` adds for the four bytes.
    fn pass_for(a: u8, b: u8, c: u8, d: u8) -> u32 {
        let mut e = rig();
        e.register(RENDER_PASS_ADD_00BA9EE0, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x3c, 0x0e92_0000);
        let done = e.mem.alloc(4);
        e.mem.set_u8(done, 1);
        let answer = e.call(
            0x00bd_9540,
            &args![this, 0x500u32, 0x600u32, 0u32, 1u32, done, a, b, c, d],
        );
        assert_eq!(answer.u32(), 0);
        assert_eq!(e.mem.u8(done), 0);
        let added = calls(&e, RENDER_PASS_ADD_00BA9EE0);
        assert_eq!(added.len(), 1);
        assert_eq!(
            added[0],
            vec![0x0e92_0000, 0x500, added[0][2], 1, 1, 0x600, 0, 0, 0]
        );
        added[0][2]
    }

    #[test]
    fn fn_00bd9540_picks_the_pass_from_the_bytes() {
        assert_eq!(pass_for(0, 1, 1, 0), 0x5b);
        assert_eq!(pass_for(0, 1, 0, 1), 0x5a);
        assert_eq!(pass_for(0, 0, 1, 1), 0x59);
        assert_eq!(pass_for(0, 0, 0, 1), 0x60);
        assert_eq!(pass_for(0, 0, 0, 0), 0x58);
        assert_eq!(pass_for(1, 1, 1, 0), 0x5f);
        assert_eq!(pass_for(1, 1, 0, 0), 0x5e);
        assert_eq!(pass_for(1, 0, 1, 1), 0x5d);
        assert_eq!(pass_for(1, 0, 0, 1), 0x5c);
    }

    #[test]
    fn fn_00bd9540_counts_instead_of_adding() {
        let mut e = rig();
        e.register(RENDER_PASS_ADD_00BA9EE0, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let count = e.mem.alloc(4);
        e.mem.set_u16(count, 0xffff);
        e.mem.set_u8(count + 2, 0x77);
        let done = e.mem.alloc(4);
        e.mem.set_u8(done, 1);
        e.call(
            0x00bd_9540,
            &args![this, 0u32, 0u32, count, 2u32, done, 0u32, 0u32, 0u32, 0u32],
        );
        assert!(calls(&e, RENDER_PASS_ADD_00BA9EE0).is_empty());
        assert_eq!(e.mem.u16(count), 0);
        assert_eq!(e.mem.u8(count + 2), 0x77);
        assert_eq!(e.mem.u8(done), 0);
    }
}
