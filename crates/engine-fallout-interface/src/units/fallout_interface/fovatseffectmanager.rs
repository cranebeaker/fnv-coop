//! `fallout/interface/fovatseffectmanager.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PC `FOVATSEffectManager` is larger than the Xbox PDB's (0x1C4
//! bytes); the offsets here are the PC code's.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiPointer` constructor (`00633c90`, thiscall: the pointer) and
/// assignment (`0066b0d0`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `` `eh vector constructor iterator' `` (`00ec782f`, stdcall: array,
/// element size, count, constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// The target destructor the iterator is given (`00800190`).
const TARGET_DESTRUCTOR: u32 = 0x0080_0190;
/// A float setting's value (`00403e20`) and an integer setting's value
/// (`0043d4d0`) (thiscall on the setting: a pointer to the value), and a
/// string setting's text (`00403df0`).
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
const SETTING_STRING_VALUE: u32 = 0x0040_3df0;
/// The settings: the distortion duration (`011db6bc`), the random part of
/// the next distortion time (`011db75c`) and the scanlines texture
/// (`011db744`).
const DISTORT_DURATION_SETTING: u32 = 0x011d_b6bc;
const DISTORT_INTERVAL_SETTING: u32 = 0x011d_b75c;
const SCANLINES_TEXTURE_SETTING: u32 = 0x011d_b744;
/// The game time in milliseconds (`00825c00`, thiscall on `011f6394`).
const TIME_GLOBALS: u32 = 0x011f_6394;
const GET_MILLISECONDS: u32 = 0x0082_5c00;
/// `rand` (`00ecadb8`).
const RAND: u32 = 0x00ec_adb8;
/// `BSShaderManager::GetTexture` (`00b55840`, Xbox PDB name, cdecl: name,
/// 1, out pointer, 0, 0).
const BS_SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;
/// Gamebryo's object allocation (`00aa13e0`) and
/// `BSShaderAccumulator::BSShaderAccumulator` (`00b660d0`, Xbox PDB name;
/// three words).
const NI_NEW: u32 = 0x00aa_13e0;
const BS_SHADER_ACCUMULATOR_CONSTRUCTOR: u32 = 0x00b6_60d0;
/// Reads the word at `this` (`00559450`) and the step the accumulator is
/// given (`005db4c0`, thiscall: one word).
const READ_WORD: u32 = 0x0055_9450;
const ACCUMULATOR_STEP: u32 = 0x005d_b4c0;

// Translated from 007ffec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOVATSEffectManager::FOVATSEffectManager` (Xbox PDB): constructs the
/// single target (+4) and the 32 targets (+0x14, 8 bytes each), the stencil
/// property (+0x114), the scanlines texture (+0x2C4) and the accumulator
/// (+0x2CC) pointers; clears the occlusion flag (+0x118), sets +0x129, the
/// distortion state (+0x28C 0, +0x290 the duration setting, +0x294 the game
/// time plus `rand() %` the interval setting, +0x298 and +0x284 off), the
/// burst and pulse values (+0x2AC, +0x2B0, +0x2C0), the scanning flags
/// (+0x2B4, +0x2B5) and +0x240; empties the single target, loads the
/// scanlines texture into +0x2C4, resets the mode (+0), the target count
/// (+0xC) and selection (+0x10, -1), makes a new `BSShaderAccumulator`
/// (0x280 bytes; `(0x65, 1, 0x2F7)`) the accumulator and steps it
/// (`005db4c0` with 0), sets +0x2C8 and clears the 16 limb entries
/// (+0x119, +0x16C, +0x12C, +0x1AC). Returns `this`.
// Translation incomplete: 007ffec0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fovats_effect_manager_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    fn_00800170(e, this.byte_add(4));
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![t + 0x14, 8u32, 0x20u32, 0x0080_0170u32, TARGET_DESTRUCTOR],
    );
    for offset in [0x114u32, 0x2c4, 0x2cc] {
        e.call(NI_POINTER_CONSTRUCT, &args![t + offset, 0u32]);
    }
    e.mem.set_u8(t + 0x118, 0);
    e.mem.set_u8(t + 0x129, 1);
    e.mem.set_f32(t + 0x28c, 0.0);
    let duration = e
        .call(SETTING_FLOAT_VALUE, &args![DISTORT_DURATION_SETTING])
        .u32();
    let duration = x87_quiet_bits(e.mem.u32(duration));
    e.mem.set_u32(t + 0x290, duration);
    e.mem.set_u8(t + 0x298, 0);
    e.mem.set_u8(t + 0x284, 0);
    e.mem.set_f32(t + 0x2ac, 0.0);
    e.mem.set_f32(t + 0x2b0, 0.0);
    e.mem.set_f32(t + 0x2c0, 0.0);
    let now = e.call(GET_MILLISECONDS, &args![TIME_GLOBALS]).u32();
    let now = f64::from(now);
    let random = e.call(RAND, &args![]).i32();
    let interval = e
        .call(SETTING_INT_VALUE, &args![DISTORT_INTERVAL_SETTING])
        .u32();
    let part = random % e.mem.i32(interval);
    e.mem
        .set_u32(t + 0x294, x87_store32(x87_add(f64::from(part), now)));
    e.mem.set_u8(t + 0x2b4, 0);
    e.mem.set_u8(t + 0x2b5, 0);
    e.mem.set_u32(t + 0x240, 0);
    e.mem.set_u32(t + 4, 0);
    e.call(NI_POINTER_ASSIGN, &args![t + 8, 0u32]);
    let name = e
        .call(SETTING_STRING_VALUE, &args![SCANLINES_TEXTURE_SETTING])
        .u32();
    e.call(
        BS_SHADER_MANAGER_GET_TEXTURE,
        &args![name, 1u32, t + 0x2c4, 0u32, 0u32],
    );
    e.mem.set_u32(t, 0);
    e.mem.set_u32(t + 0x0c, 0);
    e.mem.set_i32(t + 0x10, -1);
    let block = e.call(NI_NEW, &args![0x280u32]).u32();
    let accumulator = if block == 0 {
        0
    } else {
        e.call(
            BS_SHADER_ACCUMULATOR_CONSTRUCTOR,
            &args![block, 0x65u32, 1u32, 0x2f7u32],
        )
        .u32()
    };
    e.call(NI_POINTER_ASSIGN, &args![t + 0x2cc, accumulator]);
    let accumulator = e.call(READ_WORD, &args![t + 0x2cc]).u32();
    e.call(ACCUMULATOR_STEP, &args![accumulator, 0u32]);
    e.mem.set_u32(t + 0x2c8, 1);
    for i in 0..0x10u32 {
        e.mem.set_u8(t + i + 0x119, 0);
        e.mem.set_u32(t + i * 4 + 0x16c, 0);
        e.mem.set_u32(t + i * 4 + 0x12c, 0);
        e.mem.set_u8(t + i + 0x1ac, 0);
    }
    this
}

// Translated from 00800170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The VATS target constructor (no PDB name; `FOVATSEffectManager::
/// VATS_TARGET`, 8 bytes): constructs the pointer at +4 empty. Returns
/// `this`.
pub fn fn_00800170(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_POINTER_CONSTRUCT, &args![this.byte_add(4), 0u32]);
    this
}

/// `00801450` and `00801880` (`thiscall`; unplaced in the engine map): the two draws
/// `00801420` chooses between.
const DRAW_00801450: u32 = 0x0080_1450;
const DRAW_00801880: u32 = 0x0080_1880;
/// The renderer (`0043c4b0`), `NiColorA` construction (`00414430`, `RET 0x10`),
/// `004e9510` (a `bool`), `BSRenderedTexture::StopOffscreen` (`00b6b260`) and
/// `00b6b8d0` (cdecl, two words).
const RENDERER_0043C4B0: u32 = 0x0043_c4b0;
const COLOR_A_INIT_00414430: u32 = 0x0041_4430;
const TEST_004E9510: u32 = 0x004e_9510;
const RENDERED_TEXTURE_STOP_OFFSCREEN: u32 = 0x00b6_b260;
const SHADER_00B6B8D0: u32 = 0x00b6_b8d0;
/// The two render flags `DrawTargetToTexture` sets for its draw (`01200bf6`,
/// `011fd86c`; their names are not confirmed).
const DRAWING_FLAG_01200BF6: u32 = 0x0120_0bf6;
const DRAWING_FLAG_011FD86C: u32 = 0x011f_d86c;
/// The player (`PlayerCharacter *` global) and `BGSEntryPoint::HandleEntryPoint`
/// (`005e58f0`, cdecl, variadic).
const PLAYER: u32 = 0x011d_ea3c;
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The reference's extra data list (`005d43c0`) and
/// `ExtraDataList::GetRefractionPropertyExtra` (`00422820`).
const EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
const GET_REFRACTION_PROPERTY: u32 = 0x0042_2820;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The occlusion queries: deleting destructor (`004a4410`, `RET 4`), `operator new`
/// (`00401000`), the constructor (`00c4ecf0`), the start (`00c4ed70`) and
/// `BSOcclusionQuery::End` (`00c4edb0`).
const QUERY_DELETE_004A4410: u32 = 0x004a_4410;
const OPERATOR_NEW: u32 = 0x0040_1000;
const QUERY_CONSTRUCT_00C4ECF0: u32 = 0x00c4_ecf0;
const QUERY_BEGIN_00C4ED70: u32 = 0x00c4_ed70;
const QUERY_END: u32 = 0x00c4_edb0;
/// The accumulator's `004a1040` (`thiscall`, `RET 4`), `00b4f450` (cdecl, one word),
/// `00b6b790` (no arguments) and the print function (`005b5e40`, cdecl).
const ACCUMULATOR_004A1040: u32 = 0x004a_1040;
const SHADER_MANAGER_00B4F450: u32 = 0x00b4_f450;
const SHADER_00B6B790: u32 = 0x00b6_b790;
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `"pPartialLimbQuery..."` (`01078ee8`) and `"pEntireLimbQuery..."` (`01078e90`).
const NO_PARTIAL_QUERY_TEXT: u32 = 0x0107_8ee8;
const NO_ENTIRE_QUERY_TEXT: u32 = 0x0107_8e90;

// Translated from 00800f30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00800f30 - the C++ exception-unwinding frame (FS:[0] around the new occlusion queries) is not translated.
/// `FOVATSEffectManager::DrawTargetToTexture` (Xbox PDB, `RET 0x18`: the texture, the
/// query mode, a `bool` and a word for the draw, a word, a `bool`). Nothing when the
/// target (+4) answers its slot +0x22c (with 0). Otherwise: saves the renderer's
/// background color (slot +0xb4 into a local `NiColorA`) and clears it (slot +0xac with
/// a zero color); with `004e9510`, stops the texture's offscreen render
/// (`00b6b8d0(word, BSRenderedTexture::StopOffscreen(texture))`); sets the two draw flags.
/// For a target that is an actor (slot +0x100): its alpha (slot +0x420, else 1.0, not
/// used further) and, when the player's entry point 0x45 gives a value above 0 and it has
/// a refraction extra, the extra's value is saved and the actor's slot +0x384 is called
/// with 0 and 0.0. With the occlusion flag (+0x118) and mode 2 (partial, +0x12c) or 1
/// (entire, +0x16c), the selected limb's query (+0x23c) is replaced by a new one (the old
/// one deleted) and started. Draws (`00801420` between `00b4f450(8)` and `00b4f450(0)`,
/// the accumulator's `004a1040` with 8 and 0), ends the started query (marking +0x119
/// for a partial one, or prints its missing-query message), runs `00b6b790` with the last
/// flag, restores the actor (slot +0x384 with 1 and the saved value), clears the draw
/// flags and restores the background color.
#[allow(clippy::too_many_arguments)]
pub fn fovats_effect_manager_draw_target_to_texture(
    e: &mut Engine,
    this: Ptr,
    texture: u32,
    mode: i32,
    flag: u8,
    word: u32,
    stop_word: u32,
    finish: u8,
) {
    let base = this.addr();
    let target = e.mem.u32(base + 4);
    if e.vcall(target, 0x22c, &args![0u32]).bool() {
        return;
    }
    let renderer = e.call(RENDERER_0043C4B0, &args![]).u32();
    // The frame below `EBP`: the saved color at `EBP - 0x3c`, the zero color at
    // `EBP - 0x4c`, the entry point's value at `EBP - 0x28`.
    e.with_stack(0x28, |e, frame| {
        let zero_color = frame.addr();
        let saved_color = frame.addr() + 0x10;
        let value = frame.addr() + 0x24;
        let zero = x87_store32(0.0);
        e.call(
            COLOR_A_INIT_00414430,
            &args![saved_color, zero, zero, zero, zero],
        );
        e.vcall(renderer, 0xb4, &args![saved_color]);
        let color = e
            .call(
                COLOR_A_INIT_00414430,
                &args![zero_color, zero, zero, zero, zero],
            )
            .u32();
        e.vcall(renderer, 0xac, &args![color]);
        if e.call(TEST_004E9510, &args![]).bool() {
            let stopped = e
                .call(RENDERED_TEXTURE_STOP_OFFSCREEN, &args![texture])
                .u32();
            e.call(SHADER_00B6B8D0, &args![stop_word, stopped]);
        }
        e.set_global(DRAWING_FLAG_01200BF6, 1u8);
        e.set_global(DRAWING_FLAG_011FD86C, 1u8);
        let occluding = e.mem.u8(base + 0x118);
        let mut started = false;
        let target = e.mem.u32(base + 4);
        let actor = if e.vcall(target, 0x100, &args![]).bool() {
            e.mem.u32(base + 4)
        } else {
            0
        };
        let _alpha = if actor != 0 {
            x87_store32(e.vcall(actor, 0x420, &args![]).f64())
        } else {
            x87_store32(1.0)
        };
        let mut extra = 0;
        e.mem.set_u32(value, x87_store32(0.0));
        let player = e.global::<u32>(PLAYER);
        e.call(HANDLE_ENTRY_POINT, &args![0x45u32, player, value]);
        if actor != 0 && e.call(EXTRA_LIST_005D43C0, &args![actor]).u32() != 0 {
            // FCOMP against 0.0; TEST AH,0x41; JNZ: only a value above 0.0.
            if x87_load32(e.mem.u32(value)) > e.global::<f64>(ZERO_DOUBLE) {
                let list = e.call(EXTRA_LIST_005D43C0, &args![actor]).u32();
                extra = e.call(GET_REFRACTION_PROPERTY, &args![list]).u32();
            }
        }
        let mut restore = false;
        let mut saved = x87_store32(0.0);
        if extra != 0 && actor != 0 {
            saved = x87_quiet_bits(e.mem.u32(extra + 0xc));
            restore = true;
            e.vcall(actor, 0x384, &args![0u32, x87_store32(0.0)]);
        }
        let query_slot = |e: &Engine, table: u32| {
            let limb = e.mem.u32(base + 0x23c);
            base.wrapping_add(table).wrapping_add(limb.wrapping_mul(4))
        };
        if occluding != 0 {
            let table = match mode {
                2 => Some(0x12cu32),
                1 => Some(0x16c),
                _ => None,
            };
            if let Some(table) = table {
                let old = e.mem.u32(query_slot(e, table));
                if old != 0 {
                    e.call(QUERY_DELETE_004A4410, &args![old, 1u32]);
                }
                let block = e.call(OPERATOR_NEW, &args![4u32]).u32();
                let query = if block != 0 {
                    e.call(QUERY_CONSTRUCT_00C4ECF0, &args![block]).u32()
                } else {
                    0
                };
                let slot = query_slot(e, table);
                e.mem.set_u32(slot, query);
                started = true;
                let query = e.mem.u32(query_slot(e, table));
                e.call(QUERY_BEGIN_00C4ED70, &args![query]);
            }
        }
        let accumulator = e.call(READ_WORD, &args![base + 0x2cc]).u32();
        e.call(ACCUMULATOR_004A1040, &args![accumulator, 8u32]);
        e.call(SHADER_MANAGER_00B4F450, &args![8u32]);
        fn_00801420(e, this, flag, word);
        e.call(SHADER_MANAGER_00B4F450, &args![0u32]);
        let accumulator = e.call(READ_WORD, &args![base + 0x2cc]).u32();
        e.call(ACCUMULATOR_004A1040, &args![accumulator, 0u32]);
        if started {
            if mode == 2 {
                let query = e.mem.u32(query_slot(e, 0x12c));
                if query != 0 {
                    e.call(QUERY_END, &args![query]);
                    let limb = e.mem.u32(base + 0x23c);
                    e.mem.set_u8(base.wrapping_add(0x119).wrapping_add(limb), 1);
                } else {
                    e.call(PRINT_MESSAGE, &args![NO_PARTIAL_QUERY_TEXT]);
                }
            } else if mode == 1 {
                let query = e.mem.u32(query_slot(e, 0x16c));
                if query != 0 {
                    e.call(QUERY_END, &args![query]);
                } else {
                    e.call(PRINT_MESSAGE, &args![NO_ENTIRE_QUERY_TEXT]);
                }
            }
        }
        if finish != 0 {
            e.call(SHADER_00B6B790, &args![]);
        }
        if restore && actor != 0 {
            e.vcall(actor, 0x384, &args![1u32, saved]);
        }
        e.set_global(DRAWING_FLAG_01200BF6, 0u8);
        e.set_global(DRAWING_FLAG_011FD86C, 0u8);
        e.vcall(renderer, 0xac, &args![saved_color]);
    });
}

// Translated from 00801420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`: a `bool` and a word): with mode 0 (+0) draws the single
/// target (`00801450` with both), otherwise the list of targets (`00801880`).
pub fn fn_00801420(e: &mut Engine, this: Ptr, flag: u8, word: u32) {
    if e.mem.u32(this.addr()) == 0 {
        e.call(DRAW_00801450, &args![this, u32::from(flag), word]);
    } else {
        e.call(DRAW_00801880, &args![this]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007f_fec0, fovats_effect_manager_constructor(Ptr) -> Ptr),
        entry!(0x0080_0170, fn_00800170(Ptr) -> Ptr),
        entry!(
            0x0080_0f30,
            fovats_effect_manager_draw_target_to_texture(Ptr, u32, i32, u8, u32, u32, u8)
        ),
        entry!(0x0080_1420, fn_00801420(Ptr, u8, u32)),
    ]
}

#[cfg(test)]
mod tests {
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
    fn fn_00800170_constructs_the_pointer() {
        let mut e = Engine::new();
        e.register(NI_POINTER_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_0170, &args![0x5000u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, NI_POINTER_CONSTRUCT), vec![vec![0x5004, 0]]);
    }

    #[test]
    fn fovats_effect_manager_constructor_sets_up_the_effects() {
        let mut e = Engine::new();
        for address in [
            NI_POINTER_CONSTRUCT,
            NI_POINTER_ASSIGN,
            VECTOR_CONSTRUCTOR_ITERATOR,
            BS_SHADER_MANAGER_GET_TEXTURE,
            ACCUMULATOR_STEP,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(SETTING_FLOAT_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(SETTING_INT_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(SETTING_STRING_VALUE, |_, _| Ret {
            eax: 0x0000_7100,
            ..Ret::default()
        });
        e.map(DISTORT_DURATION_SETTING, 8);
        e.mem.set_f32(DISTORT_DURATION_SETTING + 4, 0.75);
        e.map(DISTORT_INTERVAL_SETTING, 8);
        e.mem.set_i32(DISTORT_INTERVAL_SETTING + 4, 1000);
        e.register(GET_MILLISECONDS, |_, _| Ret {
            eax: 5000,
            ..Ret::default()
        });
        e.register(RAND, |_, _| Ret {
            eax: 2345,
            ..Ret::default()
        });
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(BS_SHADER_ACCUMULATOR_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(READ_WORD, |_, _| Ret {
            eax: 0x0000_8800,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x2d0);
        for i in 0..0x10 {
            e.mem.set_u8(this + 0x119 + i, 7);
            e.mem.set_u32(this + 0x16c + 4 * i, 7);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007f_fec0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![this + 0x14, 8, 0x20, 0x0080_0170, TARGET_DESTRUCTOR]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_CONSTRUCT),
            vec![
                vec![this + 8, 0],
                vec![this + 0x114, 0],
                vec![this + 0x2c4, 0],
                vec![this + 0x2cc, 0]
            ]
        );
        assert_eq!(e.mem.u8(this + 0x129), 1);
        assert_eq!(e.mem.f32(this + 0x290), 0.75);
        assert_eq!(e.mem.f32(this + 0x294), 5345.0);
        assert_eq!(
            calls_to(&e, BS_SHADER_MANAGER_GET_TEXTURE),
            vec![vec![0x7100, 1, this + 0x2c4, 0, 0]]
        );
        assert_eq!(e.mem.i32(this + 0x10), -1);
        let accumulator = calls_to(&e, BS_SHADER_ACCUMULATOR_CONSTRUCTOR)[0].clone();
        assert_eq!(accumulator[1..], [0x65, 1, 0x2f7]);
        assert!(calls_to(&e, NI_POINTER_ASSIGN).contains(&vec![this + 0x2cc, accumulator[0]]));
        assert_eq!(calls_to(&e, ACCUMULATOR_STEP), vec![vec![0x8800, 0]]);
        assert_eq!(e.mem.u32(this + 0x2c8), 1);
        assert_eq!(e.mem.u8(this + 0x128), 0);
        assert_eq!(e.mem.u8(this + 0x119 + 15), 0);
        assert_eq!(e.mem.u32(this + 0x16c + 60), 0);
    }
}

#[cfg(test)]
mod n051_tests {
    use super::*;

    const STUBBED: &[u32] = &[
        DRAW_00801450,
        DRAW_00801880,
        RENDERER_0043C4B0,
        COLOR_A_INIT_00414430,
        TEST_004E9510,
        RENDERED_TEXTURE_STOP_OFFSCREEN,
        SHADER_00B6B8D0,
        HANDLE_ENTRY_POINT,
        EXTRA_LIST_005D43C0,
        GET_REFRACTION_PROPERTY,
        QUERY_DELETE_004A4410,
        OPERATOR_NEW,
        QUERY_CONSTRUCT_00C4ECF0,
        QUERY_BEGIN_00C4ED70,
        QUERY_END,
        ACCUMULATOR_004A1040,
        SHADER_MANAGER_00B4F450,
        SHADER_00B6B790,
        PRINT_MESSAGE,
        READ_WORD,
    ];

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0120_0000, 0x1000);
        e.map(0x011f_d000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        for &addr in STUBBED {
            e.register(addr, |_, _| Ret::default());
        }
        let slots: Vec<u32> = (0..0x110).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    #[test]
    fn the_draw_picks_the_single_target_or_the_list() {
        let mut e = engine();
        let this = e.mem.alloc(0x300);
        e.call_log = Some(vec![]);
        e.call(0x0080_1420, &args![this, 1u32, 0x77u32]);
        assert_eq!(calls_to(&e, DRAW_00801450), vec![vec![this, 1, 0x77]]);
        e.mem.set_u32(this, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_1420, &args![this, 1u32, 0x77u32]);
        assert_eq!(calls_to(&e, DRAW_00801880), vec![vec![this]]);
        assert!(calls_to(&e, DRAW_00801450).is_empty());
    }

    #[test]
    fn drawing_a_target_with_an_occlusion_query() {
        let mut e = engine();
        let this = e.mem.alloc(0x300);
        let target = e.mem.alloc(4);
        e.mem.set_u32(target, VTABLE);
        e.mem.set_u32(this + 4, target);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, VTABLE);
        answer(&mut e, RENDERER_0043C4B0, renderer);
        e.register(COLOR_A_INIT_00414430, |_, a| a[0].into_ret());
        // A target that is an actor with a refraction extra the entry point turns on.
        e.register(slot(0x100), |_, _| 1u32.into_ret());
        answer(&mut e, EXTRA_LIST_005D43C0, 0x7000);
        let extra = e.mem.alloc(0x10);
        e.mem.set_f32(extra + 0xc, 0.75);
        answer(&mut e, GET_REFRACTION_PROPERTY, extra);
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            e.mem.set_f32(a[2], 1.0);
            Ret::default()
        });
        e.mem.set_u8(this + 0x118, 1);
        e.mem.set_u32(this + 0x23c, 2);
        e.mem.set_u32(this + 0x12c + 8, 0x7100);
        answer(&mut e, OPERATOR_NEW, 0x7200);
        answer(&mut e, QUERY_CONSTRUCT_00C4ECF0, 0x7300);
        e.call_log = Some(vec![]);
        e.call(
            0x0080_0f30,
            &args![this, 0x6000u32, 2i32, 1u32, 0x55u32, 0x66u32, 1u32],
        );
        assert_eq!(calls_to(&e, QUERY_DELETE_004A4410), vec![vec![0x7100, 1]]);
        assert_eq!(e.mem.u32(this + 0x12c + 8), 0x7300);
        assert_eq!(calls_to(&e, QUERY_BEGIN_00C4ED70), vec![vec![0x7300]]);
        assert_eq!(calls_to(&e, QUERY_END), vec![vec![0x7300]]);
        assert_eq!(e.mem.u8(this + 0x119 + 2), 1);
        assert_eq!(calls_to(&e, DRAW_00801450), vec![vec![this, 1, 0x55]]);
        assert_eq!(
            calls_to(&e, SHADER_MANAGER_00B4F450),
            vec![vec![8], vec![0]]
        );
        assert_eq!(
            calls_to(&e, slot(0x384)),
            vec![vec![target, 0, 0], vec![target, 1, 0.75f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, SHADER_00B6B790).len(), 1);
        let colors = calls_to(&e, slot(0xac));
        assert_eq!(colors.len(), 2);
        assert_eq!(colors[1][1], calls_to(&e, slot(0xb4))[0][1]);
        assert_eq!(e.global::<u8>(DRAWING_FLAG_01200BF6), 0);
        // Mode 1 without a query prints its message; a target that answers +0x22c is
        // skipped.
        answer(&mut e, QUERY_CONSTRUCT_00C4ECF0, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x0080_0f30,
            &args![this, 0x6000u32, 1i32, 0u32, 0x55u32, 0x66u32, 0u32],
        );
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![NO_ENTIRE_QUERY_TEXT]]
        );
        e.register(slot(0x22c), |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x0080_0f30,
            &args![this, 0x6000u32, 1i32, 0u32, 0x55u32, 0x66u32, 0u32],
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }
}
