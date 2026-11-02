//! `BSShader/bsshadermanager.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// CRT `sin` (`00eca0a0`): the argument is the x87 `ST0`, so a leading `f64`; the result is `ST0`.
const CRT_SIN: u32 = 0x00ec_a0a0;
/// CRT `cos` (`00ec9f70`), same convention.
const CRT_COS: u32 = 0x00ec_9f70;
/// The first `double` the argument is multiplied by.
const FIRST_FACTOR: u32 = 0x0101_1588;
/// The second `double` the argument is multiplied by.
const SECOND_FACTOR: u32 = 0x0102_3128;
/// The `double` angle the ratio is divided by `tan` of.
const REFERENCE_ANGLE: u32 = 0x010a_d6d8;
/// The `float` the result is stored in.
const RATIO_RESULT: u32 = 0x011a_dcc8;

// Translated from 00b54000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadermanager.cpp`). `cdecl`, one `float` stack word: scales it
/// by the doubles at `01011588` and `01023128` (stored as a `float`), takes
/// `sin / cos` of the result and of the double at `010ad6d8` (each `sin` and `cos` stored as
/// a `float` first) and stores the first quotient divided by the second in the `float` at
/// `011adcc8`. Computed on the x87 as the exe does (53-bit precision, NaN choice of the x87).
pub fn fn_00b54000(e: &mut Engine, value: f32) {
    let first_factor: f64 = e.global(FIRST_FACTOR);
    let second_factor: f64 = e.global(SECOND_FACTOR);
    let scaled = x87_store32(x87_mul(
        x87_mul(x87_load32(value.to_bits()), first_factor),
        second_factor,
    ));
    let angle = x87_load32(scaled);
    let sine = x87_quiet_bits(x87_store32(e.call(CRT_SIN, &args![angle]).f64()));
    let cosine = x87_store32(e.call(CRT_COS, &args![angle]).f64());
    let quotient = x87_quiet_bits(x87_store32(x87_div(x87_m32(sine), x87_load32(cosine))));

    let reference: f64 = e.global(REFERENCE_ANGLE);
    let reference_sine = x87_quiet_bits(x87_store32(e.call(CRT_SIN, &args![reference]).f64()));
    let reference_cosine = x87_store32(e.call(CRT_COS, &args![reference]).f64());
    let reference_quotient = x87_div(x87_m32(reference_sine), x87_load32(reference_cosine));
    let ratio = x87_div(x87_m32(quotient), reference_quotient);
    e.set_global(RATIO_RESULT, f32::from_bits(x87_store32(ratio)));
}

/// The table of shadow scene nodes, one word per index (`011f91c8`).
const SHADOW_SCENE_NODES: u32 = 0x011f_91c8;
/// The word `BSShaderManager::GetAccumulator` answers (`011f95ec`).
const ACCUMULATOR: u32 = 0x011f_95ec;
/// `ShadowSceneNode::SetFogProperty` (engine map, `00b54110`): `thiscall`, `RET 4`.
const SHADOW_SCENE_NODE_SET_FOG_PROPERTY: u32 = 0x00b5_4110;

/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`,
/// size) and deallocate (`00aa4060`, `thiscall`, block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The allocation `00aa13e0` (`cdecl`, size).
const NEW_OBJECT: u32 = 0x00aa_13e0;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The thread block's word at `+0x2b4` (the allocation context), set to 5 during the set-up.
const TLS_CONTEXT: u32 = 0x2b4;
/// The set-up's first-run byte (`011f91c6`).
const SET_UP_DONE_011F91C6: u32 = 0x011f_91c6;
/// The objects the first run creates and the globals it reads.
const OBJECT_011F91A8: u32 = 0x011f_91a8;
const OBJECT_011F9498: u32 = 0x011f_9498;
const OBJECT_011F91AC: u32 = 0x011f_91ac;
const HOLDER_011F94BC: u32 = 0x011f_94bc;
const GLOBAL_011F9508: u32 = 0x011f_9508;
/// The four floats set to (0.5, 0.5, 0.5, 1.0) on the first run, and the `float` 0.5 the
/// code loads (`01016248`).
const FLOATS_011FA090: u32 = 0x011f_a090;
const HALF_01016248: u32 = 0x0101_6248;
/// The string the first run's holder is made from (`010ada3c`, passed to `00a5b690`).
const HOLDER_NAME_010ADA3C: u32 = 0x010a_da3c;
/// The accumulator array: count (`011f916c`) and pointer (`011f9170`); the word cleared at
/// the end (`011f9168`); the word copied (`011f94f0` into `011a9608`).
const ACCUMULATOR_COUNT_011F916C: u32 = 0x011f_916c;
const ACCUMULATOR_ARRAY_011F9170: u32 = 0x011f_9170;
const GLOBAL_011F9168: u32 = 0x011f_9168;
const GLOBAL_011F94F0: u32 = 0x011f_94f0;
const GLOBAL_011A9608: u32 = 0x011a_9608;
const GLOBAL_011FFE2C: u32 = 0x011f_fe2c;
const GLOBAL_011FFE30: u32 = 0x011f_fe30;

/// The function-pointer words `00b57420` clears, in the exe's order.
const CLEARED_WORDS_00B57420: [u32; 32] = [
    0x011f_9f40,
    0x011f_9f68,
    0x011f_9f70,
    0x011f_9f74,
    0x011f_9f78,
    0x011f_9f80,
    0x011f_9fa0,
    0x011f_9fa4,
    0x011f_9fa8,
    0x011f_9fac,
    0x011f_9fb4,
    0x011f_9fb8,
    0x011f_9f44,
    0x011f_9f48,
    0x011f_9f4c,
    0x011f_9f50,
    0x011f_9f54,
    0x011f_9f58,
    0x011f_9f5c,
    0x011f_9f60,
    0x011f_9f64,
    0x011f_9f6c,
    0x011f_9f7c,
    0x011f_9f84,
    0x011f_9f88,
    0x011f_9f8c,
    0x011f_9f90,
    0x011f_9f94,
    0x011f_9f98,
    0x011f_9f9c,
    0x011f_9fb0,
    0x011f_9fbc,
];

/// The (word, code address) stores `00b57420` makes after clearing, in the exe's order
/// (immediates of its `MOV` instructions).
const FUNCTION_WORDS_00B57420: [(u32, u32); 14] = [
    (0x011f_9f80, 0x00b6_4020),
    (0x011f_9f3c, 0x00b6_4020),
    (0x011f_9fa8, 0x00b6_5a90),
    (0x011f_9fac, 0x00b6_06e0),
    (0x011f_9fb4, 0x00b6_3f90),
    (0x011f_9fa4, 0x00b6_4440),
    (0x011f_9fb8, 0x00b6_44b0),
    (0x011f_9fa0, 0x00b6_4490),
    (0x011f_9f40, 0x00b6_65a0),
    (0x011f_9f70, 0x00b6_5dc0),
    (0x011f_9f68, 0x00b6_5e80),
    (0x011f_9f74, 0x00a2_9680),
    (0x011f_9f78, 0x00b6_4530),
    (0x011f_9f38, 0x00b6_65a0),
];

// Translated from 0070b760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `BSShaderManager::SetShadowSceneNode` in the engine map (Xbox PDB), but the body is a
/// `thiscall` on the node itself (`RET 4`): it stores the index byte at `+0x120` of `this` and
/// then puts `this` into the shadow-scene-node table at that index (`00b4f2f0`).
pub fn fn_0070b760(e: &mut Engine, this: Ptr, index: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x120), index);
    fn_00b4f2f0(e, u32::from(index), this);
}

// Translated from 00b4f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadermanager.cpp`). `cdecl` (index, node): stores the node in
/// the shadow-scene-node table at `011f91c8 + 4 * index` (no bounds check).
pub fn fn_00b4f2f0(e: &mut Engine, index: u32, node: Ptr) {
    e.mem.set_u32(
        SHADOW_SCENE_NODES.wrapping_add(index.wrapping_mul(4)),
        node.addr(),
    );
}

// Translated from 00b4f5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::GetAccumulator` (Xbox PDB): the word at `011f95ec`.
pub fn bs_shader_manager_get_accumulator(e: &mut Engine) -> Ptr {
    Ptr::new(e.global::<u32>(ACCUMULATOR))
}

// Translated from 00b55540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::SetFogProperty` (Xbox PDB). `cdecl` (index, fog property): calls
/// `ShadowSceneNode::SetFogProperty` (`00b54110`) on the shadow scene node at that index of the
/// table (`011f91c8`) with the fog property.
pub fn bs_shader_manager_set_fog_property(e: &mut Engine, index: u32, fog: Ptr) {
    let node = e
        .mem
        .u32(SHADOW_SCENE_NODES.wrapping_add(index.wrapping_mul(4)));
    e.call(SHADOW_SCENE_NODE_SET_FOG_PROPERTY, &args![node, fog]);
}

/// `MemoryManager::Allocate` on the singleton.
fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

/// The first-run object at `011f9498` released: `00b7fd50` on it, then the memory manager
/// frees it and the global is cleared.
fn release_object_011f9498(e: &mut Engine, object: u32) {
    e.call(0x00b7_fd50, &args![object]);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, object]);
    e.set_global(OBJECT_011F9498, 0u32);
}

// Translated from 00b57420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadermanager.cpp`); no arguments. The shader manager's
/// set-up, with the thread block's allocation context (`+0x2b4`) at 5 and restored at the end:
///
/// * On the first run (byte `011f91c6` clear): makes the 0xb0-byte object at `011f91a8`
///   (`00b6dd80`) and the 0x1c-byte one at `011f9498` (`00b7ff40` with `00b7faf0`'s value),
///   runs `00b7fe40` on the latter with its first word (read even when the allocation
///   failed) and releases it (`00b7fd50`, free) when its word at `+4` is 0; runs `00b55bb0`;
///   sets the floats at `011fa090` to 0.5, 0.5, 0.5 (the `float` at `01016248`) and 1.0; when
///   `011f9508` is not 0, gets shaders 1 to 0x22 (`BSShaderManager::GetShader`, `00b55560`),
///   makes the 0x31c-byte object at `011f91ac` (`00b97a80`) and releases `011f9498` if it is
///   still there; registers the streamables (`00b4f1a0`), runs `00b6e450` on `011f91a8` with
///   `011f9508`, makes a one-word holder of `00a5b690(010ada3c)` at `011f94bc` and sets the
///   first-run byte.
/// * Every run: clears 32 function-pointer words of the `011f9f38` block and stores 14 code
///   addresses into it, clears `011ffe2c` and `011ffe30`, runs `MTRenderingSystem::CreateThreads`
///   (`00ba3720`) and `00b97c10`, copies `011f94f0` to `011a9608`, then makes the array of
///   `011f916c` accumulator pointers (a count word before the elements, the elements
///   constructed by `00ec782f` with `00b54620`/`0045cec0`) at `011f9170` and fills each slot
///   with a new `BSShaderAccumulator` (`00b660d0(0x63, 1, 0x2f7)` on 0x280 bytes) through the
///   `NiPointer` assignment (the old one released, deleted through its vtable `+4` when its
///   count reaches 0), and clears `011f9168`.
///
/// The `ECX` the exe loads before `CreateThreads` (`01200088`) is not read by it.
// Translation incomplete: 00b57420 - the C++ exception frame (FS:[0] chain, unwind states 1 to 6) is not translated.
pub fn fn_00b57420(e: &mut Engine) {
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), 5);

    if e.global::<u8>(SET_UP_DONE_011F91C6) == 0 {
        let memory = allocate(e, 0xb0);
        let object = if memory != 0 {
            e.call(0x00b6_dd80, &args![memory]).u32()
        } else {
            0
        };
        e.set_global(OBJECT_011F91A8, object);

        let memory = allocate(e, 0x1c);
        let object = if memory != 0 {
            let value = e.call(0x00b7_faf0, &args![]).u32();
            e.call(0x00b7_ff40, &args![memory, value]).u32()
        } else {
            0
        };
        e.set_global(OBJECT_011F9498, object);
        let first = e.mem.u32(object);
        e.call(0x00b7_fe40, &args![object, first]);
        let object = e.global::<u32>(OBJECT_011F9498);
        if e.mem.u32(object.wrapping_add(4)) == 0 {
            release_object_011f9498(e, object);
        }

        e.call(0x00b5_5bb0, &args![]);
        let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
        e.set_global(FLOATS_011FA090, half);
        e.set_global(FLOATS_011FA090 + 4, half);
        e.set_global(FLOATS_011FA090 + 8, half);
        e.set_global(FLOATS_011FA090 + 0xc, 1.0f32.to_bits());

        if e.global::<u32>(GLOBAL_011F9508) != 0 {
            for shader in 1u32..0x23 {
                e.call(0x00b5_5560, &args![shader]);
            }
            let memory = allocate(e, 0x31c);
            let object = if memory != 0 {
                e.call(0x00b9_7a80, &args![memory]).u32()
            } else {
                0
            };
            e.set_global(OBJECT_011F91AC, object);
            let object = e.global::<u32>(OBJECT_011F9498);
            if object != 0 {
                release_object_011f9498(e, object);
            }
        }

        e.call(0x00b4_f1a0, &args![]);
        let word = e.global::<u32>(GLOBAL_011F9508);
        let object = e.global::<u32>(OBJECT_011F91A8);
        e.call(0x00b6_e450, &args![object, word]);

        let holder = allocate(e, 4);
        let holder = if holder != 0 {
            let text = e.call(0x00a5_b690, &args![HOLDER_NAME_010ADA3C]).u32();
            e.mem.set_u32(holder, text);
            holder
        } else {
            0
        };
        e.set_global(HOLDER_011F94BC, holder);
        e.set_global(SET_UP_DONE_011F91C6, 1u8);
    }

    for word in CLEARED_WORDS_00B57420 {
        e.set_global(word, 0u32);
    }
    for (word, code) in FUNCTION_WORDS_00B57420 {
        e.set_global(word, code);
    }
    e.set_global(GLOBAL_011FFE2C, 0u32);
    e.set_global(GLOBAL_011FFE30, 0u32);
    // ECX at 00b57750 is the rendering system 01200088 (MOV ECX at 00b576d1), CreateThreads's `this`.
    e.call(0x00ba_3720, &args![0x0120_0088u32]);
    e.call(0x00b9_7c10, &args![]);
    let copied = e.global::<u32>(GLOBAL_011F94F0);
    let count = e.global::<u32>(ACCUMULATOR_COUNT_011F916C);
    e.set_global(GLOBAL_011A9608, copied);

    // `count * 4 + 4` for the count word, all ones on overflow at either step
    // (`MUL; SETO; NEG; OR` then `ADD; SETC; NEG; OR`).
    let size = count
        .checked_mul(4)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX);
    let memory = allocate(e, size);
    let array = if memory != 0 {
        e.mem.set_u32(memory, count);
        let elements = memory.wrapping_add(4);
        e.call(
            0x00ec_782f,
            &args![elements, 4u32, count, 0x00b5_4620u32, 0x0045_cec0u32],
        );
        elements
    } else {
        0
    };
    e.set_global(ACCUMULATOR_ARRAY_011F9170, array);

    let mut index = 0u32;
    if e.global::<u32>(ACCUMULATOR_COUNT_011F916C) != 0 {
        loop {
            let memory = e.call(NEW_OBJECT, &args![0x280u32]).u32();
            let accumulator = if memory != 0 {
                e.call(0x00b6_60d0, &args![memory, 0x63u32, 1u32, 0x2f7u32])
                    .u32()
            } else {
                0
            };
            let array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
            let slot = array.wrapping_add(index.wrapping_mul(4));
            let old = e.mem.u32(slot);
            if old != accumulator {
                if old != 0 {
                    let left = e
                        .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                        .u32();
                    if left == 0 {
                        e.vcall(old, 4, &args![]);
                    }
                }
                e.mem.set_u32(slot, accumulator);
                if accumulator != 0 {
                    e.call(
                        IMPORT_INTERLOCKED_INCREMENT,
                        &args![accumulator.wrapping_add(4)],
                    );
                }
            }
            index = index.wrapping_add(1);
            if index >= e.global::<u32>(ACCUMULATOR_COUNT_011F916C) {
                break;
            }
        }
    }
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    e.set_global(GLOBAL_011F9168, 0u32);
}

/// The shader table `BSShaderManager::GetShader` fills, one `NiPointer` per index 0 to 0x22
/// (`011f9548`).
const SHADER_TABLE_011F9548: u32 = 0x011f_9548;
/// The highest shader index `GetShader` accepts.
const LAST_SHADER_INDEX: u32 = 0x22;
/// The byte that selects shader 0x1d's creator (`011f94a6`).
const SHADER_1D_SELECT_011F94A6: u32 = 0x011f_94a6;
/// The `NiPointer` assignment `00a5f860` (`thiscall` on the slot, `RET 4`: the new object).
const NI_POINTER_ASSIGN_00A5F860: u32 = 0x00a5_f860;
/// Shader 4's creator (`00bd4750`), which runs after shader 1 is made, and shader 0x1d's
/// when the byte at `011f94a6` is set (`00bbd0d0`).
const SHADER_4_CREATE: u32 = 0x00bd_4750;
const SHADER_1D_CREATE: u32 = 0x00bb_d0d0;

/// The creator `GetShader` calls for a shader index (no stack words, the shader in `EAX`):
/// the arms of its switch, read from the jump table at `00b55748` in Ghidra. Indices without
/// a creator (0, 0xb, 0xc, 0x12 to 0x17, 0x1b, 0x1c, 0x1f) make nothing; 4 and 0x1d are
/// handled by the caller.
fn shader_creator(index: u32) -> Option<u32> {
    match index {
        1 => Some(0x00b7_a380),
        2 => Some(0x00ba_cf00),
        3 => Some(0x00bd_5920),
        5 => Some(0x00bd_2e70),
        6 => Some(0x00bb_2c00),
        7 => Some(0x00b8_0e70),
        8 => Some(0x00bb_6e30),
        9 => Some(0x00bd_23f0),
        0xa => Some(0x00b8_a390),
        0xd => Some(0x00bd_14b0),
        0xe => Some(0x00bc_fb00),
        0xf => Some(0x00bc_d680),
        0x10 => Some(0x00bc_c0a0),
        0x11 => Some(0x00b8_6de0),
        0x18 => Some(0x00ba_58b0),
        0x19 => Some(0x00bc_10e0),
        0x1a => Some(0x00bc_4af0),
        0x1e => Some(0x00bb_0660),
        0x20 => Some(0x00bc_ae90),
        0x21 => Some(0x00bc_93c0),
        0x22 => Some(0x00bc_59b0),
        _ => None,
    }
}

/// An `NiPointer`'s release: the count at +4 decremented, the object deleted through its
/// vtable slot +4 when it reaches 0.
fn release_ni_pointer(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00b55560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::GetShader` (Xbox PDB). `cdecl`, one word (the shader index). Answers 0
/// when the word at `011f9508` is 0 or the index is above 0x22; otherwise the shader in the
/// table at `011f9548`, making it first when the slot is empty: with the thread block's
/// allocation context (`+0x2b4`) at 5, the index's creator runs (shader 4 first gets shader
/// 1 through this function) and its result is assigned to the slot (`00a5f860`); for index
/// 0x1d the creator is `00bbd0d0` when the byte at `011f94a6` is set, else the slot's
/// shader is released and the slot cleared (the slot is empty on that path, so nothing is
/// released). The context is restored before answering the slot's word.
// Translation incomplete: 00b55560 - the C++ exception frame (FS:[0] chain, unwind state 0 around the creation) is not translated.
pub fn bs_shader_manager_get_shader(e: &mut Engine, index: u32) -> Ptr {
    if e.global::<u32>(GLOBAL_011F9508) == 0 || index > LAST_SHADER_INDEX {
        return Ptr::new(0);
    }
    let slot = SHADER_TABLE_011F9548.wrapping_add(index.wrapping_mul(4));
    if e.mem.u32(slot) != 0 {
        return Ptr::new(e.mem.u32(slot));
    }
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), 5);
    let created = match index {
        4 => {
            bs_shader_manager_get_shader(e, 1);
            Some(e.call(SHADER_4_CREATE, &args![]).u32())
        }
        0x1d => {
            if e.global::<u8>(SHADER_1D_SELECT_011F94A6) != 0 {
                Some(e.call(SHADER_1D_CREATE, &args![]).u32())
            } else {
                let held = e.mem.u32(slot);
                if held != 0 {
                    release_ni_pointer(e, held);
                    e.mem.set_u32(slot, 0);
                    let shader = e.mem.u32(slot);
                    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
                    return Ptr::new(shader);
                }
                None
            }
        }
        _ => shader_creator(index).map(|create| e.call(create, &args![]).u32()),
    };
    if let Some(shader) = created {
        e.call(NI_POINTER_ASSIGN_00A5F860, &args![slot, shader]);
    }
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    Ptr::new(e.mem.u32(slot))
}

/// `NiStream::RegisterLoader` (engine map, `00a64900`, `cdecl`: class name, loader).
const NI_STREAM_REGISTER_LOADER: u32 = 0x00a6_4900;

/// The (class name, loader) pairs `RegisterStreamables` registers, in the exe's order
/// (immediates of its `PUSH` instructions; the names are the exe's strings at those
/// addresses, read in Ghidra).
const STREAMABLES_00B4F1A0: [(u32, u32); 20] = [
    // "BSShaderProperty"
    (0x010a_93a0, 0x00ba_9250),
    // "BSShaderLightingProperty"
    (0x010a_9384, 0x00b7_1920),
    // "BSShaderPPLightingProperty"
    (0x010a_9368, 0x00b6_8d50),
    // "Lighting30ShaderProperty"
    (0x010a_934c, 0x00bb_45d0),
    // "HairShaderProperty"
    (0x010a_9338, 0x00bc_4340),
    // "NoFaderShaderProperty"
    (0x010a_9320, 0x00bb_45d0),
    // "WaterShaderProperty"
    (0x010a_930c, 0x00b6_ae60),
    // "BSShaderNoLightingProperty"
    (0x010a_92f0, 0x00b6_fe80),
    // "VolumetricFogShaderProperty"
    (0x010a_92d4, 0x00bc_38c0),
    // "SkyShaderProperty"
    (0x010a_92c0, 0x00ba_ee40),
    // "TallGrassShaderProperty"
    (0x010a_92a8, 0x00b6_0f50),
    // "BSTextureData"
    (0x010a_9298, 0x00c5_eed0),
    // "BSShaderTextureSet"
    (0x010a_9284, 0x00ba_8a00),
    // "BSRefractionStrengthController"
    (0x010a_9264, 0x00bc_36a0),
    // "BSRefractionFirePeriodController"
    (0x010a_9240, 0x00bc_33f0),
    // "TileShaderProperty"
    (0x010a_922c, 0x00bb_7ef0),
    // "BSMaterialEmittanceMultController"
    (0x010a_9208, 0x00bc_3190),
    // "BSFadeNode"
    (0x010a_9194, 0x00b4_eaa0),
    // "BSOrderedNode"
    (0x010a_91f8, 0x00bc_2cb0),
    // "BSDistantTreeShaderProperty"
    (0x010a_91dc, 0x00bb_7300),
];

// Translated from 00b4f1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::RegisterStreamables` (Xbox PDB). No arguments: registers 20 stream
/// loaders with `NiStream::RegisterLoader` (`00a64900`), each a class name and the code
/// address of its loader (the same loader `00bb45d0` serves two names).
pub fn bs_shader_manager_register_streamables(e: &mut Engine) {
    for (name, loader) in STREAMABLES_00B4F1A0 {
        e.call(NI_STREAM_REGISTER_LOADER, &args![name, loader]);
    }
}

/// `BSFogProperty::BSFogProperty` (engine map, `00bb8180`, `thiscall`, no stack words).
const BS_FOG_PROPERTY_CONSTRUCT: u32 = 0x00bb_8180;
/// The `float` the default fog's four distances are loaded from (`010ad6e0`).
const FOG_DISTANCE_010AD6E0: u32 = 0x010a_d6e0;

// Translated from 00b54110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::SetFogProperty` (Xbox PDB). `thiscall`, `RET 4`, one word (the fog
/// property). With a fog property: assigns it to the node's `spFog` (+0x134; the old one
/// released, the new one's count incremented, nothing when they are the same). Without one:
/// makes a 0x64-byte `BSFogProperty` (`00aa13e0`, then its constructor `00bb8180` when the
/// allocation succeeded), assigns it the same way (its count incremented when not null),
/// then sets on the node's fog: `fStartDistance`, `fEndDistance` and the floats at +0x34 and
/// +0x38 (`fStartWaterDistance`, `fEndWaterDistance` in the Xbox PDB) from the `float` at
/// `010ad6e0`, `fPower` 1.0, and the colours at +0x20 (`m_kColor`) and +0x54 (`kWaterColor`)
/// to the `float` at `01016248` in each of their three words, in the exe's order.
// Translation incomplete: 00b54110 - the C++ exception frame (FS:[0] chain, unwind state 0 around the constructor) is not translated.
pub fn shadow_scene_node_set_fog_property(e: &mut Engine, this: Ptr, fog: Ptr) {
    use crate::units::bsshader::bsfogproperty::BSFogProperty;
    // `ShadowSceneNode::spFog` (Xbox PDB name, PC offset): `NiPointer<BSFogProperty>`.
    let field = this.addr().wrapping_add(0x134);
    let fog = fog.addr();
    if fog != 0 {
        let old = e.mem.u32(field);
        if old != fog {
            if old != 0 {
                release_ni_pointer(e, old);
            }
            e.mem.set_u32(field, fog);
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![fog.wrapping_add(4)]);
        }
        return;
    }
    let memory = e.call(NEW_OBJECT, &args![0x64u32]).u32();
    let made = if memory != 0 {
        e.call(BS_FOG_PROPERTY_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    let old = e.mem.u32(field);
    if old != made {
        if old != 0 {
            release_ni_pointer(e, old);
        }
        e.mem.set_u32(field, made);
        if made != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
        }
    }
    let property = Ptr::<BSFogProperty>::new(e.mem.u32(field));
    let distance = f32::from_bits(x87_quiet_bits(e.global::<u32>(FOG_DISTANCE_010AD6E0)));
    e.set(property, BSFogProperty::fStartDistance, distance);
    e.set(property, BSFogProperty::fEndDistance, distance);
    e.set(
        property,
        BSFogProperty::fPower,
        f32::from_bits(x87_store32(1.0)),
    );
    // `fStartWaterDistance`, `fEndWaterDistance` (Xbox PDB, `BSFogProperty`).
    e.mem.set_f32(property.addr().wrapping_add(0x34), distance);
    e.mem.set_f32(property.addr().wrapping_add(0x38), distance);
    let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
    // `m_kColor` (Xbox PDB, `NiFogProperty`), three floats.
    for word in 0..3u32 {
        e.mem
            .set_u32(property.addr().wrapping_add(0x20 + 4 * word), half);
    }
    // `kWaterColor` (Xbox PDB, `BSFogProperty`), three floats; the node's word read again.
    let property = e.mem.u32(field);
    for word in 0..3u32 {
        e.mem.set_u32(property.wrapping_add(0x54 + 4 * word), half);
    }
}

/// `NiPixelData::NiPixelData_ov2` (engine map, `00a7c190`): `thiscall`, five stack words
/// (width, height, the address of a pixel format, mipmap levels, faces).
const NI_PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
/// `NiPixelData` fields (Xbox PDB names, the PC offsets the code uses): `m_pucPixels`
/// (+0x50), `m_puiOffsetInBytes` (+0x5c), `m_uiMipmapLevels` (+0x60), `m_uiRevID` (+0x68),
/// `bNoConvert` (+0x70).
const PIXELS: u32 = 0x50;
const OFFSETS_IN_BYTES: u32 = 0x5c;
const MIPMAP_LEVELS: u32 = 0x60;
const REVISION: u32 = 0x68;
const NO_CONVERT: u32 = 0x70;
/// The pixel format copied onto the stack for the cube (0x11 words at `011aaac8`) and the
/// two formats of the small textures (`011aa2a0`, four bytes a pixel in the fills;
/// `011aa2e8`, three).
const CUBE_PIXEL_FORMAT_011AAAC8: u32 = 0x011a_aac8;
const PIXEL_FORMAT_011AA2A0: u32 = 0x011a_a2a0;
const PIXEL_FORMAT_011AA2E8: u32 = 0x011a_a2e8;
/// The texture format preferences passed to the texture constructors (`011a9598`).
const FORMAT_PREFS_011A9598: u32 = 0x011a_9598;
/// The `NiFixedString` maker (`00a5b690`, `cdecl`: text) and the word a string is compared
/// with before its count (8 bytes before its text) is decremented (`0109b220`).
const FIXED_STRING_MAKE: u32 = 0x00a5_b690;
const FIXED_STRING_NONE: u32 = 0x0109_b220;
/// `NiSourceTexture::Create_ov2` (engine map, `00a5ff10`, `cdecl`: pixel data, name,
/// format preferences) and `NiSourceTexture::Create` (`00a5fd70`, `cdecl`: the address of
/// the name, format preferences, two words).
const SOURCE_TEXTURE_FROM_PIXELS: u32 = 0x00a5_ff10;
const SOURCE_TEXTURE_FROM_FILE: u32 = 0x00a5_fd70;
/// `BSShaderManager::GetTexture` (engine map, `00b55840`, `cdecl`, five words).
const BS_SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;
/// The cube map maker of `NiMain/nisourcecubemap.cpp` (`00a61120`, `cdecl`: six file
/// names, the renderer, the address of the name, format preferences).
const SOURCE_CUBE_MAP_CREATE: u32 = 0x00a6_1120;
/// The formatter `00406d00` (`cdecl`: buffer, size, format, then the format's words).
const FORMAT_00406D00: u32 = 0x0040_6d00;
/// The CRT square root `00ec6040` (argument and result in `ST0`) and the CRT function
/// `00ec6940` (`cdecl`, one `double`; result in `ST0`).
const CRT_SQRT_00EC6040: u32 = 0x00ec_6040;
const CRT_00EC6940: u32 = 0x00ec_6940;
/// `004dc060` (engine map: `fallout shared/renderer.cpp`, `cdecl`, one word).
const RENDERER_QUERY_004DC060: u32 = 0x004d_c060;

/// The constants of the generated textures, read from the exe as the code reads them:
/// the `double`s 64.0 (`010240c0`), -64.0 (`0106b9f0`), 4096.0 (`01017a10`), 0.5
/// (`01011588`), 255.0 (`0101e568`), 63.5 (`010ad890`) and 0.0 (`01012060`), and the
/// `float` 128.0 the loops count to (`0101e704`); the values are what the exe holds today.
const CUBE_HALF_SIZE_010240C0: u32 = 0x0102_40c0;
const CUBE_MINUS_HALF_SIZE_0106B9F0: u32 = 0x0106_b9f0;
const CUBE_DEPTH_SQUARED_01017A10: u32 = 0x0101_7a10;
const HALF_01011588: u32 = 0x0101_1588;
const BYTE_SCALE_0101E568: u32 = 0x0101_e568;
const ATTENUATION_CENTRE_010AD890: u32 = 0x010a_d890;
const ZERO_01012060: u32 = 0x0101_2060;
const LOOP_END_0101E704: u32 = 0x0101_e704;

/// The texture globals `00b55bb0` fills (each an `NiPointer`).
const TEXTURE_011F9510: u32 = 0x011f_9510;
const TEXTURE_011F9514: u32 = 0x011f_9514;
const TEXTURE_011F9518: u32 = 0x011f_9518;
const TEXTURE_011F951C: u32 = 0x011f_951c;
const TEXTURE_011F9520: u32 = 0x011f_9520;
const TEXTURE_011F9524: u32 = 0x011f_9524;
const TEXTURE_011F9528: u32 = 0x011f_9528;
const TEXTURE_011F952C: u32 = 0x011f_952c;
const TEXTURE_011F9530: u32 = 0x011f_9530;
const TEXTURE_011F9534: u32 = 0x011f_9534;
const TEXTURE_011F9538: u32 = 0x011f_9538;
const TEXTURE_011F953C: u32 = 0x011f_953c;
const TEXTURE_011F9540: u32 = 0x011f_9540;
const TEXTURE_011F9544: u32 = 0x011f_9544;
const TEXTURE_011F95D4: u32 = 0x011f_95d4;

/// `00b55bb0`'s stack block, by the offsets the exe uses after its prologue: the fixed
/// string whose address the cube maps and file textures get (+0x20), the format
/// preferences of the last texture (+0x44), the cube's pixel format (+0x6c, 0x44 bytes) and
/// the 0x104-byte file name buffer (+0xb4).
const FRAME_NAME: u32 = 0x20;
const FRAME_PREFS: u32 = 0x44;
const FRAME_PIXEL_FORMAT: u32 = 0x6c;
const FRAME_BUFFER: u32 = 0xb4;
const FRAME_SIZE: u32 = 0x1b8;

/// `FISTP` with the rounding control set to truncate (the code ORs 0xc00 into the control
/// word around it): toward zero, the integer indefinite 0x80000000 for NaN and values out
/// of range.
fn fistp_truncate(value: f64) -> i32 {
    let truncated = value.trunc();
    if truncated.is_nan() || truncated < -2_147_483_648.0 || truncated > 2_147_483_647.0 {
        i32::MIN
    } else {
        truncated as i32
    }
}

/// One colour byte of the cube: `(value + 1) * 0.5 * 255` truncated, the low byte kept;
/// `one_first` for the third byte, whose addition the code makes as `1 + value`.
fn cube_byte(value_bits: u32, one_first: bool, half: f64, scale: f64) -> u8 {
    let value = x87_load32(value_bits);
    let sum = if one_first {
        x87_add(1.0, value)
    } else {
        x87_add(value, 1.0)
    };
    fistp_truncate(x87_mul(x87_mul(sum, half), scale)) as u8
}

/// The constants one cube face needs.
struct CubeConstants {
    half_size: f64,
    minus_half_size: f64,
    depth_squared: f64,
    half: f64,
    scale: f64,
}

/// One 128x128 face of the normalization cube `00b55bb0` computes: for each row and column
/// (counted as floats to 128.0), the direction from the cube's centre to the texel, made
/// unit length (`1 / sqrt` of the squared length through `00ec6040`), each component mapped
/// to a byte, then a zero byte. The six faces differ in which offsets they use, the order
/// they add the squares in and where each component goes; each arm follows its loop in the
/// exe (`00b55c57`, `00b55df4`, `00b55fa4`, `00b56145`, `00b562f6`, `00b56493`).
fn normal_cube_face(e: &mut Engine, face: u32, mut pixel: u32, k: &CubeConstants) {
    let k64 = k.half_size;
    let mut row = 0i32;
    let mut row_bits = x87_store32(0.0);
    loop {
        let y = x87_load32(row_bits);
        // The row's offset, stored as a float once per row.
        let row_value = match face {
            2 => x87_store32(x87_sub(y, k64)),
            _ => x87_store32(x87_sub(k64, y)),
        };
        let mut column = 0i32;
        let mut column_bits = x87_store32(0.0);
        loop {
            let x = x87_load32(column_bits);
            let column_value = match face {
                0 | 5 => x87_store32(x87_sub(k64, x)),
                _ => x87_store32(x87_sub(x, k64)),
            };
            let row_copy = x87_quiet_bits(row_value);
            // (first, second): the offset the code loads first and the other.
            let (first, second) = match face {
                0 | 1 | 4 | 5 => (row_copy, column_value),
                _ => (column_value, row_copy),
            };
            let first_value = x87_load32(first);
            let second_value = x87_load32(second);
            let first_square = x87_mul(first_value, first_value);
            let second_square = x87_mul(second_value, second_value);
            let length_squared = if face >= 4 {
                x87_add(x87_add(first_square, second_square), k.depth_squared)
            } else {
                x87_add(second_square, x87_add(first_square, k.depth_squared))
            };
            let length_squared = x87_store32(length_squared);
            let length = e
                .call(CRT_SQRT_00EC6040, &args![x87_load32(length_squared)])
                .f64();
            let length = x87_load32(x87_store32(length));
            let reciprocal = x87_load32(x87_store32(x87_div(1.0, length)));
            let depth = x87_store32(match face {
                1 | 3 => x87_mul(k.minus_half_size, reciprocal),
                5 => x87_mul(reciprocal, k.minus_half_size),
                _ => x87_mul(reciprocal, k64),
            });
            let first_scaled = x87_store32(x87_mul(reciprocal, first_value));
            let second_scaled = x87_store32(x87_mul(reciprocal, second_value));
            let bytes = match face {
                0 | 1 => [second_scaled, first_scaled, depth],
                2 | 3 => [second_scaled, depth, first_scaled],
                _ => [depth, first_scaled, second_scaled],
            };
            for (index, value) in bytes.iter().enumerate() {
                let byte = cube_byte(*value, index == 2, k.half, k.scale);
                e.mem.set_u8(pixel.wrapping_add(index as u32), byte);
            }
            e.mem.set_u8(pixel.wrapping_add(3), 0);
            pixel = pixel.wrapping_add(4);
            column = column.wrapping_add(1);
            column_bits = x87_store32(f64::from(column));
            if !(x87_load32(column_bits) < x87_m32(e.global::<u32>(LOOP_END_0101E704))) {
                break;
            }
        }
        row = row.wrapping_add(1);
        row_bits = x87_store32(f64::from(row));
        if !(x87_load32(row_bits) < x87_m32(e.global::<u32>(LOOP_END_0101E704))) {
            break;
        }
    }
}

/// The address of face `face` of a pixel data's first mipmap level: its pixels plus the
/// level's offset plus `face` times the offset after the last level (face 0 reads neither
/// the level count nor that offset).
fn cube_face_pixels(e: &Engine, pixel_data: u32, face: u32) -> u32 {
    let offsets = e.mem.u32(pixel_data.wrapping_add(OFFSETS_IN_BYTES));
    let first = e.mem.u32(offsets);
    let pixels = e.mem.u32(pixel_data.wrapping_add(PIXELS));
    if face == 0 {
        return first.wrapping_add(pixels);
    }
    let levels = e.mem.u32(pixel_data.wrapping_add(MIPMAP_LEVELS));
    let face_size = e.mem.u32(offsets.wrapping_add(levels.wrapping_mul(4)));
    pixels
        .wrapping_add(first)
        .wrapping_add(face_size.wrapping_mul(face))
}

/// A new pixel data (`00aa13e0(0x74)`, then its constructor with one level and one face
/// when the allocation succeeded) and its reference count incremented when not null.
fn new_pixel_data(e: &mut Engine, width: u32, height: u32, format: u32) -> u32 {
    let memory = e.call(NEW_OBJECT, &args![0x74u32]).u32();
    let pixel_data = if memory != 0 {
        e.call(
            NI_PIXEL_DATA_CONSTRUCT,
            &args![memory, width, height, format, 1u32, 1u32],
        )
        .u32()
    } else {
        0
    };
    if pixel_data != 0 {
        e.call(
            IMPORT_INTERLOCKED_INCREMENT,
            &args![pixel_data.wrapping_add(4)],
        );
    }
    pixel_data
}

/// Fills `width * height` pixels from the first level's start with `pattern`.
fn fill_pixels(e: &mut Engine, pixel_data: u32, width: u32, height: u32, pattern: &[u8]) {
    let offsets = e.mem.u32(pixel_data.wrapping_add(OFFSETS_IN_BYTES));
    let mut pixel = e
        .mem
        .u32(offsets)
        .wrapping_add(e.mem.u32(pixel_data.wrapping_add(PIXELS)));
    for _ in 0..height {
        for _ in 0..width {
            for byte in pattern {
                e.mem.set_u8(pixel, *byte);
                pixel = pixel.wrapping_add(1);
            }
        }
    }
}

/// `bNoConvert` set and `m_uiRevID` incremented, in the order the code makes them.
fn mark_pixels_changed(e: &mut Engine, pixel_data: u32, revision_first: bool) {
    let revision = pixel_data.wrapping_add(REVISION);
    if revision_first {
        let value = e.mem.u32(revision).wrapping_add(1);
        e.mem.set_u32(revision, value);
        e.mem.set_u8(pixel_data.wrapping_add(NO_CONVERT), 1);
    } else {
        e.mem.set_u8(pixel_data.wrapping_add(NO_CONVERT), 1);
        let value = e.mem.u32(revision).wrapping_add(1);
        e.mem.set_u32(revision, value);
    }
}

/// The inlined `NiPointer` assignment of a global: nothing when the same, else the old one
/// released, the new one stored and its count incremented when not null.
fn assign_global(e: &mut Engine, global: u32, object: u32) {
    let old = e.global::<u32>(global);
    if old != object {
        if old != 0 {
            release_ni_pointer(e, old);
        }
        e.set_global(global, object);
        if object != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
}

/// The inlined `NiFixedString` release: unless it is the word at `0109b220`, the count 8
/// bytes before the text is decremented.
fn release_fixed_string(e: &mut Engine, name: u32) {
    if name != e.global::<u32>(FIXED_STRING_NONE) {
        e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
}

/// A source texture named `text` made from `pixel_data` with the format preferences at
/// `prefs` (`00a5ff10`), assigned to `global`, the name released.
fn texture_from_pixels(e: &mut Engine, pixel_data: u32, text: u32, prefs: u32, global: u32) {
    let name = e.call(FIXED_STRING_MAKE, &args![text]).u32();
    let texture = e
        .call(SOURCE_TEXTURE_FROM_PIXELS, &args![pixel_data, name, prefs])
        .u32();
    assign_global(e, global, texture);
    release_fixed_string(e, name);
}

/// A filled solid texture: new pixel data, `pattern` in every pixel, marked changed, made
/// into a texture for `global` with the default preferences. Answers the pixel data.
fn solid_texture(
    e: &mut Engine,
    size: u32,
    format: u32,
    pattern: &[u8],
    revision_first: bool,
    text: u32,
    global: u32,
) -> u32 {
    let pixel_data = new_pixel_data(e, size, size, format);
    fill_pixels(e, pixel_data, size, size, pattern);
    mark_pixels_changed(e, pixel_data, revision_first);
    texture_from_pixels(e, pixel_data, text, FORMAT_PREFS_011A9598, global);
    pixel_data
}

/// Formats a file name of `Data\Textures\Effects\` (`010ad924`) into the frame's 0x104-byte
/// buffer with `format` (`00406d00`).
fn format_effects_path(e: &mut Engine, frame: u32, format: u32) {
    e.call(
        FORMAT_00406D00,
        &args![
            frame.wrapping_add(FRAME_BUFFER),
            0x104u32,
            format,
            EFFECTS_FOLDER_010AD924
        ],
    );
}

/// `"Data\\Textures\\Effects\\"` (`010ad924`).
const EFFECTS_FOLDER_010AD924: u32 = 0x010a_d924;

// Translated from 00b55bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadermanager.cpp`); no arguments. Makes the shader
/// manager's built-in textures, in the exe's order:
///
/// * a 128x128 six-face pixel data in the pixel format copied from `011aaac8` (revision
///   incremented at once) filled with a normalization cube (see `normal_cube_face`); it is
///   only released at the end;
/// * 16x16 solid textures: `BSShader_DefErrorMap` (ff 00 00 ff) into `011f9514`,
///   `BSShader_DefNormalMap` (80 80 ff, three-byte format) into `011f9510`,
///   `BSShader_DefNormalGlossMap` (80 80 ff 40) into `011f9518`, and `BSShader_DefHeightMap`
///   (00 00 00) into `011f951c`, whose pixel data replaces the normal map's in the same
///   `NiPointer` (the old one released without a null check, as the exe does);
/// * `BSShaderManager::GetTexture` (`00b55840`) of the two window environment maps into
///   `011f9520` and `011f9524`;
/// * 4x4 solid textures `BSShader_HideSecretMap` (00 00 00 40) into `011f9528` and
///   `BSShader_DefTexEffectMap` (00 00 00 ff) into `011f952c`;
/// * the cube maps `EyeReflectionCubeMap` (`%seyereflection.dds`) into `011f9544` and
///   `ArmorReflectionCubeMap` (`%sreflection.dds`) into `011f95d4` (`00a61120`, the same
///   file for all six faces), and the file textures `%saniso.dds` into `011f9530` and
///   `%sNoiseMap.dds` into `011f953c` (`00a5fd70`), all in `Data\Textures\Effects\`;
/// * the 4x4 `BSShader_EmptyNoiseMap` (8f 8f 8f ff) into `011f9540`;
/// * the 128x128 three-byte `BSShader_AttenuationXYZ` into `011f9534` and `011f9538`: for
///   each texel the distances of its column and row from 63.5 as fractions of 63.5 (`1 - v`
///   below the centre), the sum of their squares capped at 1 and raised to 0, inverted when
///   `004dc060(0)` answers 1, scaled by 255, passed through `00ec6940` and truncated into
///   all three bytes; its format preferences are a stack copy of (2, 0, 0).
///
/// Then the pixel data are released in reverse order (no null checks, as the exe).
// Translation incomplete: 00b55bb0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0x1c) and the /GS cookie check (00ec408c) are not translated.
pub fn fn_00b55bb0(e: &mut Engine) {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        for word in 0..0x11u32 {
            let value = e.global::<u32>(CUBE_PIXEL_FORMAT_011AAAC8 + 4 * word);
            e.mem
                .set_u32(frame.wrapping_add(FRAME_PIXEL_FORMAT + 4 * word), value);
        }
        let memory = e.call(NEW_OBJECT, &args![0x74u32]).u32();
        let cube = if memory != 0 {
            e.call(
                NI_PIXEL_DATA_CONSTRUCT,
                &args![
                    memory,
                    0x80u32,
                    0x80u32,
                    frame.wrapping_add(FRAME_PIXEL_FORMAT),
                    1u32,
                    6u32
                ],
            )
            .u32()
        } else {
            0
        };
        if cube != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![cube.wrapping_add(4)]);
        }
        let revision = e.mem.u32(cube.wrapping_add(REVISION)).wrapping_add(1);
        e.mem.set_u32(cube.wrapping_add(REVISION), revision);
        let constants = CubeConstants {
            half_size: e.global::<f64>(CUBE_HALF_SIZE_010240C0),
            minus_half_size: e.global::<f64>(CUBE_MINUS_HALF_SIZE_0106B9F0),
            depth_squared: e.global::<f64>(CUBE_DEPTH_SQUARED_01017A10),
            half: e.global::<f64>(HALF_01011588),
            scale: e.global::<f64>(BYTE_SCALE_0101E568),
        };
        for face in 0..6u32 {
            let pixels = cube_face_pixels(e, cube, face);
            normal_cube_face(e, face, pixels, &constants);
        }

        let prefs = frame.wrapping_add(FRAME_PREFS);
        e.mem.set_u32(prefs, 2);
        e.mem.set_u32(prefs.wrapping_add(8), 0);
        e.mem.set_u32(prefs.wrapping_add(4), 0);

        let error_map = solid_texture(
            e,
            0x10,
            PIXEL_FORMAT_011AA2A0,
            &[0xff, 0, 0, 0xff],
            false,
            0x010a_da24, // "BSShader_DefErrorMap"
            TEXTURE_011F9514,
        );
        let normal_map = solid_texture(
            e,
            0x10,
            PIXEL_FORMAT_011AA2E8,
            &[0x80, 0x80, 0xff],
            false,
            0x010a_da0c, // "BSShader_DefNormalMap"
            TEXTURE_011F9510,
        );
        let gloss_map = solid_texture(
            e,
            0x10,
            PIXEL_FORMAT_011AA2A0,
            &[0x80, 0x80, 0xff, 0x40],
            false,
            0x010a_d9f0, // "BSShader_DefNormalGlossMap"
            TEXTURE_011F9518,
        );

        // The height map's pixel data goes into the normal map's `NiPointer`.
        let memory = e.call(NEW_OBJECT, &args![0x74u32]).u32();
        let made = if memory != 0 {
            e.call(
                NI_PIXEL_DATA_CONSTRUCT,
                &args![memory, 0x10u32, 0x10u32, PIXEL_FORMAT_011AA2E8, 1u32, 1u32],
            )
            .u32()
        } else {
            0
        };
        let held = if normal_map != made {
            release_ni_pointer(e, normal_map);
            if made != 0 {
                e.call(IMPORT_INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
            }
            made
        } else {
            normal_map
        };
        fill_pixels(e, held, 0x10, 0x10, &[0, 0, 0]);
        mark_pixels_changed(e, held, false);
        texture_from_pixels(
            e,
            held,
            0x010a_d9d8, // "BSShader_DefHeightMap"
            FORMAT_PREFS_011A9598,
            TEXTURE_011F951C,
        );

        e.call(
            BS_SHADER_MANAGER_GET_TEXTURE,
            &args![
                0x010a_d9a4u32, // "Data\\Textures\\Effects\\WindowEnvironmentMap01.dds"
                1u32,
                TEXTURE_011F9520,
                1u32,
                0u32
            ],
        );
        e.call(
            BS_SHADER_MANAGER_GET_TEXTURE,
            &args![
                0x010a_d970u32, // "Data\\Textures\\Effects\\WindowEnvironmentMap02.dds"
                1u32,
                TEXTURE_011F9524,
                1u32,
                0u32
            ],
        );

        let hide_secret_map = solid_texture(
            e,
            4,
            PIXEL_FORMAT_011AA2A0,
            &[0, 0, 0, 0x40],
            true,
            0x010a_d958, // "BSShader_HideSecretMap"
            TEXTURE_011F9528,
        );
        let effect_map = solid_texture(
            e,
            4,
            PIXEL_FORMAT_011AA2A0,
            &[0, 0, 0, 0xff],
            false,
            0x010a_d93c, // "BSShader_DefTexEffectMap"
            TEXTURE_011F952C,
        );

        let name_slot = frame.wrapping_add(FRAME_NAME);
        let buffer = frame.wrapping_add(FRAME_BUFFER);
        for (format, text, global) in [
            // "%seyereflection.dds", "EyeReflectionCubeMap"
            (0x010a_d910u32, 0x010a_d8f8u32, TEXTURE_011F9544),
            // "%sreflection.dds", "ArmorReflectionCubeMap"
            (0x010a_d8e4, 0x010a_d8cc, TEXTURE_011F95D4),
        ] {
            format_effects_path(e, frame, format);
            let name = e.call(FIXED_STRING_MAKE, &args![text]).u32();
            e.mem.set_u32(name_slot, name);
            let renderer = e.global::<u32>(GLOBAL_011F9508);
            let cube_map = e
                .call(
                    SOURCE_CUBE_MAP_CREATE,
                    &args![
                        buffer,
                        buffer,
                        buffer,
                        buffer,
                        buffer,
                        buffer,
                        renderer,
                        name_slot,
                        FORMAT_PREFS_011A9598
                    ],
                )
                .u32();
            assign_global(e, global, cube_map);
            let name = e.mem.u32(name_slot);
            release_fixed_string(e, name);
        }
        for (format, global) in [
            // "%saniso.dds"
            (0x010a_d8c0u32, TEXTURE_011F9530),
            // "%sNoiseMap.dds"
            (0x010a_d8b0, TEXTURE_011F953C),
        ] {
            format_effects_path(e, frame, format);
            let name = e.call(FIXED_STRING_MAKE, &args![buffer]).u32();
            e.mem.set_u32(name_slot, name);
            let texture = e
                .call(
                    SOURCE_TEXTURE_FROM_FILE,
                    &args![name_slot, FORMAT_PREFS_011A9598, 1u32, 0u32],
                )
                .u32();
            assign_global(e, global, texture);
            let name = e.mem.u32(name_slot);
            release_fixed_string(e, name);
        }

        let empty_noise_map = solid_texture(
            e,
            4,
            PIXEL_FORMAT_011AA2A0,
            &[0x8f, 0x8f, 0x8f, 0xff],
            false,
            0x010a_d898, // "BSShader_EmptyNoiseMap"
            TEXTURE_011F9540,
        );

        let attenuation = new_pixel_data(e, 0x80, 0x80, PIXEL_FORMAT_011AA2E8);
        attenuation_pixels(e, attenuation);
        e.mem.set_u32(prefs, 2);
        e.mem.set_u32(prefs.wrapping_add(8), 0);
        e.mem.set_u32(prefs.wrapping_add(4), 0);
        mark_pixels_changed(e, attenuation, false);
        texture_from_pixels(
            e,
            attenuation,
            0x010a_d874, // "BSShader_AttenuationXYZ"
            prefs,
            TEXTURE_011F9534,
        );
        let texture = e.global::<u32>(TEXTURE_011F9534);
        assign_global(e, TEXTURE_011F9538, texture);

        for pixel_data in [
            attenuation,
            empty_noise_map,
            effect_map,
            hide_secret_map,
            gloss_map,
            held,
            error_map,
            cube,
        ] {
            release_ni_pointer(e, pixel_data);
        }
    });
}

/// The pixels of `BSShader_AttenuationXYZ` (`00b570d2` to `00b5721a`): 128 rows of 128
/// three-byte texels, each byte the same value.
fn attenuation_pixels(e: &mut Engine, pixel_data: u32) {
    let offsets = e.mem.u32(pixel_data.wrapping_add(OFFSETS_IN_BYTES));
    let mut pixel = e
        .mem
        .u32(offsets)
        .wrapping_add(e.mem.u32(pixel_data.wrapping_add(PIXELS)));
    // The fraction of 63.5 a coordinate lies from the centre: `1 - v / c` below it, else
    // `(v - c) / c` (also for a NaN comparison).
    let distance = |value: f64, centre: f64| -> u32 {
        if value < centre {
            x87_store32(x87_sub(1.0, x87_div(value, centre)))
        } else {
            x87_store32(x87_div(x87_sub(value, centre), centre))
        }
    };
    let mut row = 0i32;
    loop {
        let row_bits = x87_store32(f64::from(row));
        let mut column = 0i32;
        loop {
            // The centre is loaded again for every texel.
            let centre = e.global::<f64>(ATTENUATION_CENTRE_010AD890);
            let column_bits = x87_store32(f64::from(column));
            let across = distance(x87_load32(column_bits), centre);
            let down = distance(x87_load32(row_bits), centre);
            let across = x87_load32(across);
            let down = x87_load32(down);
            let sum = x87_add(x87_mul(down, down), x87_mul(across, across));
            let sum = x87_load32(x87_store32(sum));
            // Capped at 1 when above it (ordered), raised to 0 when below 0.
            let capped = if sum > 1.0 { 1.0 } else { sum };
            let capped = x87_load32(x87_store32(capped));
            let zero = e.global::<f64>(ZERO_01012060);
            let floored = if capped < zero { 0.0 } else { capped };
            let stored = x87_store32(floored);
            let inverted = e.call(RENDERER_QUERY_004DC060, &args![0u32]).u32();
            let value = x87_load32(stored);
            let value = if inverted == 1 {
                x87_sub(1.0, value)
            } else {
                value
            };
            let value = x87_store32(value);
            let scale = e.global::<f64>(BYTE_SCALE_0101E568);
            let scaled = x87_store32(x87_mul(x87_load32(value), scale));
            let result = e.call(CRT_00EC6940, &args![x87_load32(scaled)]).f64();
            let result = x87_load32(x87_store32(result));
            let byte = fistp_truncate(result) as u8;
            for index in 0..3u32 {
                e.mem.set_u8(pixel.wrapping_add(index), byte);
            }
            pixel = pixel.wrapping_add(3);
            column += 1;
            if column >= 0x80 {
                break;
            }
        }
        row += 1;
        if row >= 0x80 {
            break;
        }
    }
}

/// `BSTexturePalette::GetTexture_ov2` (engine map, `00a61b90`, `cdecl`: name, the
/// `NiPointer` to fill) and `BSTexturePalette::SetTexture` (`00a61c50`, `cdecl`: texture,
/// word).
const TEXTURE_PALETTE_GET_TEXTURE: u32 = 0x00a6_1b90;
const TEXTURE_PALETTE_SET_TEXTURE: u32 = 0x00a6_1c50;
/// The file finder object (`011f81dc`) and its look-up `00afe0d0` (engine map unit
/// `BSSystem/filefinder.cpp`, `thiscall`, four words: name, 0, 6, 2).
const FILE_FINDER_011F81DC: u32 = 0x011f_81dc;
const FILE_FINDER_FIND_00AFE0D0: u32 = 0x00af_e0d0;
/// The four-byte stack name made from the file name (`00438170`, `thiscall`, `RET 4`: the
/// text; it answers the name's address) and its clean-up (`004381b0`, `thiscall`).
const NAME_MAKE_00438170: u32 = 0x0043_8170;
const NAME_CLEAN_UP_004381B0: u32 = 0x0043_81b0;
/// The cube texture maker `00a60f60` (engine map unit `NiMain/nisourcecubemap.cpp`, `cdecl`:
/// the name's address, the renderer, format preferences).
const SOURCE_CUBE_MAP_FROM_FILE_00A60F60: u32 = 0x00a6_0f60;
/// The `NiPointer` assignment `0066b0d0` (`thiscall` on the pointer, `RET 4`).
const NI_POINTER_ASSIGN_0066B0D0: u32 = 0x0066_b0d0;
/// The kind test `00653290` (`thiscall` on the object, `RET 4`: an `NiRTTI` record; answers
/// the object or 0) and the record it is given (`011f444c`, in `.bss`).
const KIND_TEST_00653290: u32 = 0x0065_3290;
const RTTI_RECORD_011F444C: u32 = 0x011f_444c;
/// The two messages formatted for a texture without mipmaps (`010ad830`, with the name;
/// `010ad7d8`, "... NOTASOURCETEXTURE") and the stack buffer's size.
const NO_MIPMAPS_MESSAGE_010AD830: u32 = 0x010a_d830;
const NO_MIPMAPS_PLAIN_MESSAGE_010AD7D8: u32 = 0x010a_d7d8;
const MESSAGE_BYTES: u32 = 500;

// Translated from 00b55840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::GetTexture` (engine map). `cdecl`, five words: the file name, whether to
/// load a texture the palette does not have (byte), the `NiPointer` to fill, whether to refuse
/// a texture without mipmaps (byte), and whether it is a cube map (byte).
///
/// Without a name the pointer is released and cleared. Otherwise the palette fills it
/// (`00a61b90`); without the load byte that is all. When it stayed null and the file finder
/// (`011f81dc`) exists and finds the file (`00afe0d0` with 0, 6, 2), a stack name is made
/// from the file name (`00438170`), the texture made from it (cube: `00a60f60` with the
/// renderer `011f9508` and the preferences `011a9598`; else `NiSourceTexture::Create`
/// `00a5fd70` with the preferences, 1, 0), assigned to the pointer (`0066b0d0`), the name
/// cleaned up (`004381b0`), and a texture that came out given to the palette
/// (`BSTexturePalette::SetTexture`, `00a61c50`, with 0). Then, with the mipmap byte and a
/// texture: the texture is kind-tested against the record at `011f444c` (`00653290`); when
/// its renderer data (+0x24) is set and that data's vtable slot +0x98 answers at most 1
/// (unsigned), a message is formatted into a 500-byte stack buffer (`00406d00`, with the
/// kind-tested object's +0x34 when it passed, else the plain message; nothing reads the
/// buffer) and the texture is released and the pointer cleared.
// Translation incomplete: 00b55840 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the stack name) and the /GS cookie check (00ec408c) are not translated.
pub fn bs_shader_manager_get_texture(
    e: &mut Engine,
    name: Ptr,
    load_missing: bool,
    texture: Ptr,
    refuse_without_mipmaps: bool,
    cube: bool,
) {
    let slot = texture.addr();
    if !name.is_null() {
        e.call(TEXTURE_PALETTE_GET_TEXTURE, &args![name, texture]);
        if !load_missing {
            return;
        }
        let finder = e.global::<u32>(FILE_FINDER_011F81DC);
        if e.mem.u32(slot) == 0
            && finder != 0
            && e.call(
                FILE_FINDER_FIND_00AFE0D0,
                &args![finder, name, 0u32, 6u32, 2u32],
            )
            .u32()
                != 0
        {
            e.with_stack(4, |e, local| {
                let made = e.call(NAME_MAKE_00438170, &args![local, name]).u32();
                let object = if cube {
                    let renderer = e.global::<u32>(GLOBAL_011F9508);
                    e.call(
                        SOURCE_CUBE_MAP_FROM_FILE_00A60F60,
                        &args![made, renderer, FORMAT_PREFS_011A9598],
                    )
                    .u32()
                } else {
                    e.call(
                        SOURCE_TEXTURE_FROM_FILE,
                        &args![made, FORMAT_PREFS_011A9598, 1u32, 0u32],
                    )
                    .u32()
                };
                e.call(NI_POINTER_ASSIGN_0066B0D0, &args![texture, object]);
                e.call(NAME_CLEAN_UP_004381B0, &args![local]);
            });
            let held = e.mem.u32(slot);
            if held != 0 {
                e.call(TEXTURE_PALETTE_SET_TEXTURE, &args![held, 0u32]);
            }
        }
        if !refuse_without_mipmaps || e.mem.u32(slot) == 0 {
            return;
        }
        let held = e.mem.u32(slot);
        let source = e
            .call(KIND_TEST_00653290, &args![held, RTTI_RECORD_011F444C])
            .u32();
        let renderer_data = e.mem.u32(e.mem.u32(slot).wrapping_add(0x24));
        if renderer_data == 0 {
            return;
        }
        if e.vcall(renderer_data, 0x98, &args![]).u32() > 1 {
            return;
        }
        e.with_stack(MESSAGE_BYTES, |e, buffer| {
            if source != 0 {
                let text = e.mem.u32(source.wrapping_add(0x34));
                e.call(
                    FORMAT_00406D00,
                    &args![buffer, MESSAGE_BYTES, NO_MIPMAPS_MESSAGE_010AD830, text],
                );
            } else {
                e.call(
                    FORMAT_00406D00,
                    &args![buffer, MESSAGE_BYTES, NO_MIPMAPS_PLAIN_MESSAGE_010AD7D8],
                );
            }
        });
    }
    let held = e.mem.u32(slot);
    if held != 0 {
        release_ni_pointer(e, held);
        e.mem.set_u32(slot, 0);
    }
}

/// The two shader settings the profile choosers read: `011f91bc` (the pixel chooser's when its
/// byte is set) and `011f91c0` (otherwise, and the vertex chooser's).
const PROFILE_SETTING_011F91BC: u32 = 0x011f_91bc;
const PROFILE_SETTING_011F91C0: u32 = 0x011f_91c0;
/// The profile strings (read in Ghidra).
const PS_1_3_010A93D4: u32 = 0x010a_93d4;
const PS_2_0_010A93CC: u32 = 0x010a_93cc;
const PS_2_A_010A93C4: u32 = 0x010a_93c4;
const PS_2_B_010A93BC: u32 = 0x010a_93bc;
const PS_3_0_010A93B4: u32 = 0x010a_93b4;
const VS_1_1_010A93EC: u32 = 0x010a_93ec;
const VS_2_0_010A93E4: u32 = 0x010a_93e4;
const VS_3_0_010A93DC: u32 = 0x010a_93dc;

// Translated from 00b4f380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshadermanager.cpp`): the pixel shader profile string,
/// `cdecl` with one byte. The setting is the word at `011f91bc` (always read) when the byte is
/// set, else the one at `011f91c0`; 1 gives `"ps_1_3"`, 2 `"ps_2_0"`, 3 and 5 `"ps_2_a"`, 4
/// and 6 `"ps_2_b"`, 7 `"ps_3_0"` (the arms of the switch's jump table at `00b4f3c0`, read in
/// Ghidra), anything else null. Answers the string's address.
pub fn fn_00b4f380(e: &mut Engine, first_setting: u8) -> u32 {
    let mut setting = e.global::<u32>(PROFILE_SETTING_011F91BC);
    if first_setting == 0 {
        setting = e.global::<u32>(PROFILE_SETTING_011F91C0);
    }
    match setting.wrapping_sub(1) {
        0 => PS_1_3_010A93D4,
        1 => PS_2_0_010A93CC,
        2 | 4 => PS_2_A_010A93C4,
        3 | 5 => PS_2_B_010A93BC,
        6 => PS_3_0_010A93B4,
        _ => 0,
    }
}

// Translated from 00b4f3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshadermanager.cpp`): the vertex shader profile string,
/// no arguments. For the setting at `011f91c0`: 1 gives `"vs_1_1"`, 2 to 6 `"vs_2_0"`, 7
/// `"vs_3_0"` (the switch's jump table at `00b4f408`, read in Ghidra), anything else null.
pub fn fn_00b4f3e0(e: &mut Engine) -> u32 {
    match e.global::<u32>(PROFILE_SETTING_011F91C0).wrapping_sub(1) {
        0 => VS_1_1_010A93EC,
        1..=5 => VS_2_0_010A93E4,
        6 => VS_3_0_010A93DC,
        _ => 0,
    }
}

/// The shader level names `GetShaderLevelString` answers (read in Ghidra), levels 0 to 7,
/// and `"???"` for any other level.
const SHADER_LEVEL_NAMES: [u32; 8] = [
    0x010a_9454, // "BSSM_SV_NONE"
    0x010a_9448, // "BSSM_SV_1_X"
    0x010a_943c, // "BSSM_SV_2_0"
    0x010a_942c, // "BSSM_SV_2_A96"
    0x010a_941c, // "BSSM_SV_2_B96"
    0x010a_9410, // "BSSM_SV_2_A"
    0x010a_9404, // "BSSM_SV_2_B"
    0x010a_93f8, // "BSSM_SV_3_0"
];
const SHADER_LEVEL_UNKNOWN: u32 = 0x010a_93f4;

// Translated from 00b4f4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::GetShaderLevelString` (Xbox PDB), no arguments: the name of the shader
/// level at `011f91c0` (the arms of the switch's jump table at `00b4f4e8`, read in Ghidra), the
/// unknown-level string for anything above 7.
pub fn bs_shader_manager_get_shader_level_string(e: &mut Engine) -> u32 {
    let level = e.global::<u32>(PROFILE_SETTING_011F91C0);
    match SHADER_LEVEL_NAMES.get(level as usize) {
        Some(name) => *name,
        None => SHADER_LEVEL_UNKNOWN,
    }
}

/// The globals `00b4f710` sets besides the two profile settings: a flag word (0x2f at level 2
/// and above, 0xf at level 1, bit 4 cleared below level 2), the byte set when both major
/// versions are 3 or more, the byte cleared below level 2, the word set to 0x15 and the vendor
/// word (2, 1 or 3) for the three recognised drivers, the `float` copied from `010a9478`, the
/// byte kept set only with `011f94a4` and `011f94d8`, and the byte copied from `011f94e4`.
const SHADER_FLAGS_011F91D8: u32 = 0x011f_91d8;
const VERSION_3_BOTH_011F94A4: u32 = 0x011f_94a4;
const BELOW_LEVEL_2_FLAG_011F941E: u32 = 0x011f_941e;
const VENDOR_SETTING_011AD870: u32 = 0x011a_d870;
const VENDOR_011F94B8: u32 = 0x011f_94b8;
const FLOAT_TARGET_01200458: u32 = 0x0120_0458;
const FLOAT_SOURCE_010A9478: u32 = 0x010a_9478;
const KEPT_FLAG_011AD80E: u32 = 0x011a_d80e;
const REQUIRED_FLAG_011F94D8: u32 = 0x011f_94d8;
const COPIED_FLAG_011F91A4: u32 = 0x011f_91a4;
const COPY_SOURCE_011F94E4: u32 = 0x011f_94e4;
/// `_strnicmp` (`00ec7ec0`, `cdecl`: string, string, count) and the driver prefixes it
/// compares (`"ati"`, `"nv"`, `"ig"`, read in Ghidra).
const STRNICMP: u32 = 0x00ec_7ec0;
const PREFIX_ATI_010A9484: u32 = 0x010a_9484;
const PREFIX_NV_010A9480: u32 = 0x010a_9480;
const PREFIX_IG_010A947C: u32 = 0x010a_947c;

// Translated from 00b4f710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadermanager.cpp`): the shader manager's set-up from the
/// device, `cdecl`: pixel and vertex shader versions (their major version is byte 1), the
/// force-1.x byte, the device and driver name strings and the maximum pixel shader instruction
/// count.
///
/// Both majors 2 or more: level 2 (`011f91c0`), flags 0x2f, `011f94a4` when both are 3 or
/// more; with both strings, the driver prefix picks the level and settings: `"ati"` gives level
/// 6 above 0x60 instructions (else 4), pixel setting (`011f91bc`) 6, `011ad870` 0x15 and vendor
/// 2; `"nv"` and `"ig"` give level 5 above 0x60 (else 3), pixel setting 5 and vendor 1 or 3;
/// above 0xff instructions `011f94a6` is set. (The level changes only from 2.) Any other driver
/// gives level 2 and pixel setting 2. After a driver check the `float` at `010a9478` is copied
/// to `01200458` and bit 4 of the flags is cleared for a level below 2. Otherwise both majors 1
/// or more give level 1 and flags 0xf, else level 0; `011f941e` is cleared and bit 4 of the
/// flags too. Then the force byte turns a nonzero level into 1, `011ad80e` stays set only when
/// it, `011f94a4` and `011f94d8` are all set, and `011f91a4` gets the byte at `011f94e4`.
pub fn fn_00b4f710(
    e: &mut Engine,
    pixel_shader_version: u32,
    vertex_shader_version: u32,
    force_1x_shaders: u8,
    device_name: Ptr,
    driver_name: Ptr,
    max_ps_instructions: i32,
) {
    let pixel_major = (pixel_shader_version >> 8) as u8;
    let vertex_major = (vertex_shader_version >> 8) as u8;
    // The level as the exe keeps it in EAX (always the value of 011f91c0).
    let mut level: u32;
    if pixel_major >= 2 && vertex_major >= 2 {
        level = 2;
        e.set_global(PROFILE_SETTING_011F91C0, level);
        e.set_global(SHADER_FLAGS_011F91D8, 0x2fu32);
        if pixel_major >= 3 && vertex_major >= 3 {
            e.set_global(VERSION_3_BOTH_011F94A4, 1u8);
        }
        if !device_name.is_null() && !driver_name.is_null() {
            let driver = driver_name.addr();
            let has_prefix = |e: &mut Engine, prefix: u32, count: u32| {
                e.call(STRNICMP, &args![driver, prefix, count]).i32() == 0
            };
            // (level when 0x60 instructions or fewer, level above, vendor word)
            let vendor = if has_prefix(e, PREFIX_ATI_010A9484, 3) {
                Some((4u32, 6u32, 2u32))
            } else if has_prefix(e, PREFIX_NV_010A9480, 2) {
                Some((3, 5, 1))
            } else if has_prefix(e, PREFIX_IG_010A947C, 2) {
                Some((3, 5, 3))
            } else {
                None
            };
            match vendor {
                Some((low_level, high_level, vendor_word)) => {
                    level = e.global::<u32>(PROFILE_SETTING_011F91C0);
                    if max_ps_instructions > 0x60 {
                        if level == 2 {
                            level = high_level;
                            e.set_global(PROFILE_SETTING_011F91C0, level);
                        }
                        e.set_global(PROFILE_SETTING_011F91BC, high_level);
                        if max_ps_instructions > 0xff {
                            e.set_global(SHADER_1D_SELECT_011F94A6, 1u8);
                        }
                    } else {
                        if level == 2 {
                            level = low_level;
                            e.set_global(PROFILE_SETTING_011F91C0, level);
                        }
                        e.set_global(PROFILE_SETTING_011F91BC, high_level);
                    }
                    if vendor_word == 2 {
                        e.set_global(VENDOR_SETTING_011AD870, 0x15u32);
                    }
                    e.set_global(VENDOR_011F94B8, vendor_word);
                }
                None => {
                    level = 2;
                    e.set_global(PROFILE_SETTING_011F91C0, level);
                    e.set_global(PROFILE_SETTING_011F91BC, level);
                }
            }
            let copied = x87_quiet_bits(e.global::<u32>(FLOAT_SOURCE_010A9478));
            e.set_global(FLOAT_TARGET_01200458, copied);
            if (level as i32) < 2 {
                let flags = e.global::<u32>(SHADER_FLAGS_011F91D8) & 0xffff_ffef;
                e.set_global(SHADER_FLAGS_011F91D8, flags);
            }
        }
    } else {
        if pixel_major >= 1 && vertex_major >= 1 {
            level = 1;
            e.set_global(PROFILE_SETTING_011F91C0, level);
            e.set_global(BELOW_LEVEL_2_FLAG_011F941E, 0u8);
            e.set_global(SHADER_FLAGS_011F91D8, 0xfu32);
        } else {
            level = 0;
            e.set_global(PROFILE_SETTING_011F91C0, level);
            e.set_global(BELOW_LEVEL_2_FLAG_011F941E, 0u8);
        }
        let flags = e.global::<u32>(SHADER_FLAGS_011F91D8) & 0xffff_ffef;
        e.set_global(SHADER_FLAGS_011F91D8, flags);
    }
    if force_1x_shaders != 0 && level != 0 {
        e.set_global(PROFILE_SETTING_011F91C0, 1u32);
    }
    let copied = e.global::<u8>(COPY_SOURCE_011F94E4);
    let keep = e.global::<u8>(KEPT_FLAG_011AD80E) != 0
        && e.global::<u8>(VERSION_3_BOTH_011F94A4) != 0
        && e.global::<u8>(REQUIRED_FLAG_011F94D8) != 0;
    e.set_global(KEPT_FLAG_011AD80E, u8::from(keep));
    e.set_global(COPIED_FLAG_011F91A4, copied);
}

/// The floats `00b4f300` writes: the rectangle's centre (x, y) and half
/// extents (x, y).
const VIEWPORT_CENTRE_X_011F95F4: u32 = 0x011f_95f4;
const VIEWPORT_CENTRE_Y_011F95F8: u32 = 0x011f_95f8;
const VIEWPORT_HALF_WIDTH_011F95FC: u32 = 0x011f_95fc;
const VIEWPORT_HALF_HEIGHT_011F9600: u32 = 0x011f_9600;
/// Thiscall on the object at `011f91ac` (word), `RET 4`: keeps the word at
/// its +0xAC.
const KEEP_VALUE_00B8CAD0: u32 = 0x00b8_cad0;
/// The index `00b4f450` keeps, and its two tables of function words with
/// the selected entries (entry 0 when the selected one is null).
const SELECTED_INDEX_011F91B8: u32 = 0x011f_91b8;
const FIRST_TABLE_011F9F80: u32 = 0x011f_9f80;
const FIRST_SELECTED_011F9F3C: u32 = 0x011f_9f3c;
const SECOND_TABLE_011F9F40: u32 = 0x011f_9f40;
const SECOND_SELECTED_011F9F38: u32 = 0x011f_9f38;
/// The second reference-counted global `BSShaderManager` keeps next to the
/// accumulator (`011f95f0`).
const SECOND_ACCUMULATOR_011F95F0: u32 = 0x011f_95f0;
/// `_strcpy_s`, `strrchr` and the formatted print (`sprintf_s`-like) the
/// texture name code calls, cdecl.
const STRCPY_S: u32 = 0x00ec_65a6;
const STRRCHR: u32 = 0x00ec_6e30;
/// `"data"`, `"Data\%s%s%s"` and `"%s%s%s"`.
const DATA_PREFIX_010A9470: u32 = 0x010a_9470;
const DATA_FORMAT_010A9464: u32 = 0x010a_9464;
const PLAIN_FORMAT_0104F208: u32 = 0x0104_f208;
/// `ImageSpaceTexture::GetTexture` and `ImageSpaceTexture::ReturnRenderedTexture`
/// (engine map), thiscall on the texture at `011f91ac` + 0x204.
const IMAGE_SPACE_TEXTURE_GET_TEXTURE: u32 = 0x00ba_3780;
const IMAGE_SPACE_TEXTURE_RETURN_RENDERED: u32 = 0x00ba_39a0;
/// The index `00b540f0` steps through the accumulator array.
const ACCUMULATOR_INDEX_011F9168: u32 = 0x011f_9168;
/// `NiAVObject::GetProperty` (engine map), thiscall (property type), `RET 4`.
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// `NiAVObject::RemoveProperty_ov2` (engine map), thiscall (type), `RET 4`.
const REMOVE_PROPERTY: u32 = 0x00a5_b230;
/// `NiAVObject::AttachProperty` (engine map), thiscall (property), `RET 4`.
const ATTACH_PROPERTY: u32 = 0x0043_9410;
/// `NiAVObject::UpdateProperties` (engine map), thiscall.
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// The object-type function `SetObjectType` and `PrepareObject` call when
/// it is set (`011f91ec`, cdecl (object)).
const OBJECT_TYPE_FUNCTION_011F91EC: u32 = 0x011f_91ec;
/// The debug shader the debug-shader set-up keeps (`011f956c`).
const DEBUG_SHADER_011F956C: u32 = 0x011f_956c;
/// `DebugShader::CreateShader` (engine map).
const DEBUG_SHADER_CREATE: u32 = 0x00bd_23f0;
/// Thiscall on the debug shader (word), `RET 4`.
const DEBUG_SHADER_00BD1CF0: u32 = 0x00bd_1cf0;
/// Thiscall on the object at `011f91ac` (index, two out words), `RET 0xC`.
const QUERY_00B97900: u32 = 0x00b9_7900;
/// The renderer object `PrepareGeometry` asks last (`011f4748`).
const RENDERER_OBJECT_011F4748: u32 = 0x011f_4748;
/// RTTI records `PrepareGeometry` compares a shader's vtable slot +8 with.
const SHADER_RTTI_012021B8: u32 = 0x0120_21b8;
const SHADER_RTTI_01200508: u32 = 0x0120_0508;
const SHADER_RTTI_01202040: u32 = 0x0120_2040;
/// `"ERROR: %s does not have a shader property. "`.
const NO_SHADER_PROPERTY_010ADA44: u32 = 0x010a_da44;
/// The set-up's float (`0102f0f8`) passed to `00b54000`.
const DEFAULT_VALUE_0102F0F8: u32 = 0x0102_f0f8;
/// `"Shader %i is leaking. ..."`.
const SHADER_LEAKING_010AD6E8: u32 = 0x010a_d6e8;
/// The object the shut-down stops (`00ba2d80`, `run_00ba2d40.rs`): the
/// rendering task threads.
const RENDER_THREADS_01200088: u32 = 0x0120_0088;
const STOP_RENDER_THREADS: u32 = 0x00ba_2d80;
/// `NiMaterialProperty::NiMaterialProperty` (engine map), thiscall.
const MATERIAL_PROPERTY_CONSTRUCT: u32 = 0x00a7_5650;
/// Thiscall on an object (property), `RET 4` (`run_00653e50.rs`).
const OBJECT_00654640: u32 = 0x0065_4640;
/// The 0x10-byte scope object `CloneMaterialPropertyRecurse` keeps on its
/// stack: its constructor (`bipedanim.rs`), thiscall (word), `RET 4`.
const SCOPE_CONSTRUCT_004AD0C0: u32 = 0x004a_d0c0;

/// Releases the reference-counted object in `global` and clears it, when
/// there is one.
fn release_global(e: &mut Engine, global: u32) {
    let object = e.global::<u32>(global);
    if object != 0 {
        release_ni_pointer(e, object);
        e.set_global(global, 0u32);
    }
}

// Translated from 00b4f300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): from the rectangle at `rect` (left,
/// top, right, bottom), the half extents `(right - left) / 2` and `(top -
/// bottom) / 2` (integer division) as floats into `011f95fc` and
/// `011f9600`, and the centre `left + half width`, `bottom + half height`
/// into `011f95f4` and `011f95f8`.
pub fn fn_00b4f300(e: &mut Engine, rect: u32) {
    let left = e.mem.i32(rect);
    let half_width = e.mem.i32(rect + 8).wrapping_sub(left) / 2;
    let half_height = e.mem.i32(rect + 4).wrapping_sub(e.mem.i32(rect + 0xc)) / 2;
    let width_bits = x87_store32(f64::from(half_width));
    let centre_x = x87_add(f64::from(left), x87_load32(width_bits));
    e.set_global(VIEWPORT_CENTRE_X_011F95F4, x87_store32(centre_x));
    let height_bits = x87_store32(f64::from(half_height));
    let bottom = f64::from(e.mem.i32(rect + 0xc));
    let centre_y = x87_add(bottom, x87_load32(height_bits));
    e.set_global(VIEWPORT_CENTRE_Y_011F95F8, x87_store32(centre_y));
    e.set_global(VIEWPORT_HALF_WIDTH_011F95FC, width_bits);
    e.set_global(VIEWPORT_HALF_HEIGHT_011F9600, height_bits);
}

// Translated from 00b4f430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): `00b8cad0(word)` on the object at
/// `011f91ac`.
pub fn fn_00b4f430(e: &mut Engine, word: u32) {
    let object = e.global::<u32>(OBJECT_011F91AC);
    e.call(KEEP_VALUE_00B8CAD0, &args![object, word]);
}

// Translated from 00b4f450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): keeps `index` (`011f91b8`) and
/// selects entry `index` of the two function tables (`011f9f80` into
/// `011f9f3c`, `011f9f40` into `011f9f38`), each falling back to its entry
/// 0 when null.
pub fn fn_00b4f450(e: &mut Engine, index: u32) {
    let first = e
        .mem
        .u32(FIRST_TABLE_011F9F80.wrapping_add(index.wrapping_mul(4)));
    e.set_global(SELECTED_INDEX_011F91B8, index);
    e.set_global(FIRST_SELECTED_011F9F3C, first);
    if first == 0 {
        let fallback = e.global::<u32>(FIRST_TABLE_011F9F80);
        e.set_global(FIRST_SELECTED_011F9F3C, fallback);
    }
    let second = e
        .mem
        .u32(SECOND_TABLE_011F9F40.wrapping_add(index.wrapping_mul(4)));
    e.set_global(SECOND_SELECTED_011F9F38, second);
    if second == 0 {
        let fallback = e.global::<u32>(SECOND_TABLE_011F9F40);
        e.set_global(SECOND_SELECTED_011F9F38, fallback);
    }
}

// Translated from 00b4f5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at `011f95f0`.
pub fn fn_00b4f5d0(e: &mut Engine) -> u32 {
    e.global::<u32>(SECOND_ACCUMULATOR_011F95F0)
}

// Translated from 00b4f5e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b4f5e0 - the /GS cookie check (00ec408c) is not translated.
/// `BSShaderManager::GetModifiedTextureFilename` (engine map; cdecl): copies
/// `source` into a 0x104-byte buffer and clears `destination[0]`; with an
/// extension (the last `'.'`), keeps up to 0xA bytes of it, cuts the name at
/// the last `'_'` (unless that is before the last `'\'`) and at the
/// extension, and writes `name`, `suffix` and the extension into
/// `destination` (0x104 bytes) as `"Data\%s%s%s"` when `prefix_data` is
/// set and the name is relative (not starting `'\'` nor with `':'` second)
/// and does not start with `"data"` (`_strnicmp`, 4), else as `"%s%s%s"`.
pub fn bs_shader_manager_get_modified_texture_filename(
    e: &mut Engine,
    destination: u32,
    source: u32,
    suffix: u32,
    prefix_data: u8,
) {
    // The locals: the suffix word (+0), the extension (+4, 0xA bytes) and
    // the name (+0x10, 0x104 bytes).
    e.with_stack(0x114, |e, locals| {
        let base = locals.addr();
        let extension = base + 4;
        let name = base + 0x10;
        e.call(STRCPY_S, &args![name, 0x104u32, source]);
        e.mem.set_u32(base, suffix);
        e.mem.set_u8(destination, 0);
        let dot = e.call(STRRCHR, &args![name, 0x2eu32]).u32();
        let slash = e.call(STRRCHR, &args![name, 0x5cu32]).u32();
        let mut underscore = e.call(STRRCHR, &args![name, 0x5fu32]).u32();
        if slash != 0 && underscore != 0 && underscore < slash {
            underscore = 0;
        }
        if dot == 0 {
            return;
        }
        e.call(STRCPY_S, &args![extension, 0xau32, dot]);
        if underscore != 0 {
            e.mem.set_u8(underscore, 0);
        }
        e.mem.set_u8(dot, 0);
        let format = if prefix_data != 0
            && e.mem.u8(name) != 0x5c
            && e.mem.u8(name + 1) != 0x3a
            && e.call(STRNICMP, &args![name, DATA_PREFIX_010A9470, 4u32])
                .u32()
                != 0
        {
            DATA_FORMAT_010A9464
        } else {
            PLAIN_FORMAT_0104F208
        };
        let suffix = e.mem.u32(base);
        e.call(
            FORMAT_00406D00,
            &args![destination, 0x104u32, format, name, suffix, extension],
        );
    });
}

// Translated from 00b54090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `ImageSpaceTexture::GetTexture` on the
/// texture at +0x204 of the object at `011f91ac` (a jump); its result.
pub fn fn_00b54090(e: &mut Engine) -> u32 {
    let object = e.global::<u32>(OBJECT_011F91AC);
    e.call(
        IMAGE_SPACE_TEXTURE_GET_TEXTURE,
        &args![object.wrapping_add(0x204)],
    )
    .u32()
}

// Translated from 00b540b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `ImageSpaceTexture::ReturnRenderedTexture` on
/// the texture at +0x204 of the object at `011f91ac` (a jump); its `EAX`.
pub fn fn_00b540b0(e: &mut Engine) -> u32 {
    let object = e.global::<u32>(OBJECT_011F91AC);
    e.call(
        IMAGE_SPACE_TEXTURE_RETURN_RENDERED,
        &args![object.wrapping_add(0x204)],
    )
    .u32()
}

// Translated from 00b540f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the next accumulator of the array at
/// `011f9170` (index `011f9168`, counted up).
pub fn fn_00b540f0(e: &mut Engine) -> u32 {
    let index = e.global::<u32>(ACCUMULATOR_INDEX_011F9168);
    let array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
    let slot = array.wrapping_add(index.wrapping_mul(4));
    e.set_global(ACCUMULATOR_INDEX_011F9168, index.wrapping_add(1));
    e.mem.u32(slot)
}

// Translated from 00b54280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: resets the shader manager's state: its flags
/// and words to their defaults (among them `011f91d8` = 0x2F and
/// `011f91f8` = `00b4f2e0`), releases the 0x23 shaders of `011f9548`, the
/// textures `011f950c` to `011f9544` and `011f95d4` and the accumulator
/// `011f95ec`, passes the float at `0102f0f8` to `00b54000`, sets the four
/// floats `011f9184` to `011f9190` to `1.0` and clears `011f91ac`.
/// Returns `this` (not otherwise used).
pub fn fn_00b54280(e: &mut Engine, this: Ptr) -> Ptr {
    for (address, value) in [(0x011f_9308u32, 0u8), (0x011f_9200, 0), (0x011f_91df, 1)] {
        e.set_global(address, value);
    }
    e.set_global(0x011f_91e4u32, 0u32);
    e.set_global(0x011f_91e0u32, 0u32);
    e.set_global(0x011f_91deu32, 1u8);
    e.set_global(0x011f_91ddu32, 0u8);
    e.set_global(SHADER_FLAGS_011F91D8, 0x2fu32);
    e.set_global(0x011f_91f8u32, 0x00b4_f2e0u32);
    e.set_global(0x011f_91f0u32, 0u32);
    e.set_global(OBJECT_TYPE_FUNCTION_011F91EC, 0u32);
    e.set_global(0x011f_91c7u32, 1u8);
    e.set_global(SET_UP_DONE_011F91C6, 0u8);
    e.set_global(0x011f_91c5u32, 0u8);
    e.set_global(0x011f_91a7u32, 0u8);
    e.set_global(PROFILE_SETTING_011F91C0, 0u32);
    e.set_global(PROFILE_SETTING_011F91BC, 0u32);
    e.set_global(0x011f_91a5u32, 1u8);
    let mut slot = SHADER_TABLE_011F9548;
    while (slot as i32) < (TEXTURE_011F95D4 as i32) {
        let shader = e.mem.u32(slot);
        if shader != 0 {
            release_ni_pointer(e, shader);
            e.mem.set_u32(slot, 0);
        }
        slot += 4;
    }
    for global in [
        0x011f_950c,
        TEXTURE_011F9514,
        TEXTURE_011F9510,
        TEXTURE_011F9518,
        TEXTURE_011F951C,
        TEXTURE_011F9520,
        TEXTURE_011F9524,
        TEXTURE_011F9528,
        TEXTURE_011F952C,
        TEXTURE_011F9530,
        TEXTURE_011F9534,
        TEXTURE_011F9538,
        TEXTURE_011F953C,
        TEXTURE_011F9540,
        TEXTURE_011F9544,
        TEXTURE_011F95D4,
    ] {
        release_global(e, global);
    }
    e.set_global(0x011f_91c4u32, 0u8);
    e.set_global(0x011a_d80cu32, 1u8);
    e.set_global(0x011f_91dcu32, 0u8);
    e.set_global(COPIED_FLAG_011F91A4, 1u8);
    e.set_global(0x011f_9178u32, 0u32);
    release_global(e, ACCUMULATOR);
    e.set_global(0x011f_9181u32, 0u8);
    let value = x87_quiet_bits(e.global::<u32>(DEFAULT_VALUE_0102F0F8));
    fn_00b54000(e, f32::from_bits(value));
    for address in [0x011f_9190u32, 0x011f_918c, 0x011f_9188, 0x011f_9184] {
        e.set_global(address, 1.0f32);
    }
    e.set_global(0x011f_91a3u32, 0u8);
    e.set_global(0x011f_9182u32, 0u8);
    e.set_global(0x011f_9180u32, 0u8);
    e.set_global(0x011f_917cu32, 0u32);
    e.set_global(VERSION_3_BOTH_011F94A4, 0u8);
    e.set_global(SHADER_1D_SELECT_011F94A6, 0u8);
    e.set_global(OBJECT_011F91AC, 0u32);
    this
}

// Translated from 00b54620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the accumulator pointer's constructor the
/// set-up passes to the array helper): clears the word at `this`.
pub fn fn_00b54620(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 00b54630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b54630 - the /GS cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (cdecl): the shader manager's shut-down. With
/// `full` set: runs `00b62a20`, `00b59940`, `00ba50d0`, `00bc0940` and
/// `00bc4610`; releases the textures `011f950c` to `011f9544` and
/// `011f95d4`; clears `011f917c`; destroys the objects at `011f91ac`
/// (`00b97690`) and `011f91a8` (`00b6f0b0`) and frees them; releases the
/// accumulator `011f95ec` and the 0x23 shaders of `011f9548` (formatting a
/// leak message into a local buffer for each one still referenced
/// elsewhere); frees the holder at `011f94bc` (releasing its string unless
/// it is the empty one); clears `011f91c6`; stops the rendering threads
/// (`00ba2d80`), runs `00a29680` and releases `011f95f0`. Always: releases
/// `011f9508` and `011f95d8` (clearing `011f91e0`), and every accumulator
/// of the array at `011f9170` (`011f916c` of them), then destroys the array
/// (an `operator new[]` block: its count before it).
pub fn fn_00b54630(e: &mut Engine, full: u8) {
    if full != 0 {
        for address in [
            0x00b6_2a20u32,
            0x00b5_9940,
            0x00ba_50d0,
            0x00bc_0940,
            0x00bc_4610,
        ] {
            e.call(address, &args![]);
        }
        for global in [
            0x011f_950c,
            TEXTURE_011F9510,
            TEXTURE_011F9514,
            TEXTURE_011F9518,
            TEXTURE_011F951C,
            TEXTURE_011F9520,
            TEXTURE_011F9524,
            TEXTURE_011F9528,
            TEXTURE_011F952C,
            TEXTURE_011F9530,
            TEXTURE_011F9534,
            TEXTURE_011F9538,
            TEXTURE_011F953C,
            TEXTURE_011F9540,
            TEXTURE_011F9544,
            TEXTURE_011F95D4,
        ] {
            release_global(e, global);
        }
        e.set_global(0x011f_917cu32, 0u32);
        let object = e.global::<u32>(OBJECT_011F91AC);
        if object != 0 {
            e.call(0x00b9_7690, &args![object]);
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, object]);
            e.set_global(OBJECT_011F91AC, 0u32);
        }
        let object = e.global::<u32>(OBJECT_011F91A8);
        if object != 0 {
            e.call(0x00b6_f0b0, &args![object]);
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, object]);
            e.set_global(OBJECT_011F91A8, 0u32);
        }
        release_global(e, ACCUMULATOR);
        e.with_stack(0x100, |e, buffer| {
            for index in 0..0x23u32 {
                let slot = SHADER_TABLE_011F9548 + 4 * index;
                let shader = e.mem.u32(slot);
                if shader == 0 {
                    continue;
                }
                let count = e.mem.u32(shader + 4);
                if count > 1 {
                    e.call(
                        FORMAT_00406D00,
                        &args![buffer, 0xffu32, SHADER_LEAKING_010AD6E8, index, count],
                    );
                }
                let shader = e.mem.u32(slot);
                if shader != 0 {
                    release_ni_pointer(e, shader);
                    e.mem.set_u32(slot, 0);
                }
            }
        });
        let holder = e.global::<u32>(HOLDER_011F94BC);
        if holder != 0 {
            let name = e.mem.u32(holder);
            if name != e.global::<u32>(FIXED_STRING_NONE) {
                e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
            }
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, holder]);
        }
        e.set_global(HOLDER_011F94BC, 0u32);
        e.set_global(SET_UP_DONE_011F91C6, 0u8);
        e.call(STOP_RENDER_THREADS, &args![RENDER_THREADS_01200088]);
        e.call(0x00a2_9680, &args![]);
        release_global(e, SECOND_ACCUMULATOR_011F95F0);
    }
    release_global(e, GLOBAL_011F9508);
    e.set_global(0x011f_91e0u32, 0u32);
    release_global(e, 0x011f_95d8);
    let mut array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
    let mut index = 0u32;
    while index < e.global::<u32>(ACCUMULATOR_COUNT_011F916C) {
        let slot = array.wrapping_add(4 * index);
        let accumulator = e.mem.u32(slot);
        if accumulator != 0 {
            release_ni_pointer(e, accumulator);
            e.mem.set_u32(slot, 0);
            array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
        }
        index += 1;
    }
    if array != 0 {
        let count = e.mem.u32(array - 4);
        e.call(0x00ec_5fce, &args![array, 4u32, count, 0x0045_cec0u32]);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, array - 4]);
    }
}

// Translated from 00b54ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::SetAccumulator` (engine map; cdecl): `accumulator`
/// into the reference-counted global `011f95ec`.
pub fn bs_shader_manager_set_accumulator(e: &mut Engine, accumulator: u32) {
    assign_global(e, ACCUMULATOR, accumulator);
}

// Translated from 00b54b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): `object` into the reference-counted
/// global `011f95f0`.
pub fn fn_00b54b10(e: &mut Engine, object: u32) {
    assign_global(e, SECOND_ACCUMULATOR_011F95F0, object);
}

// Translated from 00b55360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::SetObjectTypeRecurse` (engine map; cdecl): for
/// geometry (vtable +0x18), `NiAVObject::GetProperty(3)` (nothing more:
/// the result is not used); for a node (vtable +0xC), recurses into each
/// non-null child (`+0xA0`, `+0xA6` of them) with `object_type`.
pub fn bs_shader_manager_set_object_type_recurse(e: &mut Engine, object: u32, object_type: u32) {
    if object == 0 {
        return;
    }
    if e.vcall(object, 0x18, &args![]).u32() != 0 {
        e.call(GET_PROPERTY, &args![object, 3u32]);
        return;
    }
    if e.vcall(object, 0xc, &args![]).u32() == 0 {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(object + 0xa6)) {
        let children = e.mem.u32(object + 0xa0);
        let child = e.mem.u32(children + 4 * index);
        if child != 0 {
            bs_shader_manager_set_object_type_recurse(e, child, object_type);
        }
        index += 1;
    }
}

// Translated from 00b55480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): for geometry, when both its property 2
/// and its shader property (3) exist and the shader property's flags (+0x20)
/// have bit 29, sets property 2's word +0x34 to `value`; for a node (the
/// object vtable +0xC answers), recurses into its non-null children.
pub fn fn_00b55480(e: &mut Engine, object: u32, value: u32) {
    if object == 0 {
        return;
    }
    if e.vcall(object, 0x18, &args![]).u32() != 0 {
        let first = e.call(GET_PROPERTY, &args![object, 2u32]).u32();
        let shader_property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
        if first != 0
            && shader_property != 0
            && e.mem.u32(shader_property + 0x20) & 0x2000_0000 != 0
        {
            e.mem.set_u32(first + 0x34, value);
        }
        return;
    }
    let node = e.vcall(object, 0xc, &args![]).u32();
    if node == 0 {
        return;
    }
    let count = u32::from(e.mem.u16(node + 0xa6));
    for index in 0..count {
        let children = e.mem.u32(node + 0xa0);
        let child = e.mem.u32(children + 4 * index);
        if child != 0 {
            fn_00b55480(e, child, value);
        }
    }
}

// Translated from 00b55520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::GetFogProperty` (engine map; cdecl): the fog property
/// (+0x134) of shadow scene node `index` (`011f91c8`).
pub fn bs_shader_manager_get_fog_property(e: &mut Engine, index: u32) -> u32 {
    let node = e
        .mem
        .u32(SHADOW_SCENE_NODES.wrapping_add(index.wrapping_mul(4)));
    e.mem.u32(node + 0x134)
}

// Translated from 00b55a10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b55a10 - the C++ exception frame (FS:[0] chain, unwind state 0 around the creation) is not translated.
/// Unnamed in the engine map: with a renderer (`011f9508`), makes the debug
/// shader (`DebugShader::CreateShader`, with the thread block's allocation
/// context at 5, restored after) into `011f956c` when there is none, then
/// runs `00bd1cf0(0)` on it when it exists.
pub fn fn_00b55a10(e: &mut Engine) {
    if e.global::<u32>(GLOBAL_011F9508) == 0 {
        return;
    }
    if e.global::<u32>(DEBUG_SHADER_011F956C) == 0 {
        let context = e.tls() + TLS_CONTEXT;
        let saved = e.mem.u32(context);
        e.mem.set_u32(context, 5);
        let shader = e.call(DEBUG_SHADER_CREATE, &args![]).u32();
        e.call(
            NI_POINTER_ASSIGN_00A5F860,
            &args![DEBUG_SHADER_011F956C, shader],
        );
        e.mem.set_u32(context, saved);
        if e.global::<u32>(DEBUG_SHADER_011F956C) == 0 {
            return;
        }
    }
    let shader = e.global::<u32>(DEBUG_SHADER_011F956C);
    e.call(DEBUG_SHADER_00BD1CF0, &args![shader, 0u32]);
}

// Translated from 00b55ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): `00b97900(index, &second, &third)` on
/// the object at `011f91ac` (the two words' stack slots), then the
/// renderer's object at +0x8B8 gets its vtable +0xDC with `(stage, 0)` for
/// the stages 0 to 7.
pub fn fn_00b55ac0(e: &mut Engine, index: u32, second: u32, third: u32) {
    e.with_stack(8, |e, slots| {
        e.mem.set_u32(slots.addr(), second);
        e.mem.set_u32(slots.addr() + 4, third);
        let object = e.global::<u32>(OBJECT_011F91AC);
        e.call(
            QUERY_00B97900,
            &args![object, index, slots.addr(), slots.addr() + 4],
        );
    });
    for stage in 0..8u32 {
        let renderer = e.global::<u32>(GLOBAL_011F9508);
        let state = e.mem.u32(renderer + 0x8b8);
        e.vcall(state, 0xdc, &args![stage, 0u32]);
    }
}

/// The object type from the function at `011f91ec` (cdecl (object)), or 0
/// when it is not set.
fn object_type(e: &mut Engine, object: u32) -> u32 {
    let function = e.global::<u32>(OBJECT_TYPE_FUNCTION_011F91EC);
    if function != 0 {
        e.call(function, &args![object]).u32()
    } else {
        0
    }
}

// Translated from 00b55b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::SetObjectType` (engine map; cdecl): for a non-null
/// `object`, `SetObjectTypeRecurse` with the type from `011f91ec`.
pub fn bs_shader_manager_set_object_type(e: &mut Engine, object: u32) {
    if object != 0 {
        let kind = object_type(e, object);
        bs_shader_manager_set_object_type_recurse(e, object, kind);
    }
}

// Translated from 00b579e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b579e0 - the /GS cookie check (00ec408c) is not translated.
/// `BSShaderManager::PrepareGeometry` (engine map; cdecl, `geometry`, its
/// object type (not read), `force` and `skip`). Without a usable shader
/// property (property 3, its word +0x1C not -1) formats the "does not have
/// a shader property" message into a local buffer and returns. Otherwise
/// the property's vtable +0xB0 `(geometry, 0, 1)` may give a replacement,
/// which takes the old one's place (removed, attached, and when its vtable
/// +0x8C answers at most 7, assigned to the geometry's slot +0x9C + 4 *
/// that). Without a shader number (+0x58) the property is removed;
/// otherwise the shader (`GetShader`) goes to the geometry's +0xC0 and gets
/// it (vtable +0x50); the low bits of the geometry's properties byte
/// (+0xB8, +0x38) get 0x33, 4 for the shader class `01202040` and 8 with
/// `force` or that class. Unless `skip` (or-ed with "the shader is one of
/// `012021b8`/`01200508`, or the geometry's vtable +0x30 answers") is set,
/// the renderer object at `011f4748` is told (vtable +0xEC: geometry, 0x12,
/// 4, the shader's vtable +0x118 for geometry and property).
pub fn bs_shader_manager_prepare_geometry(
    e: &mut Engine,
    geometry: u32,
    _unused_2: u32,
    force: u8,
    skip: u8,
) {
    let mut skip = skip;
    let found = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
    let mut property = if found != 0 && e.mem.u32(found + 0x1c).wrapping_add(1) != 0 {
        found
    } else {
        0
    };
    if property == 0 {
        let name = e.mem.u32(geometry + 8);
        e.with_stack(0x3e8, |e, buffer| {
            e.call(
                FORMAT_00406D00,
                &args![buffer, 0x3e8u32, NO_SHADER_PROPERTY_010ADA44, name],
            );
        });
        return;
    }
    let replacement = e.vcall(property, 0xb0, &args![geometry, 0u32, 1u32]).u32();
    if replacement != 0 {
        e.call(REMOVE_PROPERTY, &args![geometry, 3u32]);
        property = replacement;
        e.call(ATTACH_PROPERTY, &args![geometry, replacement]);
        if e.vcall(replacement, 0x8c, &args![]).i32() <= 7 {
            let slot = e.vcall(replacement, 0x8c, &args![]).u32();
            e.call(
                NI_POINTER_ASSIGN_0066B0D0,
                &args![
                    geometry
                        .wrapping_add(slot.wrapping_mul(4))
                        .wrapping_add(0x9c),
                    replacement
                ],
            );
        }
    }
    let shader_number = e.mem.u32(property + 0x58);
    if shader_number == 0 {
        e.call(REMOVE_PROPERTY, &args![geometry, 3u32]);
        return;
    }
    let shader = bs_shader_manager_get_shader(e, shader_number).addr();
    e.mem.set_u32(geometry + 0xc0, shader);
    e.vcall(shader, 0x50, &args![geometry]);
    let first_class = e.vcall(shader, 8, &args![]).u32() == SHADER_RTTI_012021B8;
    let second_class = e.vcall(shader, 8, &args![]).u32() == SHADER_RTTI_01200508;
    let flag = if second_class || first_class {
        1
    } else {
        u8::from(e.vcall(geometry, 0x30, &args![]).u32() != 0)
    };
    skip |= flag;
    let third_class = e.vcall(shader, 8, &args![]).u32() == SHADER_RTTI_01202040;
    let properties = e.mem.u32(geometry + 0xb8);
    let low = e.mem.u8(properties + 0x38) | 0x33;
    let forced = if force != 0 || third_class { 8 } else { 0 };
    let class_bit = if third_class { 4 } else { 0 };
    e.mem.set_u8(properties + 0x38, class_bit | forced | low);
    if skip != 0 {
        return;
    }
    let renderer = e.global::<u32>(RENDERER_OBJECT_011F4748);
    let renderer_vtable = e.mem.u32(renderer);
    let state = e.vcall(shader, 0x118, &args![geometry, property]).u32();
    let target = e.mem.u32(renderer_vtable + 0xec);
    e.call(target, &args![renderer, geometry, 0x12u32, 4u32, state]);
}

// Translated from 00b57bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::PrepareObjectRecurse` (engine map; cdecl): geometry
/// goes to `PrepareGeometry`, a node's non-null children recurse; anything
/// else is left.
pub fn bs_shader_manager_prepare_object_recurse(
    e: &mut Engine,
    object: u32,
    object_type: u32,
    force: u8,
    skip: u8,
) {
    if e.vcall(object, 0xc, &args![]).u32() == 0 && e.vcall(object, 0x18, &args![]).u32() == 0 {
        return;
    }
    if e.vcall(object, 0x18, &args![]).u32() != 0 {
        bs_shader_manager_prepare_geometry(e, object, object_type, force, skip);
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(object + 0xa6)) {
        let children = e.mem.u32(object + 0xa0);
        let child = e.mem.u32(children + 4 * index);
        if child != 0 {
            bs_shader_manager_prepare_object_recurse(e, child, object_type, force, skip);
        }
        index += 1;
    }
}

// Translated from 00b57c60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b57c60 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 around the clone) is not translated.
/// `BSShaderManager::CloneMaterialPropertyRecurse` (engine map; cdecl): for
/// an object with a material property (property 2, held with a reference):
/// `00654640` with it, a stack scope object (`004ad0c0(0x101)`), then the
/// property itself when its word +0xC is set, else a new
/// `NiMaterialProperty` (0x4C bytes) with its colours (+0x1C to +0x30, the
/// emissive block from +0x34 or +0x28) and floats (+0x38 to +0x40) copied
/// and its revision (+0x44) stepped once per change, attached to the
/// object; the scope's two objects are deleted. Then a node's children
/// recurse, and the reference is released.
pub fn bs_shader_manager_clone_material_property_recurse(e: &mut Engine, object: u32) {
    if object == 0 {
        return;
    }
    let property = e.call(GET_PROPERTY, &args![object, 2u32]).u32();
    if property != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![property + 4]);
        e.call(OBJECT_00654640, &args![object, property]);
        e.with_stack(0x10, |e, scope| {
            let scope = scope.addr();
            e.call(SCOPE_CONSTRUCT_004AD0C0, &args![scope, 0x101u32]);
            let attached = if e.mem.u32(property + 0xc) != 0 {
                property
            } else {
                let block = e.call(NEW_OBJECT, &args![0x4cu32]).u32();
                let copy = if block != 0 {
                    e.call(MATERIAL_PROPERTY_CONSTRUCT, &args![block]).u32()
                } else {
                    0
                };
                for offset in [0x1c, 0x20] {
                    let word = e.mem.u32(property + offset);
                    e.mem.set_u32(copy + offset, word);
                }
                let word = e.mem.u32(property + 0x24);
                let revision = e.mem.u32(copy + 0x44).wrapping_add(1);
                e.mem.set_u32(copy + 0x44, revision);
                e.mem.set_u32(copy + 0x24, word);
                let emissive = if e.mem.u32(property + 0x34) != 0 {
                    e.mem.u32(property + 0x34)
                } else {
                    property + 0x28
                };
                for word in 0..3 {
                    let value = e.mem.u32(emissive + 4 * word);
                    e.mem.set_u32(copy + 0x28 + 4 * word, value);
                }
                let mut revision = revision.wrapping_add(1);
                e.mem.set_u32(copy + 0x44, revision);
                for offset in [0x40, 0x38, 0x3c] {
                    let value = x87_quiet_bits(e.mem.u32(property + offset));
                    revision = revision.wrapping_add(1);
                    e.mem.set_u32(copy + 0x44, revision);
                    e.mem.set_u32(copy + offset, value);
                }
                copy
            };
            e.call(ATTACH_PROPERTY, &args![object, attached]);
            for offset in [0, 4] {
                let held = e.mem.u32(scope + offset);
                if held != 0 {
                    e.vcall(held, 0, &args![1u32]);
                }
            }
        });
    }
    let node = e.vcall(object, 0xc, &args![]).u32();
    if node != 0 {
        let count = u32::from(e.mem.u16(node + 0xa6));
        for index in 0..count {
            let children = e.mem.u32(node + 0xa0);
            let child = e.mem.u32(children + 4 * index);
            if child != 0 {
                bs_shader_manager_clone_material_property_recurse(e, child);
            }
        }
    }
    if property != 0 {
        release_ni_pointer(e, property);
    }
}

// Translated from 00b57e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::PrepareObject` (engine map; cdecl): updates the
/// object's properties (`NiAVObject::UpdateProperties`), then
/// `PrepareObjectRecurse` with the type from `011f91ec`, `force` and `skip`.
pub fn bs_shader_manager_prepare_object(e: &mut Engine, object: u32, force: u8, skip: u8) {
    e.call(UPDATE_PROPERTIES, &args![object]);
    let kind = object_type(e, object);
    bs_shader_manager_prepare_object_recurse(e, object, kind, force, skip);
}

/// `"STBB"`, the prefix `CreateTangentSpaceSimple` compares the name with
/// (the result is not used).
const STBB_0102BF34: u32 = 0x0102_bf34;
/// `"Tangent space (binormal & tangent vectors)"`, the extra data's name.
const TANGENT_SPACE_NAME_010AD7A8: u32 = 0x010a_d7a8;
/// The message formatted when the geometry has no normals.
const NO_NORMALS_MESSAGE_010AD730: u32 = 0x010a_d730;
/// Thiscall on a new 0x14-byte block (word), `RET 4`: the object
/// `CreateTangentSpaceSimple` answers.
const TANGENT_DATA_CONSTRUCT: u32 = 0x00b6_6810;
/// `NiObjectNET::GetExtraData` / `RemoveExtraData` (engine map), thiscall
/// (address of the name), `RET 4`.
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const REMOVE_EXTRA_DATA: u32 = 0x00a5_be90;
/// `NiTriStripsData::GetStripLists` (engine map), thiscall.
const GET_STRIP_LISTS: u32 = 0x00a7_5f10;
/// Thiscall on a vector (`bgsdecalmanager.rs`): normalizes it.
const NORMALIZE_004A0C10: u32 = 0x004a_0c10;
/// `NiGeometryData::ReplaceData` (engine map), thiscall (count, vertices,
/// normals, colours, texture coordinates, flags, word), `RET 0x1C`.
const REPLACE_DATA: u32 = 0x00a6_6bb0;
/// `_memcpy` and `_memset` (engine map), cdecl.
const MEMCPY_00EC44D0: u32 = 0x00ec_44d0;
const MEMSET_00EC61C0: u32 = 0x00ec_61c0;

/// `count * 12` as the code's `MUL`/`SETO`/`NEG`/`OR` computes it: all
/// ones on overflow.
fn twelve_times(count: u32) -> u32 {
    count.checked_mul(0xc).unwrap_or(u32::MAX)
}

/// A float's bits through `FLD dword` and `FSTP dword` of an x87 result.
fn store(value: f64) -> u32 {
    x87_store32(value)
}

/// `FLD dword` of the float at `address`.
fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// A float memory operand at `address`.
fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

// Translated from 00b54b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b54b60 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 around the names) and the /GS cookie check (00ec408c) are not translated.
/// `BSShaderManager::CreateTangentSpaceSimple` (engine map; cdecl). Makes a
/// 0x14-byte object (`00b66810(1)`). When the geometry has the tangent
/// space extra data, the object gets two copies of its two halves (+0xC the
/// second, +0x10 the first, 12 bytes per vertex), the extra data is removed
/// and the object is returned. Otherwise, for a triangle shape (vtable
/// +0x1C) with normals, it sums each triangle's tangent and binormal
/// (from the positions and texture coordinates, through `1 / (du1 * dv2 -
/// du2 * dv1)`) into each of its three vertices (strips read with the
/// flipped winding every other triangle; degenerate triangles skipped),
/// makes each vertex's tangent orthogonal to its normal and the binormal
/// their cross product (both normalized, `004a0c10`), hands the geometry
/// data the normals followed by the binormals and tangents
/// (`NiGeometryData::ReplaceData`), frees the old normals and the work
/// buffer, deletes the object and returns 0. Without normals a message is
/// formatted into a local buffer and 0 returned (the object kept).
pub fn bs_shader_manager_create_tangent_space_simple(e: &mut Engine, geometry: u32) -> u32 {
    let name = e.mem.u32(geometry + 8);
    if name != 0 {
        e.call(STRNICMP, &args![name, STBB_0102BF34, 4u32]);
    }
    let block = e.call(NEW_OBJECT, &args![0x14u32]).u32();
    let object = if block != 0 {
        e.call(TANGENT_DATA_CONSTRUCT, &args![block, 1u32]).u32()
    } else {
        0
    };
    let existing = e.with_stack(4, |e, slot| {
        let fixed = e
            .call(FIXED_STRING_MAKE, &args![TANGENT_SPACE_NAME_010AD7A8])
            .u32();
        e.mem.set_u32(slot.addr(), fixed);
        let found = e.call(GET_EXTRA_DATA, &args![geometry, slot]).u32();
        let fixed = e.mem.u32(slot.addr());
        release_fixed_string(e, fixed);
        found
    });
    if existing != 0 {
        let data = e.mem.u32(geometry + 0xb8);
        let count = u32::from(e.mem.u16(data + 8));
        let first = e.mem.u32(existing + 0xc);
        let size = count * 12;
        let second = first.wrapping_add(size);
        let copy = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, twelve_times(count)])
            .u32();
        e.mem.set_u32(object + 0xc, copy);
        let copy = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, twelve_times(count)])
            .u32();
        e.mem.set_u32(object + 0x10, copy);
        let destination = e.mem.u32(object + 0xc);
        e.call(MEMCPY_00EC44D0, &args![destination, second, size]);
        let destination = e.mem.u32(object + 0x10);
        e.call(MEMCPY_00EC44D0, &args![destination, first, size]);
        e.with_stack(4, |e, slot| {
            let fixed = e
                .call(FIXED_STRING_MAKE, &args![TANGENT_SPACE_NAME_010AD7A8])
                .u32();
            e.mem.set_u32(slot.addr(), fixed);
            e.call(REMOVE_EXTRA_DATA, &args![geometry, slot]);
            let fixed = e.mem.u32(slot.addr());
            release_fixed_string(e, fixed);
        });
        return object;
    }
    if e.vcall(geometry, 0x1c, &args![]).u32() == 0 {
        return 0;
    }
    let shape = e.vcall(geometry, 0x1c, &args![]).u32();
    let data = e.mem.u32(shape + 0xb8);
    let triangles = e.mem.u16(data + 0x40);
    let normals = e.mem.u32(data + 0x24);
    let count = u32::from(e.mem.u16(data + 8));
    let vertices = e.mem.u32(data + 0x20);
    let coordinates = e.mem.u32(data + 0x2c);
    let strips = e.vcall(shape, 0x20, &args![]).u32() != 0;
    let mut strip = 0u32;
    if strips {
        let data = e.mem.u32(shape + 0xb8);
        strip = e.call(GET_STRIP_LISTS, &args![data]).u32();
    }
    if normals == 0 {
        e.with_stack(0x100, |e, buffer| {
            e.call(
                FORMAT_00406D00,
                &args![buffer, 0xffu32, NO_NORMALS_MESSAGE_010AD730],
            );
        });
        return 0;
    }
    let sums = e
        .call(
            MEMORY_ALLOCATE,
            &args![MEMORY_MANAGER, twelve_times(count * 2)],
        )
        .u32();
    let binormal_sums = sums.wrapping_add(count * 12);
    e.call(MEMSET_00EC61C0, &args![sums, 0u32, count * 24]);
    let output = e
        .call(
            MEMORY_ALLOCATE,
            &args![MEMORY_MANAGER, twelve_times(count * 3)],
        )
        .u32();
    e.call(MEMCPY_00EC44D0, &args![output, normals, count * 12]);
    let binormals_out = output.wrapping_add(count * 12);
    let tangents_out = output.wrapping_add(count * 24);
    // The locals the triangle's indices are read into (words; the
    // triangle query writes their low halves).
    e.with_stack(0xc, |e, indices| {
        let slot0 = indices.addr();
        let slot1 = slot0 + 4;
        let slot2 = slot0 + 8;
        let mut flip = false;
        let mut triangle = 0u32;
        let mut cursor = strip.wrapping_add(4);
        if triangles == 0 {
            return;
        }
        loop {
            if strips {
                let a = u32::from(e.mem.u16(cursor.wrapping_sub(4)));
                let b = u32::from(e.mem.u16(cursor.wrapping_sub(2)));
                let c = u32::from(e.mem.u16(cursor));
                e.mem.set_u32(slot0, a);
                e.mem.set_u32(slot1, b);
                e.mem.set_u32(slot2, c);
                if flip {
                    e.mem.set_u32(slot2, a);
                    e.mem.set_u32(slot0, c);
                }
                flip = !flip;
            } else {
                let data = e.mem.u32(shape + 0xb8);
                e.vcall(data, 0xac, &args![triangle, slot0, slot1, slot2]);
            }
            let i0 = e.mem.u16(slot0);
            let i1 = e.mem.u16(slot1);
            let i2 = e.mem.u16(slot2);
            if i0 != i1 && i1 != i2 && i0 != i2 {
                add_triangle(
                    e,
                    [u32::from(i0), u32::from(i1), u32::from(i2)],
                    vertices,
                    coordinates,
                    sums,
                    binormal_sums,
                );
            }
            cursor = cursor.wrapping_add(2);
            triangle = triangle.wrapping_add(1);
            if triangle as u16 >= triangles {
                break;
            }
        }
    });
    for index in 0..count {
        let normal = normals + 12 * index;
        let sum = sums + 12 * index;
        let tangent = tangents_out + 12 * index;
        let binormal = binormals_out + 12 * index;
        let along = x87_add(
            x87_add(
                x87_mul(load(e, normal), operand(e, sum)),
                x87_mul(load(e, normal + 4), operand(e, sum + 4)),
            ),
            x87_mul(load(e, normal + 8), operand(e, sum + 8)),
        );
        let along = x87_load32(store(along));
        let part_x = store(x87_mul(load(e, normal), along));
        let part_y = store(x87_mul(load(e, normal + 4), along));
        let part_z = store(x87_mul(along, operand(e, normal + 8)));
        let tangent_x = store(x87_sub(load(e, sum), x87_m32(part_x)));
        let tangent_y = store(x87_sub(load(e, sum + 4), x87_m32(part_y)));
        let tangent_z = store(x87_sub(load(e, sum + 8), x87_m32(part_z)));
        e.mem.set_u32(tangent, tangent_x);
        e.mem.set_u32(tangent + 4, tangent_y);
        e.mem.set_u32(tangent + 8, tangent_z);
        let cross_x = store(x87_sub(
            x87_mul(load(e, tangent + 8), operand(e, normal + 4)),
            x87_mul(load(e, normal + 8), operand(e, tangent + 4)),
        ));
        let cross_y = store(x87_sub(
            x87_mul(load(e, normal + 8), operand(e, tangent)),
            x87_mul(load(e, tangent + 8), operand(e, normal)),
        ));
        let cross_z = store(x87_sub(
            x87_mul(load(e, normal), operand(e, tangent + 4)),
            x87_mul(load(e, normal + 4), operand(e, tangent)),
        ));
        e.mem.set_u32(binormal, cross_x);
        e.mem.set_u32(binormal + 4, cross_y);
        e.mem.set_u32(binormal + 8, cross_z);
        e.call(NORMALIZE_004A0C10, &args![tangent]);
        e.call(NORMALIZE_004A0C10, &args![binormal]);
    }
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, sums]);
    let data = e.mem.u32(geometry + 0xb8);
    let old_normals = e.mem.u32(data + 0x24);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, old_normals]);
    let flags = u32::from(e.mem.u16(data + 0xc) & 1);
    let coordinates = e.mem.u32(data + 0x2c);
    let colours = e.mem.u32(data + 0x28);
    let vertices = e.mem.u32(data + 0x20);
    e.call(
        REPLACE_DATA,
        &args![
            data,
            count,
            vertices,
            output,
            colours,
            coordinates,
            flags,
            0x1000u32
        ],
    );
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    0
}

/// One triangle of `CreateTangentSpaceSimple`: its tangent and binormal
/// from the edge vectors and texture coordinate differences (in the code's
/// x87 order, each stored as a `float`), added to the sums of its three
/// vertices.
fn add_triangle(
    e: &mut Engine,
    [i0, i1, i2]: [u32; 3],
    vertices: u32,
    coordinates: u32,
    sums: u32,
    binormal_sums: u32,
) {
    let v0 = vertices + 12 * i0;
    let v1 = vertices + 12 * i1;
    let v2 = vertices + 12 * i2;
    let edge = |e: &Engine, to: u32, offset: u32| {
        store(x87_sub(load(e, to + offset), operand(e, v0 + offset)))
    };
    let e1x = edge(e, v1, 0);
    let e2x = edge(e, v2, 0);
    let e1y = edge(e, v1, 4);
    let e2y = edge(e, v2, 4);
    let e1z = edge(e, v1, 8);
    let e2z = edge(e, v2, 8);
    let uv0 = coordinates + 8 * i0;
    let uv1 = coordinates + 8 * i1;
    let uv2 = coordinates + 8 * i2;
    let du1 = x87_load32(store(x87_sub(load(e, uv1), operand(e, uv0))));
    let du2 = x87_load32(store(x87_sub(load(e, uv2), operand(e, uv0))));
    let dv1 = x87_load32(store(x87_sub(load(e, uv1 + 4), operand(e, uv0 + 4))));
    let dv2 = x87_load32(store(x87_sub(load(e, uv2 + 4), operand(e, uv0 + 4))));
    let determinant = x87_sub(x87_mul(dv2, du1), x87_mul(dv1, du2));
    let inverse = x87_load32(store(x87_div(1.0, determinant)));
    let tangent_x = store(x87_mul(
        x87_sub(x87_mul(dv2, x87_m32(e1x)), x87_mul(dv1, x87_m32(e2x))),
        inverse,
    ));
    let tangent_y = store(x87_mul(
        x87_sub(x87_mul(dv2, x87_m32(e1y)), x87_mul(dv1, x87_m32(e2y))),
        inverse,
    ));
    let e1z = x87_load32(e1z);
    let e2z = x87_load32(e2z);
    let tangent_z = store(x87_mul(
        x87_sub(x87_mul(dv2, e1z), x87_mul(dv1, e2z)),
        inverse,
    ));
    let binormal_x = store(x87_mul(
        x87_sub(x87_mul(du1, x87_m32(e2x)), x87_mul(du2, x87_m32(e1x))),
        inverse,
    ));
    let binormal_y = store(x87_mul(
        x87_sub(x87_mul(du1, x87_m32(e2y)), x87_mul(du2, x87_m32(e1y))),
        inverse,
    ));
    let binormal_z = store(x87_mul(
        inverse,
        x87_sub(x87_mul(e2z, du1), x87_mul(du2, e1z)),
    ));
    let tangent = [tangent_x, tangent_y, tangent_z].map(x87_load32);
    let binormal = [binormal_x, binormal_y, binormal_z].map(x87_load32);
    // The x87 form of each addition, as the code orders its operands:
    // `ValueAndOperand` is `FLD value; FADD [sum]`, `SumAndValue` loads the
    // sum and adds the value into it, `ValueAndSum` loads the sum and adds
    // it into the value's register.
    use Addition::{SumAndValue as S, ValueAndOperand as A, ValueAndSum as C};
    let t0 = sums + 12 * i0;
    let t1 = sums + 12 * i1;
    let t2 = sums + 12 * i2;
    for (at, modes) in [(t0, [A, S, S]), (t1, [A, S, A]), (t2, [C, A, A])] {
        for axis in 0..3 {
            add_to(e, at + 4 * axis as u32, tangent[axis], modes[axis]);
        }
    }
    let b0 = binormal_sums + 12 * i0;
    let b1 = binormal_sums + 12 * i1;
    let b2 = binormal_sums + 12 * i2;
    for (at, modes) in [(b0, [S, A, S]), (b1, [S, A, S]), (b2, [C, A, A])] {
        for axis in 0..3 {
            add_to(e, at + 4 * axis as u32, binormal[axis], modes[axis]);
        }
    }
}

/// How one sum of `CreateTangentSpaceSimple` is added (see `add_to`).
#[derive(Clone, Copy)]
enum Addition {
    ValueAndOperand,
    SumAndValue,
    ValueAndSum,
}

/// The float at `at` plus `value`, stored back as a `float`.
fn add_to(e: &mut Engine, at: u32, value: f64, mode: Addition) {
    let sum = match mode {
        Addition::ValueAndOperand => x87_add(value, operand(e, at)),
        Addition::SumAndValue => x87_add(load(e, at), value),
        Addition::ValueAndSum => x87_add(value, load(e, at)),
    };
    e.mem.set_u32(at, store(sum));
}

/// `NiFile::GetFile` (`00aa1a20`, `cdecl`, three words), which `00b4f2e0` jumps to.
const NI_FILE_GET_FILE: u32 = 0x00aa_1a20;
/// The end of the shader table (`011f95d4`, one past index 0x22).
const SHADER_TABLE_END_011F95D4: u32 = 0x011f_95d4;
/// The two counters `00b540d0` lowers (`011ffa10`, `011ffa0c`) and the call it then makes
/// (`00b984f0`, `cdecl`, two words).
const COUNTER_011FFA10: u32 = 0x011f_fa10;
const COUNTER_011FFA0C: u32 = 0x011f_fa0c;
const FUNCTION_00B984F0: u32 = 0x00b9_84f0;
/// The run-time type record `00b557d0` looks for in each shader's chain (`0126f554`, the one
/// `00b8bd90` answers).
const RTTI_0126F554: u32 = 0x0126_f554;
/// The `ImageSpaceManager` method `00b8b3b0` (`thiscall`, no words) `00b557d0` ends with.
const IMAGE_SPACE_MANAGER_00B8B3B0: u32 = 0x00b8_b3b0;
/// The calls `00b578a0` makes on a reset (all without stack words): three set-ups, the
/// destructors of the objects at `011f91ac` (`00b97690`) and `011f91a8` (`00b6f0b0`), their
/// constructors (`00b97a80` for 0x31c bytes, `00b6dd80` for 0xb0 bytes) and `00b6e450`
/// (`thiscall` on the object at `011f91a8`, one word).
const RESET_00BA50D0: u32 = 0x00ba_50d0;
const RESET_00BC0940: u32 = 0x00bc_0940;
const RESET_00BC4610: u32 = 0x00bc_4610;
const DESTRUCTOR_00B97690: u32 = 0x00b9_7690;
const DESTRUCTOR_00B6F0B0: u32 = 0x00b6_f0b0;
const CONSTRUCTOR_00B97A80: u32 = 0x00b9_7a80;
const CONSTRUCTOR_00B6DD80: u32 = 0x00b6_dd80;
const METHOD_00B6E450: u32 = 0x00b6_e450;
/// The word `00b578a0` clears (`011f91e0`).
const GLOBAL_011F91E0: u32 = 0x011f_91e0;
/// The virtual slots (byte offsets) `SetEmittanceRecurse` and `RecursiveSetForcedDarkness`
/// call: `IsGeometry` (+0x18) and `IsNode` (+0xc), as in `bsshaderproperty.rs`.
const SLOT_IS_GEOMETRY: u32 = 0x18;
const SLOT_IS_NODE: u32 = 0x0c;
/// `GetProperty` types: the material property (2) and the shader property (3).
const MATERIAL_PROPERTY_TYPE: u32 = 2;
const SHADER_PROPERTY_TYPE: u32 = 3;
/// The format the pass-name default prints the pass with (`010a9488`), the `sprintf_s` it
/// uses (`00406d00`, `cdecl`: buffer, size, format, value) and the string it answers then
/// (`010a93f4`).
const PASS_NAME_FORMAT_010A9488: u32 = 0x010a_9488;
const SPRINTF_S_00406D00: u32 = 0x0040_6d00;
const PASS_NAME_UNKNOWN_010A93F4: u32 = 0x010a_93f4;

// Translated from 00b4f2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; the engine map's gap list puts it in this unit,
/// stored as a function pointer by `00b54280`): `JMP 00aa1a20`, so the three `cdecl` words
/// go unchanged to `NiFile::GetFile` and its file object comes back.
pub fn fn_00b4f2e0(e: &mut Engine, name: u32, mode: u32, buffer_size: u32) -> u32 {
    e.call(NI_FILE_GET_FILE, &args![name, mode, buffer_size])
        .u32()
}

// Translated from 00b4f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, no arguments: calls slot +0x124 of each non-null shader in
/// the shader table (`011f9548` up to `011f95d4`).
pub fn fn_00b4f9a0(e: &mut Engine) {
    let mut slot = SHADER_TABLE_011F9548;
    while slot < SHADER_TABLE_END_011F95D4 {
        let shader = e.mem.u32(slot);
        if shader != 0 {
            e.vcall(shader, 0x124, &args![]);
        }
        slot = slot.wrapping_add(4);
    }
}

// Translated from 00b4f9d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b4f9d0 - the /GS stack cookie check (00ec408c) is not translated
/// `BSShaderManager::GetPassName` (engine map; the body matches): `cdecl`, one word, the
/// pass enum. Passes 1 to 0x2f5 index the switch's jump table at `00b5342c` (read in
/// Ghidra); each arm answers its name string (the match below: the addresses the arms'
/// `MOV EAX` immediates load). The table's arms at `00b53403` (passes 0x158, 0x159, 0x179
/// to 0x17e, 0x237 and 0x257 to 0x25b) and every other pass print the pass into a 12-byte
/// local buffer (`00406d00` with the buffer, 10, the format `"??? %d"` at `010a9488` and the pass) and
/// answer the string `"???"` at `010a93f4`, not the buffer.
pub fn bs_shader_manager_get_pass_name(e: &mut Engine, pass: u32) -> u32 {
    let name = match pass {
        0x001 => 0x010a_d6c8,
        0x002 => 0x010a_d6b8,
        0x003 => 0x010a_d6a8,
        0x004 => 0x010a_d690,
        0x005 => 0x010a_d678,
        0x006 => 0x010a_d668,
        0x007 => 0x010a_d654,
        0x008 => 0x010a_d640,
        0x009 => 0x010a_d628,
        0x00a => 0x010a_afc4,
        0x00b => 0x010a_afb4,
        0x00c => 0x010a_afa4,
        0x00d => 0x010a_af90,
        0x00e => 0x010a_af78,
        0x00f => 0x010a_af5c,
        0x010 => 0x010a_d618,
        0x011 => 0x010a_d604,
        0x012 => 0x010a_d5f4,
        0x013 => 0x010a_d5e0,
        0x014 => 0x010a_95e0,
        0x015 => 0x010a_95c4,
        0x016 => 0x010a_d13c,
        0x017 => 0x010a_d11c,
        0x018 => 0x010a_d0fc,
        0x019 => 0x010a_d0dc,
        0x01a => 0x010a_d0bc,
        0x01b => 0x010a_d09c,
        0x01c => 0x010a_d07c,
        0x01d => 0x010a_d05c,
        0x01e => 0x010a_d038,
        0x01f => 0x010a_d014,
        0x020 => 0x010a_cff0,
        0x021 => 0x010a_cfcc,
        0x022 => 0x010a_cfa8,
        0x023 => 0x010a_cf84,
        0x024 => 0x010a_cf60,
        0x025 => 0x010a_cf3c,
        0x026 => 0x010a_cf1c,
        0x027 => 0x010a_cefc,
        0x028 => 0x010a_cedc,
        0x029 => 0x010a_ceb8,
        0x02a => 0x010a_ce98,
        0x02b => 0x010a_ce78,
        0x02c => 0x010a_ce58,
        0x02d => 0x010a_ce38,
        0x02e => 0x010a_ce18,
        0x02f => 0x010a_cdf8,
        0x030 => 0x010a_cdd8,
        0x031 => 0x010a_cdb8,
        0x032 => 0x010a_cd98,
        0x033 => 0x010a_cd78,
        0x034 => 0x010a_cd54,
        0x035 => 0x010a_cd30,
        0x036 => 0x010a_cd0c,
        0x037 => 0x010a_cce8,
        0x038 => 0x010a_ccc4,
        0x039 => 0x010a_cca0,
        0x03a => 0x010a_cc80,
        0x03b => 0x010a_cc60,
        0x03c => 0x010a_cc40,
        0x03d => 0x010a_cc20,
        0x03e => 0x010a_cc10,
        0x03f => 0x010a_cbfc,
        0x040 => 0x010a_cbf0,
        0x041 => 0x010a_cbd8,
        0x042 => 0x010a_cbc8,
        0x043 => 0x010a_cbb4,
        0x044 => 0x010a_cb9c,
        0x045 => 0x010a_cb88,
        0x046 => 0x010a_cb04,
        0x047 => 0x010a_caec,
        0x048 => 0x010a_cad4,
        0x049 => 0x010a_cabc,
        0x04a => 0x010a_cb70,
        0x04b => 0x010a_cb54,
        0x04c => 0x010a_cb34,
        0x04d => 0x010a_cb18,
        0x04e => 0x010a_caa0,
        0x04f => 0x010a_ca80,
        0x050 => 0x010a_ca60,
        0x051 => 0x010a_ca40,
        0x052 => 0x010a_ca28,
        0x053 => 0x010a_ca04,
        0x054 => 0x010a_c9e0,
        0x055 => 0x010a_c9ac,
        0x056 => 0x010a_c990,
        0x057 => 0x010a_c96c,
        0x058 => 0x010a_c95c,
        0x059 => 0x010a_c94c,
        0x05a => 0x010a_c93c,
        0x05b => 0x010a_c928,
        0x05c => 0x010a_c918,
        0x05d => 0x010a_c908,
        0x05e => 0x010a_c8f4,
        0x05f => 0x010a_c8e0,
        0x060 => 0x010a_c8d0,
        0x061 => 0x010a_c8c0,
        0x062 => 0x010a_c8b0,
        0x063 => 0x010a_c89c,
        0x064 => 0x010a_c88c,
        0x065 => 0x010a_c878,
        0x066 => 0x010a_c83c,
        0x067 => 0x010a_c828,
        0x068 => 0x010a_c7e8,
        0x069 => 0x010a_c7d4,
        0x06a => 0x010a_c7c0,
        0x06b => 0x010a_c780,
        0x06c => 0x010a_c768,
        0x06d => 0x010a_c754,
        0x06e => 0x010a_c6fc,
        0x06f => 0x010a_c6d0,
        0x070 => 0x010a_c6bc,
        0x071 => 0x010a_c67c,
        0x072 => 0x010a_c664,
        0x073 => 0x010a_c624,
        0x074 => 0x010a_c5f8,
        0x075 => 0x010a_c5e0,
        0x076 => 0x010a_c5a0,
        0x077 => 0x010a_c588,
        0x078 => 0x010a_c570,
        0x079 => 0x010a_c528,
        0x07a => 0x010a_c4fc,
        0x07b => 0x010a_c4e8,
        0x07c => 0x010a_c4d0,
        0x07d => 0x010a_c4b8,
        0x07e => 0x010a_c4a0,
        0x07f => 0x010a_c864,
        0x080 => 0x010a_c850,
        0x081 => 0x010a_c814,
        0x082 => 0x010a_c7fc,
        0x083 => 0x010a_c7ac,
        0x084 => 0x010a_c794,
        0x085 => 0x010a_c740,
        0x086 => 0x010a_c72c,
        0x087 => 0x010a_c714,
        0x088 => 0x010a_c6e4,
        0x089 => 0x010a_c6a8,
        0x08a => 0x010a_c690,
        0x08b => 0x010a_c650,
        0x08c => 0x010a_c638,
        0x08d => 0x010a_c5cc,
        0x08e => 0x010a_c5b4,
        0x08f => 0x010a_c60c,
        0x090 => 0x010a_c558,
        0x091 => 0x010a_c540,
        0x092 => 0x010a_c510,
        0x093 => 0x010a_c494,
        0x094 => 0x010a_b540,
        0x095 => 0x010a_bfa0,
        0x096 => 0x010a_bf94,
        0x097 => 0x010a_bf88,
        0x098 => 0x010a_bf78,
        0x099 => 0x010a_bf68,
        0x09a => 0x010a_bf58,
        0x09b => 0x010a_bf4c,
        0x09c => 0x010a_bf3c,
        0x09d => 0x010a_bf2c,
        0x09e => 0x010a_bf1c,
        0x09f => 0x010a_bf0c,
        0x0a0 => 0x010a_befc,
        0x0a1 => 0x010a_beec,
        0x0a2 => 0x010a_bedc,
        0x0a3 => 0x010a_becc,
        0x0a4 => 0x010a_bebc,
        0x0a5 => 0x010a_bea8,
        0x0a6 => 0x010a_be98,
        0x0a7 => 0x010a_be88,
        0x0a8 => 0x010a_be78,
        0x0a9 => 0x010a_be64,
        0x0aa => 0x010a_be54,
        0x0ab => 0x010a_be48,
        0x0ac => 0x010a_be38,
        0x0ad => 0x010a_be28,
        0x0ae => 0x010a_be18,
        0x0af => 0x010a_be08,
        0x0b0 => 0x010a_bdf8,
        0x0b1 => 0x010a_bde8,
        0x0b2 => 0x010a_bdd8,
        0x0b3 => 0x010a_bdc8,
        0x0b4 => 0x010a_bdb8,
        0x0b5 => 0x010a_bda8,
        0x0b6 => 0x010a_bd98,
        0x0b7 => 0x010a_bd88,
        0x0b8 => 0x010a_bd78,
        0x0b9 => 0x010a_bd64,
        0x0ba => 0x010a_bd50,
        0x0bb => 0x010a_bd3c,
        0x0bc => 0x010a_bd2c,
        0x0bd => 0x010a_bd18,
        0x0be => 0x010a_bd04,
        0x0bf => 0x010a_bcf0,
        0x0c0 => 0x010a_bcdc,
        0x0c1 => 0x010a_bcc8,
        0x0c2 => 0x010a_bcb4,
        0x0c3 => 0x010a_bca8,
        0x0c4 => 0x010a_bc98,
        0x0c5 => 0x010a_bc88,
        0x0c6 => 0x010a_bc78,
        0x0c7 => 0x010a_bc68,
        0x0c8 => 0x010a_bc58,
        0x0c9 => 0x010a_bc4c,
        0x0ca => 0x010a_bc3c,
        0x0cb => 0x010a_bc30,
        0x0cc => 0x010a_bc20,
        0x0cd => 0x010a_bc14,
        0x0ce => 0x010a_bc04,
        0x0cf => 0x010a_bbf4,
        0x0d0 => 0x010a_bbe8,
        0x0d1 => 0x010a_bbd8,
        0x0d2 => 0x010a_bbc8,
        0x0d3 => 0x010a_bbb8,
        0x0d4 => 0x010a_bba8,
        0x0d5 => 0x010a_bb98,
        0x0d6 => 0x010a_bb88,
        0x0d7 => 0x010a_c488,
        0x0d8 => 0x010a_c47c,
        0x0d9 => 0x010a_c470,
        0x0da => 0x010a_c460,
        0x0db => 0x010a_c454,
        0x0dc => 0x010a_c448,
        0x0dd => 0x010a_c43c,
        0x0de => 0x010a_c42c,
        0x0df => 0x010a_c420,
        0x0e0 => 0x010a_c410,
        0x0e1 => 0x010a_c400,
        0x0e2 => 0x010a_c3f0,
        0x0e3 => 0x010a_c3e0,
        0x0e4 => 0x010a_c3d0,
        0x0e5 => 0x010a_c3c0,
        0x0e6 => 0x010a_c3b0,
        0x0e7 => 0x010a_c3a0,
        0x0e8 => 0x010a_c390,
        0x0e9 => 0x010a_c384,
        0x0ea => 0x010a_c378,
        0x0eb => 0x010a_c36c,
        0x0ec => 0x010a_c35c,
        0x0ed => 0x010a_c350,
        0x0ee => 0x010a_c344,
        0x0ef => 0x010a_c338,
        0x0f0 => 0x010a_c328,
        0x0f1 => 0x010a_c31c,
        0x0f2 => 0x010a_c30c,
        0x0f3 => 0x010a_c2fc,
        0x0f4 => 0x010a_c2ec,
        0x0f5 => 0x010a_c2dc,
        0x0f6 => 0x010a_c2cc,
        0x0f7 => 0x010a_c2bc,
        0x0f8 => 0x010a_c2ac,
        0x0f9 => 0x010a_c29c,
        0x0fa => 0x010a_c28c,
        0x0fb => 0x010a_c280,
        0x0fc => 0x010a_c270,
        0x0fd => 0x010a_c260,
        0x0fe => 0x010a_c254,
        0x0ff => 0x010a_c248,
        0x100 => 0x010a_c23c,
        0x101 => 0x010a_c230,
        0x102 => 0x010a_c220,
        0x103 => 0x010a_c210,
        0x104 => 0x010a_c204,
        0x105 => 0x010a_c1f8,
        0x106 => 0x010a_c1ec,
        0x107 => 0x010a_c1dc,
        0x108 => 0x010a_c1d0,
        0x109 => 0x010a_c1c4,
        0x10a => 0x010a_c1b4,
        0x10b => 0x010a_c1a4,
        0x10c => 0x010a_c194,
        0x10d => 0x010a_c184,
        0x10e => 0x010a_c174,
        0x10f => 0x010a_c164,
        0x110 => 0x010a_c154,
        0x111 => 0x010a_c144,
        0x112 => 0x010a_c134,
        0x113 => 0x010a_c124,
        0x114 => 0x010a_c114,
        0x115 => 0x010a_c104,
        0x116 => 0x010a_c0f8,
        0x117 => 0x010a_c0ec,
        0x118 => 0x010a_c0dc,
        0x119 => 0x010a_c0cc,
        0x11a => 0x010a_c0bc,
        0x11b => 0x010a_c0b0,
        0x11c => 0x010a_c0a4,
        0x11d => 0x010a_c094,
        0x11e => 0x010a_c084,
        0x11f => 0x010a_c074,
        0x120 => 0x010a_c064,
        0x121 => 0x010a_c054,
        0x122 => 0x010a_c044,
        0x123 => 0x010a_c034,
        0x124 => 0x010a_c024,
        0x125 => 0x010a_c010,
        0x126 => 0x010a_c000,
        0x127 => 0x010a_bff0,
        0x128 => 0x010a_bfe0,
        0x129 => 0x010a_bfcc,
        0x12a => 0x010a_bfbc,
        0x12b => 0x010a_bfac,
        0x12c => 0x010a_bb78,
        0x12d => 0x010a_bb68,
        0x12e => 0x010a_bb54,
        0x12f => 0x010a_bb40,
        0x130 => 0x010a_bb2c,
        0x131 => 0x010a_bb18,
        0x132 => 0x010a_bb08,
        0x133 => 0x010a_baf4,
        0x134 => 0x010a_bae0,
        0x135 => 0x010a_bacc,
        0x136 => 0x010a_bab8,
        0x137 => 0x010a_baa4,
        0x138 => 0x010a_ba90,
        0x139 => 0x010a_ba78,
        0x13a => 0x010a_ba60,
        0x13b => 0x010a_ba48,
        0x13c => 0x010a_ba30,
        0x13d => 0x010a_ba18,
        0x13e => 0x010a_ba08,
        0x13f => 0x010a_b9f4,
        0x140 => 0x010a_b9e0,
        0x141 => 0x010a_b9cc,
        0x142 => 0x010a_b9b8,
        0x143 => 0x010a_b9a4,
        0x144 => 0x010a_b990,
        0x145 => 0x010a_b978,
        0x146 => 0x010a_b960,
        0x147 => 0x010a_b948,
        0x148 => 0x010a_b930,
        0x149 => 0x010a_b918,
        0x14a => 0x010a_b908,
        0x14b => 0x010a_b8f8,
        0x14c => 0x010a_b8e8,
        0x14d => 0x010a_b8d8,
        0x14e => 0x010a_b8c8,
        0x14f => 0x010a_b8b4,
        0x150 => 0x010a_b8a0,
        0x151 => 0x010a_b890,
        0x152 => 0x010a_b880,
        0x153 => 0x010a_b870,
        0x154 => 0x010a_b85c,
        0x155 => 0x010a_b84c,
        0x156 => 0x010a_b838,
        0x157 => 0x010a_b824,
        0x15a => 0x010a_b810,
        0x15b => 0x010a_b7f8,
        0x15c => 0x010a_b7e0,
        0x15d => 0x010a_b7c8,
        0x15e => 0x010a_b7b0,
        0x15f => 0x010a_b798,
        0x160 => 0x010a_b780,
        0x161 => 0x010a_b764,
        0x162 => 0x010a_b748,
        0x163 => 0x010a_b72c,
        0x164 => 0x010a_b710,
        0x165 => 0x010a_b6f4,
        0x166 => 0x010a_b6e0,
        0x167 => 0x010a_b6c8,
        0x168 => 0x010a_b6b0,
        0x169 => 0x010a_b698,
        0x16a => 0x010a_b680,
        0x16b => 0x010a_b668,
        0x16c => 0x010a_b654,
        0x16d => 0x010a_b63c,
        0x16e => 0x010a_b624,
        0x16f => 0x010a_b60c,
        0x170 => 0x010a_b5f4,
        0x171 => 0x010a_b5dc,
        0x172 => 0x010a_b5c8,
        0x173 => 0x010a_b5b0,
        0x174 => 0x010a_b598,
        0x175 => 0x010a_b580,
        0x176 => 0x010a_b568,
        0x177 => 0x010a_b550,
        0x178 => 0x010a_afd4,
        0x17f => 0x010a_af4c,
        0x180 => 0x010a_af38,
        0x181 => 0x010a_af24,
        0x182 => 0x010a_af10,
        0x183 => 0x010a_aefc,
        0x184 => 0x010a_aee8,
        0x185 => 0x010a_aed4,
        0x186 => 0x010a_aec0,
        0x187 => 0x010a_aeac,
        0x188 => 0x010a_ae98,
        0x189 => 0x010a_ae84,
        0x18a => 0x010a_ae6c,
        0x18b => 0x010a_ae54,
        0x18c => 0x010a_ae40,
        0x18d => 0x010a_ae28,
        0x18e => 0x010a_ae14,
        0x18f => 0x010a_adfc,
        0x190 => 0x010a_ade4,
        0x191 => 0x010a_adcc,
        0x192 => 0x010a_adb4,
        0x193 => 0x010a_ad9c,
        0x194 => 0x010a_ad84,
        0x195 => 0x010a_ad6c,
        0x196 => 0x010a_ad54,
        0x197 => 0x010a_ad3c,
        0x198 => 0x010a_ad24,
        0x199 => 0x010a_ad0c,
        0x19a => 0x010a_acf0,
        0x19b => 0x010a_acd8,
        0x19c => 0x010a_acc0,
        0x19d => 0x010a_acac,
        0x19e => 0x010a_ac94,
        0x19f => 0x010a_ac7c,
        0x1a0 => 0x010a_ac64,
        0x1a1 => 0x010a_ac4c,
        0x1a2 => 0x010a_ac34,
        0x1a3 => 0x010a_ac1c,
        0x1a4 => 0x010a_ac04,
        0x1a5 => 0x010a_abec,
        0x1a6 => 0x010a_abd4,
        0x1a7 => 0x010a_abbc,
        0x1a8 => 0x010a_aba4,
        0x1a9 => 0x010a_ab88,
        0x1aa => 0x010a_ab70,
        0x1ab => 0x010a_ab58,
        0x1ac => 0x010a_ab40,
        0x1ad => 0x010a_ab28,
        0x1ae => 0x010a_ab10,
        0x1af => 0x010a_aaf4,
        0x1b0 => 0x010a_aad8,
        0x1b1 => 0x010a_aabc,
        0x1b2 => 0x010a_aaa0,
        0x1b3 => 0x010a_aa88,
        0x1b4 => 0x010a_aa6c,
        0x1b5 => 0x010a_aa50,
        0x1b6 => 0x010a_aa34,
        0x1b7 => 0x010a_aa18,
        0x1b8 => 0x010a_a9fc,
        0x1b9 => 0x010a_a9e0,
        0x1ba => 0x010a_a9c4,
        0x1bb => 0x010a_a74c,
        0x1bc => 0x010a_a734,
        0x1bd => 0x010a_a9b4,
        0x1be => 0x010a_a9a4,
        0x1bf => 0x010a_a990,
        0x1c0 => 0x010a_a97c,
        0x1c1 => 0x010a_a96c,
        0x1c2 => 0x010a_a958,
        0x1c3 => 0x010a_a944,
        0x1c4 => 0x010a_a924,
        0x1c5 => 0x010a_a904,
        0x1c6 => 0x010a_a8e4,
        0x1c7 => 0x010a_a8c0,
        0x1c8 => 0x010a_a8b0,
        0x1c9 => 0x010a_a89c,
        0x1ca => 0x010a_a888,
        0x1cb => 0x010a_a874,
        0x1cc => 0x010a_a804,
        0x1cd => 0x010a_a7ec,
        0x1ce => 0x010a_a7d4,
        0x1cf => 0x010a_a7b8,
        0x1d0 => 0x010a_a79c,
        0x1d1 => 0x010a_a780,
        0x1d2 => 0x010a_a760,
        0x1d3 => 0x010a_a860,
        0x1d4 => 0x010a_a848,
        0x1d5 => 0x010a_a834,
        0x1d6 => 0x010a_a81c,
        0x1d7 => 0x010a_a67c,
        0x1d8 => 0x010a_a66c,
        0x1d9 => 0x010a_a65c,
        0x1da => 0x010a_a648,
        0x1db => 0x010a_a6a0,
        0x1dc => 0x010a_a68c,
        0x1dd => 0x010a_a724,
        0x1de => 0x010a_a710,
        0x1df => 0x010a_a6f8,
        0x1e0 => 0x010a_a6e4,
        0x1e1 => 0x010a_a6d0,
        0x1e2 => 0x010a_9568,
        0x1e3 => 0x010a_9550,
        0x1e4 => 0x010a_9538,
        0x1e5 => 0x010a_951c,
        0x1e6 => 0x010a_9500,
        0x1e7 => 0x010a_d5d4,
        0x1e8 => 0x010a_d5c4,
        0x1e9 => 0x010a_d5b4,
        0x1ea => 0x010a_a6c0,
        0x1eb => 0x010a_a6b0,
        0x1ec => 0x010a_94f0,
        0x1ed => 0x010a_94e0,
        0x1ee => 0x010a_94cc,
        0x1ef => 0x010a_b530,
        0x1f0 => 0x010a_b520,
        0x1f1 => 0x010a_b514,
        0x1f2 => 0x010a_b504,
        0x1f3 => 0x010a_b4f0,
        0x1f4 => 0x010a_b4e0,
        0x1f5 => 0x010a_b4d0,
        0x1f6 => 0x010a_b4bc,
        0x1f7 => 0x010a_b4b0,
        0x1f8 => 0x010a_b4a0,
        0x1f9 => 0x010a_b48c,
        0x1fa => 0x010a_b474,
        0x1fb => 0x010a_b464,
        0x1fc => 0x010a_b450,
        0x1fd => 0x010a_b43c,
        0x1fe => 0x010a_b424,
        0x1ff => 0x010a_b418,
        0x200 => 0x010a_b408,
        0x201 => 0x010a_b3f4,
        0x202 => 0x010a_b3dc,
        0x203 => 0x010a_b3cc,
        0x204 => 0x010a_b3b8,
        0x205 => 0x010a_b3a4,
        0x206 => 0x010a_b38c,
        0x207 => 0x010a_b380,
        0x208 => 0x010a_b370,
        0x209 => 0x010a_b35c,
        0x20a => 0x010a_b344,
        0x20b => 0x010a_b334,
        0x20c => 0x010a_b320,
        0x20d => 0x010a_b30c,
        0x20e => 0x010a_b2f4,
        0x20f => 0x010a_b2e8,
        0x210 => 0x010a_b2d8,
        0x211 => 0x010a_b2c4,
        0x212 => 0x010a_b2ac,
        0x213 => 0x010a_b29c,
        0x214 => 0x010a_b288,
        0x215 => 0x010a_b274,
        0x216 => 0x010a_b25c,
        0x217 => 0x010a_b250,
        0x218 => 0x010a_b240,
        0x219 => 0x010a_b22c,
        0x21a => 0x010a_b214,
        0x21b => 0x010a_b204,
        0x21c => 0x010a_b1f0,
        0x21d => 0x010a_b1dc,
        0x21e => 0x010a_b1c4,
        0x21f => 0x010a_b1b8,
        0x220 => 0x010a_b1a8,
        0x221 => 0x010a_b194,
        0x222 => 0x010a_b17c,
        0x223 => 0x010a_b16c,
        0x224 => 0x010a_b158,
        0x225 => 0x010a_b144,
        0x226 => 0x010a_b12c,
        0x227 => 0x010a_b120,
        0x228 => 0x010a_b110,
        0x229 => 0x010a_b0fc,
        0x22a => 0x010a_b0e4,
        0x22b => 0x010a_b0d4,
        0x22c => 0x010a_b0c0,
        0x22d => 0x010a_b0ac,
        0x22e => 0x010a_b094,
        0x22f => 0x010a_b084,
        0x230 => 0x010a_b074,
        0x231 => 0x010a_b05c,
        0x232 => 0x010a_b044,
        0x233 => 0x010a_b028,
        0x234 => 0x010a_b010,
        0x235 => 0x010a_b000,
        0x236 => 0x010a_afe8,
        0x238 => 0x010a_a638,
        0x239 => 0x010a_a62c,
        0x23a => 0x010a_a5fc,
        0x23b => 0x010a_a5d8,
        0x23c => 0x010a_a618,
        0x23d => 0x010a_a5ac,
        0x23e => 0x010a_a5bc,
        0x23f => 0x010a_9a14,
        0x240 => 0x010a_99f4,
        0x241 => 0x010a_99cc,
        0x242 => 0x010a_99c0,
        0x243 => 0x010a_99b4,
        0x244 => 0x010a_99a8,
        0x245 => 0x010a_9998,
        0x246 => 0x010a_9988,
        0x247 => 0x010a_9978,
        0x248 => 0x010a_9968,
        0x249 => 0x010a_9954,
        0x24a => 0x010a_9940,
        0x24b => 0x010a_992c,
        0x24c => 0x010a_98d0,
        0x24d => 0x010a_98bc,
        0x24e => 0x010a_991c,
        0x24f => 0x010a_9908,
        0x250 => 0x010a_98f4,
        0x251 => 0x010a_98e0,
        0x252 => 0x010a_98b0,
        0x253 => 0x010a_98a4,
        0x254 => 0x010a_9898,
        0x255 => 0x010a_988c,
        0x256 => 0x010a_9880,
        0x25c => 0x010a_d59c,
        0x25d => 0x010a_d584,
        0x25e => 0x010a_d56c,
        0x25f => 0x010a_d554,
        0x260 => 0x010a_d53c,
        0x261 => 0x010a_d524,
        0x262 => 0x010a_d50c,
        0x263 => 0x010a_d4f4,
        0x264 => 0x010a_d4d8,
        0x265 => 0x010a_d4bc,
        0x266 => 0x010a_d4a0,
        0x267 => 0x010a_d484,
        0x268 => 0x010a_d468,
        0x269 => 0x010a_d44c,
        0x26a => 0x010a_d430,
        0x26b => 0x010a_d414,
        0x26c => 0x010a_d3fc,
        0x26d => 0x010a_d3e4,
        0x26e => 0x010a_d3cc,
        0x26f => 0x010a_d3b0,
        0x270 => 0x010a_d398,
        0x271 => 0x010a_d380,
        0x272 => 0x010a_d368,
        0x273 => 0x010a_d350,
        0x274 => 0x010a_d338,
        0x275 => 0x010a_d320,
        0x276 => 0x010a_d308,
        0x277 => 0x010a_d2f0,
        0x278 => 0x010a_d2d8,
        0x279 => 0x010a_d2c0,
        0x27a => 0x010a_d2a4,
        0x27b => 0x010a_d288,
        0x27c => 0x010a_d26c,
        0x27d => 0x010a_d250,
        0x27e => 0x010a_d234,
        0x27f => 0x010a_d218,
        0x280 => 0x010a_d200,
        0x281 => 0x010a_d1e8,
        0x282 => 0x010a_d1d0,
        0x283 => 0x010a_d1b8,
        0x284 => 0x010a_d1a4,
        0x285 => 0x010a_d18c,
        0x286 => 0x010a_d174,
        0x287 => 0x010a_d15c,
        0x288 => 0x010a_a024,
        0x289 => 0x010a_9a44,
        0x28a => 0x010a_a548,
        0x28b => 0x010a_a518,
        0x28c => 0x010a_a4f4,
        0x28d => 0x010a_a4d0,
        0x28e => 0x010a_a4b4,
        0x28f => 0x010a_a478,
        0x290 => 0x010a_a44c,
        0x291 => 0x010a_a420,
        0x292 => 0x010a_a404,
        0x293 => 0x010a_a3c8,
        0x294 => 0x010a_a39c,
        0x295 => 0x010a_a370,
        0x296 => 0x010a_a34c,
        0x297 => 0x010a_a308,
        0x298 => 0x010a_a2d0,
        0x299 => 0x010a_a29c,
        0x29a => 0x010a_a278,
        0x29b => 0x010a_9fd4,
        0x29c => 0x010a_9f58,
        0x29d => 0x010a_9f7c,
        0x29e => 0x010a_9fa0,
        0x29f => 0x010a_9f38,
        0x2a0 => 0x010a_9f08,
        0x2a1 => 0x010a_9ed8,
        0x2a2 => 0x010a_9e9c,
        0x2a3 => 0x010a_9e7c,
        0x2a4 => 0x010a_9de0,
        0x2a5 => 0x010a_9e10,
        0x2a6 => 0x010a_9e40,
        0x2a7 => 0x010a_9db8,
        0x2a8 => 0x010a_9d80,
        0x2a9 => 0x010a_9d48,
        0x2aa => 0x010a_9d00,
        0x2ab => 0x010a_9cd4,
        0x2ac => 0x010a_a26c,
        0x2ad => 0x010a_a240,
        0x2ae => 0x010a_a224,
        0x2af => 0x010a_a208,
        0x2b0 => 0x010a_a1f4,
        0x2b1 => 0x010a_a1c0,
        0x2b2 => 0x010a_a19c,
        0x2b3 => 0x010a_a178,
        0x2b4 => 0x010a_a164,
        0x2b5 => 0x010a_a140,
        0x2b6 => 0x010a_a11c,
        0x2b7 => 0x010a_a0e8,
        0x2b8 => 0x010a_a0cc,
        0x2b9 => 0x010a_a090,
        0x2ba => 0x010a_a064,
        0x2bb => 0x010a_a038,
        0x2bc => 0x010a_a004,
        0x2bd => 0x010a_9fec,
        0x2be => 0x010a_9cc4,
        0x2bf => 0x010a_9c98,
        0x2c0 => 0x010a_9c78,
        0x2c1 => 0x010a_9c58,
        0x2c2 => 0x010a_9c40,
        0x2c3 => 0x010a_9c08,
        0x2c4 => 0x010a_9be0,
        0x2c5 => 0x010a_9bb8,
        0x2c6 => 0x010a_9ba0,
        0x2c7 => 0x010a_9b78,
        0x2c8 => 0x010a_9b50,
        0x2c9 => 0x010a_9b1c,
        0x2ca => 0x010a_9afc,
        0x2cb => 0x010a_9abc,
        0x2cc => 0x010a_9a8c,
        0x2cd => 0x010a_9a5c,
        0x2ce => 0x010a_9a24,
        0x2cf => 0x010a_99dc,
        0x2d0 => 0x010a_963c,
        0x2d1 => 0x010a_9628,
        0x2d2 => 0x010a_a5e8,
        0x2d3 => 0x010a_a594,
        0x2d4 => 0x010a_a55c,
        0x2d5 => 0x010a_a578,
        0x2d6 => 0x010a_9614,
        0x2d7 => 0x010a_95fc,
        0x2d8 => 0x010a_986c,
        0x2d9 => 0x010a_9858,
        0x2da => 0x010a_9844,
        0x2db => 0x010a_982c,
        0x2dc => 0x010a_9818,
        0x2dd => 0x010a_9800,
        0x2de => 0x010a_97ec,
        0x2df => 0x010a_97d4,
        0x2e0 => 0x010a_97bc,
        0x2e1 => 0x010a_97a4,
        0x2e2 => 0x010a_978c,
        0x2e3 => 0x010a_9774,
        0x2e4 => 0x010a_9760,
        0x2e5 => 0x010a_9748,
        0x2e6 => 0x010a_9730,
        0x2e7 => 0x010a_9718,
        0x2e8 => 0x010a_9700,
        0x2e9 => 0x010a_96e8,
        0x2ea => 0x010a_96d0,
        0x2eb => 0x010a_96b8,
        0x2ec => 0x010a_96a0,
        0x2ed => 0x010a_9684,
        0x2ee => 0x010a_966c,
        0x2ef => 0x010a_9650,
        0x2f0 => 0x010a_949c,
        0x2f1 => 0x010a_94b8,
        0x2f2 => 0x010a_95ac,
        0x2f3 => 0x010a_9594,
        0x2f4 => 0x010a_957c,
        0x2f5 => 0x010a_9490,
        _ => 0,
    };
    if name != 0 {
        return name;
    }
    e.with_stack(0xc, |e, buffer| {
        e.call(
            SPRINTF_S_00406D00,
            &args![buffer, 0xau32, PASS_NAME_FORMAT_010A9488, pass],
        );
    });
    PASS_NAME_UNKNOWN_010A93F4
}

// Translated from 00b540d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, no arguments: lowers the words at `011ffa10` and `011ffa0c`
/// by one and calls `00b984f0(0, 0)`.
pub fn fn_00b540d0(e: &mut Engine) {
    let v = e.global::<u32>(COUNTER_011FFA10);
    e.set_global(COUNTER_011FFA10, v.wrapping_sub(1));
    let v = e.global::<u32>(COUNTER_011FFA0C);
    e.set_global(COUNTER_011FFA0C, v.wrapping_sub(1));
    e.call(FUNCTION_00B984F0, &args![0u32, 0u32]);
}

// Translated from 00b553d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::SetEmittanceRecurse` (engine map; the body matches): `cdecl`, an object
/// and the address of three words (a colour). For a non-null object: a geometry (slot +0x18
/// non-zero) with both a material property (`GetProperty(2)`) and a shader property
/// (`GetProperty(3)`) whose first flag word has bit 29 (`0x20000000`) set gets the three
/// words at material `+0x28`, `+0x2c` and `+0x30` and its revision word at `+0x44` raised by
/// one; any other object that is a node (slot +0xc) passes the call on to each non-null
/// child (the array at node `+0xa0`, the count at `+0xa6` read once).
pub fn bs_shader_manager_set_emittance_recurse(e: &mut Engine, object: u32, color: u32) {
    if object == 0 {
        return;
    }
    if e.vcall(object, SLOT_IS_GEOMETRY, &args![]).u32() != 0 {
        let material = e
            .call(GET_PROPERTY, &args![object, MATERIAL_PROPERTY_TYPE])
            .u32();
        let shader = e
            .call(GET_PROPERTY, &args![object, SHADER_PROPERTY_TYPE])
            .u32();
        if material == 0 || shader == 0 {
            return;
        }
        if e.mem.u32(shader.wrapping_add(0x20)) & 0x2000_0000 == 0 {
            return;
        }
        let v = e.mem.u32(color);
        e.mem.set_u32(material.wrapping_add(0x28), v);
        let v = e.mem.u32(color.wrapping_add(4));
        e.mem.set_u32(material.wrapping_add(0x2c), v);
        let v = e.mem.u32(color.wrapping_add(8));
        let revision = e.mem.u32(material.wrapping_add(0x44));
        e.mem
            .set_u32(material.wrapping_add(0x44), revision.wrapping_add(1));
        e.mem.set_u32(material.wrapping_add(0x30), v);
        return;
    }
    let node = e.vcall(object, SLOT_IS_NODE, &args![]).u32();
    if node == 0 {
        return;
    }
    let count = u32::from(e.mem.u16(node.wrapping_add(0xa6)));
    let mut index = 0u32;
    while index < count {
        let buffer = e.mem.u32(node.wrapping_add(0xa0));
        let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
        if child != 0 {
            bs_shader_manager_set_emittance_recurse(e, child, color);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00b557d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, no arguments: for each non-null shader in the shader table
/// whose run-time type chain (slot +8, then each record's base at `+4`) reaches the record
/// at `0126f554`, calls the shader's slot +0x120; then the `ImageSpaceManager` method
/// `00b8b3b0` on the object at `011f91ac` (a tail jump).
pub fn fn_00b557d0(e: &mut Engine) {
    let mut slot = SHADER_TABLE_011F9548;
    while slot < SHADER_TABLE_END_011F95D4 {
        let shader = e.mem.u32(slot);
        if shader != 0 {
            let mut rtti = e.vcall(shader, 8, &args![]).u32();
            let mut found = false;
            while rtti != 0 {
                if rtti == RTTI_0126F554 {
                    found = true;
                    break;
                }
                rtti = e.mem.u32(rtti.wrapping_add(4));
            }
            if found {
                e.vcall(shader, 0x120, &args![]);
            }
        }
        slot = slot.wrapping_add(4);
    }
    let manager = e.global::<u32>(OBJECT_011F91AC);
    e.call(IMAGE_SPACE_MANAGER_00B8B3B0, &args![manager]);
}

// Translated from 00b55b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::RecursiveSetForcedDarkness` (engine map; the body matches): `cdecl`,
/// an object and a `float`. A shader property (`GetProperty(3)`) whose
/// `iShaderPropertyType` (`+0x1c`) is 1 to 12 gets the `float` at `+0x6c` (`FLD`/`FSTP`, a
/// signalling NaN arrives quiet); then, when the object is a node (slot +0xc), each non-null
/// child (the count at `+0xa6` read again after each one) gets the call with the `float`
/// (pushed through `FLD`/`FSTP`).
pub fn bs_shader_manager_recursive_set_forced_darkness(e: &mut Engine, object: u32, darkness: u32) {
    let property = e
        .call(GET_PROPERTY, &args![object, SHADER_PROPERTY_TYPE])
        .u32();
    if property != 0 {
        let kind = e.mem.u32(property.wrapping_add(0x1c));
        if kind.wrapping_sub(1) <= 0xb {
            e.mem
                .set_u32(property.wrapping_add(0x6c), x87_quiet_bits(darkness));
        }
    }
    let node = e.vcall(object, SLOT_IS_NODE, &args![]).u32();
    if node == 0 {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(object.wrapping_add(0xa6))) {
        let buffer = e.mem.u32(object.wrapping_add(0xa0));
        let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
        if child != 0 {
            bs_shader_manager_recursive_set_forced_darkness(e, child, x87_quiet_bits(darkness));
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00b578a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b578a0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, no arguments (after a device reset). Before the set-up has
/// run (the byte at `011f91c6` clear) it runs [`fn_00b57420`]. Otherwise: [`fn_00b4f9a0`],
/// `00ba50d0`, `00bc0940` and `00bc4610`; the object at `011f91ac` destroyed (`00b97690`),
/// freed and cleared, the one at `011f91a8` likewise (`00b6f0b0`); the word at `011f91e0`
/// cleared; a new 0xb0-byte object (`00b6dd80`, null when the allocation fails) at
/// `011f91a8`; with a word at `011f9508`, a new 0x31c-byte object (`00b97a80`) at
/// `011f91ac`; and `00b6e450` on the object at `011f91a8` with the word at `011f9508`.
pub fn fn_00b578a0(e: &mut Engine) {
    if e.global::<u8>(SET_UP_DONE_011F91C6) == 0 {
        fn_00b57420(e);
        return;
    }
    fn_00b4f9a0(e);
    e.call(RESET_00BA50D0, &args![]);
    e.call(RESET_00BC0940, &args![]);
    e.call(RESET_00BC4610, &args![]);
    let object = e.global::<u32>(OBJECT_011F91AC);
    if object != 0 {
        e.call(DESTRUCTOR_00B97690, &args![object]);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, object]);
        e.set_global(OBJECT_011F91AC, 0u32);
    }
    let object = e.global::<u32>(OBJECT_011F91A8);
    if object != 0 {
        e.call(DESTRUCTOR_00B6F0B0, &args![object]);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, object]);
        e.set_global(OBJECT_011F91A8, 0u32);
    }
    e.set_global(GLOBAL_011F91E0, 0u32);
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xb0u32])
        .u32();
    let object = if block != 0 {
        e.call(CONSTRUCTOR_00B6DD80, &args![block]).u32()
    } else {
        0
    };
    e.set_global(OBJECT_011F91A8, object);
    if e.global::<u32>(GLOBAL_011F9508) != 0 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x31cu32])
            .u32();
        let object = if block != 0 {
            e.call(CONSTRUCTOR_00B97A80, &args![block]).u32()
        } else {
            0
        };
        e.set_global(OBJECT_011F91AC, object);
    }
    let source = e.global::<u32>(GLOBAL_011F9508);
    let object = e.global::<u32>(OBJECT_011F91A8);
    e.call(METHOD_00B6E450, &args![object, source]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b4_f4a0,
            bs_shader_manager_get_shader_level_string() -> u32
        ),
        entry!(0x00b4_f710, fn_00b4f710(u32, u32, u8, Ptr, Ptr, i32)),
        entry!(
            0x00b5_5840,
            bs_shader_manager_get_texture(Ptr, bool, Ptr, bool, bool)
        ),
        entry!(0x00b5_5bb0, fn_00b55bb0()),
        entry!(0x00b5_5560, bs_shader_manager_get_shader(u32) -> Ptr),
        entry!(0x00b4_f1a0, bs_shader_manager_register_streamables()),
        entry!(0x00b5_4110, shadow_scene_node_set_fog_property(Ptr, Ptr)),
        entry!(0x00b5_4000, fn_00b54000(f32)),
        entry!(0x0070_b760, fn_0070b760(Ptr, u8)),
        entry!(0x00b4_f2f0, fn_00b4f2f0(u32, Ptr)),
        entry!(0x00b4_f5c0, bs_shader_manager_get_accumulator() -> Ptr),
        entry!(0x00b5_5540, bs_shader_manager_set_fog_property(u32, Ptr)),
        entry!(0x00b5_7420, fn_00b57420()),
        entry!(0x00b4_f380, fn_00b4f380(u8) -> u32),
        entry!(0x00b4_f3e0, fn_00b4f3e0() -> u32),
        entry!(0x00b4_f300, fn_00b4f300(u32)),
        entry!(0x00b4_f430, fn_00b4f430(u32)),
        entry!(0x00b4_f450, fn_00b4f450(u32)),
        entry!(0x00b4_f5d0, fn_00b4f5d0() -> u32),
        entry!(
            0x00b4_f5e0,
            bs_shader_manager_get_modified_texture_filename(u32, u32, u32, u8)
        ),
        entry!(0x00b5_4090, fn_00b54090() -> u32),
        entry!(0x00b5_40b0, fn_00b540b0() -> u32),
        entry!(0x00b5_40f0, fn_00b540f0() -> u32),
        entry!(0x00b5_4280, fn_00b54280(Ptr) -> Ptr),
        entry!(0x00b5_4620, fn_00b54620(Ptr)),
        entry!(0x00b5_4630, fn_00b54630(u8)),
        entry!(0x00b5_4ac0, bs_shader_manager_set_accumulator(u32)),
        entry!(0x00b5_4b10, fn_00b54b10(u32)),
        entry!(0x00b5_4b60, bs_shader_manager_create_tangent_space_simple(u32) -> u32),
        entry!(
            0x00b5_5360,
            bs_shader_manager_set_object_type_recurse(u32, u32)
        ),
        entry!(0x00b5_5480, fn_00b55480(u32, u32)),
        entry!(0x00b5_5520, bs_shader_manager_get_fog_property(u32) -> u32),
        entry!(0x00b5_5a10, fn_00b55a10()),
        entry!(0x00b5_5ac0, fn_00b55ac0(u32, u32, u32)),
        entry!(0x00b5_5b80, bs_shader_manager_set_object_type(u32)),
        entry!(
            0x00b5_79e0,
            bs_shader_manager_prepare_geometry(u32, u32, u8, u8)
        ),
        entry!(
            0x00b5_7bd0,
            bs_shader_manager_prepare_object_recurse(u32, u32, u8, u8)
        ),
        entry!(
            0x00b5_7c60,
            bs_shader_manager_clone_material_property_recurse(u32)
        ),
        entry!(0x00b5_7e30, bs_shader_manager_prepare_object(u32, u8, u8)),
        entry!(0x00b4_f2e0, fn_00b4f2e0(u32, u32, u32) -> u32),
        entry!(0x00b4_f9a0, fn_00b4f9a0()),
        entry!(0x00b4_f9d0, bs_shader_manager_get_pass_name(u32) -> u32),
        entry!(0x00b5_40d0, fn_00b540d0()),
        entry!(
            0x00b5_53d0,
            bs_shader_manager_set_emittance_recurse(u32, u32)
        ),
        entry!(0x00b5_57d0, fn_00b557d0()),
        entry!(
            0x00b5_5b00,
            bs_shader_manager_recursive_set_forced_darkness(u32, u32)
        ),
        entry!(0x00b5_78a0, fn_00b578a0()),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(FIRST_FACTOR, 8);
        e.map(SECOND_FACTOR, 8);
        e.map(REFERENCE_ANGLE, 8);
        e.map(RATIO_RESULT, 4);
        e.set_global(FIRST_FACTOR, 0.5f64);
        e.set_global(SECOND_FACTOR, 2.0f64);
        e.set_global(REFERENCE_ANGLE, 3.0f64);
        // sin(x) = x + 1, cos(x) = 2: the quotients are exact in binary.
        e.register(CRT_SIN, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: x + 1.0,
                ..Ret::default()
            }
        });
        e.register(CRT_COS, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e
    }

    #[test]
    fn at_00b54000_divides_tan_of_the_scaled_angle_by_tan_of_the_reference() {
        let mut e = rig();
        // 0.5 * 0.5 * 2.0 = 0.5; (0.5 + 1) / 2 = 0.75; (3 + 1) / 2 = 2; 0.75 / 2 = 0.375.
        e.call(0x00b5_4000, &args![0.5f32]);
        assert_eq!(e.global::<f32>(RATIO_RESULT), 0.375);
        let log = e.call_log.take().unwrap();
        let sines: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == CRT_SIN)
            .map(|c| &c.1)
            .collect();
        let half = 0.5f64.to_bits();
        let three = 3.0f64.to_bits();
        assert_eq!(
            sines,
            vec![
                &vec![half as u32, (half >> 32) as u32],
                &vec![three as u32, (three >> 32) as u32]
            ]
        );
    }

    #[test]
    fn at_00b54000_rounds_each_stage_to_a_float() {
        let mut e = rig();
        // The scaled angle (0.2 * 0.5 * 1.0) is a float before sin sees it.
        e.set_global(SECOND_FACTOR, 1.0f64);
        e.call(0x00b5_4000, &args![0.2f32]);
        let scaled = f64::from(0.2f32 / 2.0);
        let sine = f64::from((scaled + 1.0) as f32);
        let quotient = f64::from((sine / 2.0) as f32);
        let expected = (quotient / 2.0) as f32;
        assert_eq!(e.global::<f32>(RATIO_RESULT), expected);
    }

    #[test]
    fn at_00b54000_gives_a_nan_for_a_nan_argument() {
        let mut e = rig();
        e.call(0x00b5_4000, &args![f32::from_bits(0x7f80_0001)]);
        assert!(e.global::<f32>(RATIO_RESULT).is_nan());
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

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
    fn fn_0070b760_stores_the_index_and_fills_the_table() {
        let mut e = Engine::new();
        e.map(SHADOW_SCENE_NODES, 0x40);
        let node = e.mem.alloc(0x130);
        e.call(0x0070_b760, &args![node, 0xffff_ff03u32]);
        assert_eq!(e.mem.u8(node + 0x120), 3);
        assert_eq!(e.mem.u32(SHADOW_SCENE_NODES + 12), node);
    }

    #[test]
    fn fn_00b4f2f0_stores_the_node_at_its_index() {
        let mut e = Engine::new();
        e.map(SHADOW_SCENE_NODES, 0x40);
        e.call(0x00b4_f2f0, &args![2u32, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(SHADOW_SCENE_NODES + 8), 0x1234_5678);
        assert_eq!(e.mem.u32(SHADOW_SCENE_NODES + 4), 0);
    }

    #[test]
    fn get_accumulator_reads_its_global() {
        let mut e = Engine::new();
        e.map(ACCUMULATOR, 4);
        e.set_global(ACCUMULATOR, 0x0bad_f00du32);
        assert_eq!(e.call(0x00b4_f5c0, &args![]).u32(), 0x0bad_f00d);
    }

    #[test]
    fn set_fog_property_calls_the_node_of_the_index() {
        let mut e = Engine::new();
        e.map(SHADOW_SCENE_NODES, 0x40);
        e.mem.set_u32(SHADOW_SCENE_NODES + 4, 0x5000);
        e.register(SHADOW_SCENE_NODE_SET_FOG_PROPERTY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_5540, &args![1u32, 0x6000u32]);
        assert_eq!(
            calls(&e, SHADOW_SCENE_NODE_SET_FOG_PROPERTY),
            vec![vec![0x5000, 0x6000]]
        );
    }

    /// Doubles for every callee of `00b57420`; constructors answer `this`. The vector
    /// constructor fills the slots with `old` (0 for none).
    fn set_up_rig(old: u32) -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_6000,
            0x010a_d000,
            0x011a_9000,
            0x011f_9000,
            0x011f_a000,
            0x011f_f000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(HALF_01016248, 0.5f32);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1].max(8)).into_ret());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        for this_returning in [0x00b6_dd80u32, 0x00b9_7a80, 0x00b6_60d0] {
            e.register(this_returning, |_, a| a[0].into_ret());
        }
        // `00b7ff40` keeps `this`, with its word at +4 as 1 (kept by the first check).
        e.register(0x00b7_ff40, |e, a| {
            e.mem.set_u32(a[0] + 4, 1);
            a[0].into_ret()
        });
        e.register(0x00b7_faf0, |_, _| 7u32.into_ret());
        e.register(0x00a5_b690, |_, _| 0x0777_0000u32.into_ret());
        for plain in [
            0x00b7_fe40u32,
            0x00b7_fd50,
            0x00b5_5bb0,
            0x00b5_5560,
            0x00b4_f1a0,
            0x00b6_e450,
            0x00ba_3720,
            0x00b9_7c10,
        ] {
            e.register(plain, |_, _| Ret::default());
        }
        e.register_double(0x00ec_782f, move |e, a| {
            for i in 0..a[2] {
                e.mem.set_u32(a[0] + 4 * i, old);
            }
            Ret::default()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e
    }

    #[test]
    fn fn_00b57420_first_run_builds_the_manager_and_the_accumulators() {
        let mut e = set_up_rig(0);
        e.set_global(GLOBAL_011F9508, 1u32);
        e.set_global(ACCUMULATOR_COUNT_011F916C, 2u32);
        e.set_global(GLOBAL_011F94F0, 0x4444u32);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        e.register_double(0x00b5_5bb0, move |e, _| {
            // The context is 5 while the set-up runs.
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 5);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b5_7420, &args![]);

        let shaders: Vec<u32> = calls(&e, 0x00b5_5560).iter().map(|w| w[0]).collect();
        assert_eq!(shaders, (1..0x23).collect::<Vec<u32>>());
        // Kept by the first check, released after the shaders.
        assert_eq!(calls(&e, 0x00b7_fd50).len(), 1);
        assert_eq!(e.global::<u32>(OBJECT_011F9498), 0);
        assert_ne!(e.global::<u32>(OBJECT_011F91A8), 0);
        assert_ne!(e.global::<u32>(OBJECT_011F91AC), 0);
        let first = e.global::<u32>(OBJECT_011F91A8);
        assert_eq!(calls(&e, 0x00b6_e450), vec![vec![first, 1]]);
        assert_eq!(calls(&e, 0x00a5_b690), vec![vec![HOLDER_NAME_010ADA3C]]);
        let holder = e.global::<u32>(HOLDER_011F94BC);
        assert_eq!(e.mem.u32(holder), 0x0777_0000);
        assert_eq!(e.global::<u8>(SET_UP_DONE_011F91C6), 1);
        for (i, want) in [0.5f32, 0.5, 0.5, 1.0].iter().enumerate() {
            assert_eq!(e.global::<f32>(FLOATS_011FA090 + 4 * i as u32), *want);
        }
        for (word, code) in FUNCTION_WORDS_00B57420 {
            assert_eq!(e.global::<u32>(word), code);
        }
        assert_eq!(e.global::<u32>(0x011f_9f44), 0);
        assert_eq!(e.global::<u32>(GLOBAL_011A9608), 0x4444);

        // Two accumulators, each held once, after the count word.
        let array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
        assert_eq!(e.mem.u32(array - 4), 2);
        let sizes: Vec<u32> = calls(&e, MEMORY_ALLOCATE).iter().map(|w| w[1]).collect();
        assert_eq!(sizes, vec![0xb0, 0x1c, 0x31c, 4, 12]);
        let vector = calls(&e, 0x00ec_782f);
        assert_eq!(vector, vec![vec![array, 4, 2, 0x00b5_4620, 0x0045_cec0]]);
        let accumulators = calls(&e, 0x00b6_60d0);
        assert_eq!(accumulators.len(), 2);
        assert_eq!(&accumulators[0][1..], &[0x63, 1, 0x2f7]);
        for (i, call) in accumulators.iter().enumerate() {
            let accumulator = e.mem.u32(array + 4 * i as u32);
            assert_eq!(accumulator, call[0]);
            assert_eq!(e.mem.u32(accumulator + 4), 1);
        }
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
    }

    #[test]
    fn fn_00b57420_first_run_without_shaders_releases_an_empty_object_at_once() {
        let mut e = set_up_rig(0);
        e.register(0x00b7_ff40, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00b5_7420, &args![]);
        assert!(calls(&e, 0x00b5_5560).is_empty());
        assert!(calls(&e, 0x00b9_7a80).is_empty());
        assert_eq!(calls(&e, 0x00b7_fd50).len(), 1);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), 1);
        assert_eq!(e.global::<u32>(OBJECT_011F9498), 0);
        // No accumulators: the array has only its count word.
        assert!(calls(&e, NEW_OBJECT).is_empty());
        let array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
        assert_eq!(e.mem.u32(array - 4), 0);
    }

    #[test]
    fn fn_00b57420_later_runs_skip_the_set_up_and_release_old_slots() {
        let mut e = set_up_rig(0);
        let old = e.mem.alloc(8);
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_2000]);
        e.mem.set_u32(old, 0x0f00_1000);
        e.mem.set_u32(old + 4, 1);
        e.register(0x0f00_2000, |_, _| Ret::default());
        e.register_double(0x00ec_782f, move |e, a| {
            for i in 0..a[2] {
                e.mem.set_u32(a[0] + 4 * i, old);
            }
            Ret::default()
        });
        e.set_global(SET_UP_DONE_011F91C6, 1u8);
        e.set_global(ACCUMULATOR_COUNT_011F916C, 1u32);
        e.call_log = Some(vec![]);
        e.call(0x00b5_7420, &args![]);
        assert!(calls(&e, 0x00b6_dd80).is_empty());
        assert!(calls(&e, 0x00b4_f1a0).is_empty());
        // The old pointer's count went to 0: deleted through its vtable +4.
        assert_eq!(calls(&e, 0x0f00_2000), vec![vec![old]]);
        let array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
        assert_ne!(e.mem.u32(array), old);
        assert_eq!(calls(&e, 0x00ba_3720).len(), 1);
        assert_eq!(e.global::<u32>(GLOBAL_011F9168), 0);
    }
}

#[cfg(test)]
mod b000c_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    /// The renderer word set, each creator answering `0x5000 + index`, and the assignment
    /// storing into the slot (as the `NiPointer` assignment does).
    fn shader_rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(GLOBAL_011F9508, 1u32);
        for index in 0..=LAST_SHADER_INDEX {
            if let Some(create) = shader_creator(index) {
                e.register_double(create, move |_, _| (0x5000 + index).into_ret());
            }
        }
        e.register(SHADER_4_CREATE, |_, _| 0x5004u32.into_ret());
        e.register(SHADER_1D_CREATE, |_, _| 0x501du32.into_ret());
        e.register(NI_POINTER_ASSIGN_00A5F860, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e
    }

    #[test]
    fn get_shader_makes_a_missing_shader_once_with_the_context_at_5() {
        let mut e = shader_rig();
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        e.register_double(0x00b7_a380, move |e, _| {
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 5);
            0x5001u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_5560, &args![1u32]).u32(), 0x5001);
        assert_eq!(e.call(0x00b5_5560, &args![1u32]).u32(), 0x5001);
        assert_eq!(calls(&e, 0x00b7_a380).len(), 1);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN_00A5F860),
            vec![vec![SHADER_TABLE_011F9548 + 4, 0x5001]]
        );
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
    }

    #[test]
    fn get_shader_calls_each_index_creator() {
        let mut e = shader_rig();
        for index in [
            2u32, 3, 5, 6, 7, 8, 9, 0xa, 0xd, 0xe, 0xf, 0x10, 0x11, 0x18, 0x19, 0x1a, 0x1e, 0x20,
            0x21, 0x22,
        ] {
            assert_eq!(e.call(0x00b5_5560, &args![index]).u32(), 0x5000 + index);
        }
        assert_eq!(shader_creator(7), Some(0x00b8_0e70));
        assert_eq!(shader_creator(0x22), Some(0x00bc_59b0));
    }

    #[test]
    fn get_shader_4_makes_shader_1_first() {
        let mut e = shader_rig();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_5560, &args![4u32]).u32(), 0x5004);
        assert_eq!(e.mem.u32(SHADER_TABLE_011F9548 + 4), 0x5001);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN_00A5F860),
            vec![
                vec![SHADER_TABLE_011F9548 + 4, 0x5001],
                vec![SHADER_TABLE_011F9548 + 16, 0x5004]
            ]
        );
    }

    #[test]
    fn get_shader_without_a_creator_or_out_of_range_answers_0() {
        let mut e = shader_rig();
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        e.call_log = Some(vec![]);
        for index in [
            0u32,
            0xb,
            0xc,
            0x12,
            0x17,
            0x1b,
            0x1c,
            0x1f,
            0x23,
            0xffff_ffff,
        ] {
            assert_eq!(e.call(0x00b5_5560, &args![index]).u32(), 0);
        }
        assert!(calls(&e, NI_POINTER_ASSIGN_00A5F860).is_empty());
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
        // Without the renderer word nothing is looked at.
        e.mem.set_u32(SHADER_TABLE_011F9548 + 4, 0x7777);
        e.set_global(GLOBAL_011F9508, 0u32);
        assert_eq!(e.call(0x00b5_5560, &args![1u32]).u32(), 0);
    }

    #[test]
    fn get_shader_1d_depends_on_its_byte() {
        let mut e = shader_rig();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_5560, &args![0x1du32]).u32(), 0);
        assert!(calls(&e, SHADER_1D_CREATE).is_empty());
        e.set_global(SHADER_1D_SELECT_011F94A6, 1u8);
        assert_eq!(e.call(0x00b5_5560, &args![0x1du32]).u32(), 0x501d);
        assert_eq!(calls(&e, SHADER_1D_CREATE).len(), 1);
    }

    const PIXEL_DATA_VTABLE: u32 = 0x0f00_7000;
    const PIXEL_DATA_DELETE: u32 = 0x0f00_7004;
    const TEXTURE_VTABLE: u32 = 0x0f00_8000;
    const TEXTURE_DELETE: u32 = 0x0f00_8004;

    /// An object of `vtable` with reference count 0.
    fn object(e: &mut Engine, vtable: u32, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The exe's constants, a pixel data constructor laying out `faces` faces of four-byte
    /// texels, a real square root, `00ec6940` answering its argument, and `004dc060`
    /// answering `inverted`.
    fn textures_rig(inverted: u32) -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_7000,
            0x0101_e000,
            0x0102_4000,
            0x0106_b000,
            0x0109_b000,
            0x010a_d000,
            0x011a_a000,
            0x011f_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(CUBE_HALF_SIZE_010240C0, 64.0f64);
        e.set_global(CUBE_MINUS_HALF_SIZE_0106B9F0, -64.0f64);
        e.set_global(CUBE_DEPTH_SQUARED_01017A10, 4096.0f64);
        e.set_global(HALF_01011588, 0.5f64);
        e.set_global(BYTE_SCALE_0101E568, 255.0f64);
        e.set_global(ATTENUATION_CENTRE_010AD890, 63.5f64);
        e.set_global(ZERO_01012060, 0.0f64);
        e.set_global(LOOP_END_0101E704, 128.0f32);
        e.set_global(GLOBAL_011F9508, 0x0777_0000u32);
        for word in 0..0x11u32 {
            e.set_global(CUBE_PIXEL_FORMAT_011AAAC8 + 4 * word, 0x100 + word);
        }
        counting(&mut e);
        e.put_vtable(PIXEL_DATA_VTABLE, &[0, PIXEL_DATA_DELETE]);
        e.put_vtable(TEXTURE_VTABLE, &[0, TEXTURE_DELETE]);
        e.register(PIXEL_DATA_DELETE, |_, _| Ret::default());
        e.register(TEXTURE_DELETE, |_, _| Ret::default());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |e, a| {
            let (this, face) = (a[0], a[1] * a[2] * 4);
            let pixels = e.mem.alloc(face * a[5]);
            let offsets = e.mem.alloc(8);
            e.mem.set_u32(offsets + 4, face);
            e.mem.set_u32(this, PIXEL_DATA_VTABLE);
            e.mem.set_u32(this + PIXELS, pixels);
            e.mem.set_u32(this + OFFSETS_IN_BYTES, offsets);
            e.mem.set_u32(this + MIPMAP_LEVELS, a[4]);
            this.into_ret()
        });
        e.register(CRT_SQRT_00EC6040, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        e.register(CRT_00EC6940, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32),
            ..Ret::default()
        });
        e.register_double(RENDERER_QUERY_004DC060, move |_, _| inverted.into_ret());
        // A fixed string: its text 8 bytes after a count of 1.
        e.register(FIXED_STRING_MAKE, |e, _| {
            let block = e.mem.alloc(16);
            e.mem.set_u32(block, 1);
            (block + 8).into_ret()
        });
        for maker in [
            SOURCE_TEXTURE_FROM_PIXELS,
            SOURCE_TEXTURE_FROM_FILE,
            SOURCE_CUBE_MAP_CREATE,
        ] {
            e.register(maker, |e, _| object(e, TEXTURE_VTABLE, 8).into_ret());
        }
        e.register(BS_SHADER_MANAGER_GET_TEXTURE, |_, _| Ret::default());
        e.register(FORMAT_00406D00, |_, _| Ret::default());
        e
    }

    fn texel(e: &Engine, pixel_data: u32, face: u32, column: u32, row: u32) -> [u8; 4] {
        texel_of(e, pixel_data, 128, face, column, row)
    }

    fn texel_of(
        e: &Engine,
        pixel_data: u32,
        width: u32,
        face: u32,
        column: u32,
        row: u32,
    ) -> [u8; 4] {
        let offsets = e.mem.u32(pixel_data + OFFSETS_IN_BYTES);
        let base = e.mem.u32(pixel_data + PIXELS) + face * e.mem.u32(offsets + 4);
        let at = base + (row * width + column) * 4;
        [
            e.mem.u8(at),
            e.mem.u8(at + 1),
            e.mem.u8(at + 2),
            e.mem.u8(at + 3),
        ]
    }

    #[test]
    fn fn_00b55bb0_makes_every_built_in_texture() {
        let mut e = textures_rig(0);
        // The last texture gets the stack's preferences (2, 0, 0).
        e.register(SOURCE_TEXTURE_FROM_PIXELS, |e, a| {
            if a[2] != FORMAT_PREFS_011A9598 {
                assert_eq!(e.mem.bytes(a[2], 12), [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            }
            object(e, TEXTURE_VTABLE, 8).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b5_5bb0, &args![]);

        let constructed = calls(&e, NI_PIXEL_DATA_CONSTRUCT);
        let shapes: Vec<&[u32]> = constructed.iter().map(|c| &c[1..]).collect();
        assert_eq!(shapes.len(), 9);
        assert_eq!(shapes[0][..2], [0x80, 0x80]);
        assert_eq!(shapes[0][3..], [1, 6]);
        let cube = constructed[0][0];
        // The pixel format was copied onto the stack.
        assert_eq!(e.mem.u32(shapes[0][2]), 0x100);
        assert_eq!(shapes[1], &[0x10, 0x10, PIXEL_FORMAT_011AA2A0, 1, 1]);
        assert_eq!(shapes[4], &[0x10, 0x10, PIXEL_FORMAT_011AA2E8, 1, 1]);
        assert_eq!(shapes[5], &[4, 4, PIXEL_FORMAT_011AA2A0, 1, 1]);
        assert_eq!(shapes[8], &[0x80, 0x80, PIXEL_FORMAT_011AA2E8, 1, 1]);

        // The normalization cube: the centre of each face points along its axis.
        assert_eq!(texel(&e, cube, 0, 64, 64), [127, 127, 255, 0]);
        assert_eq!(texel(&e, cube, 1, 64, 64), [127, 127, 0, 0]);
        assert_eq!(texel(&e, cube, 2, 64, 64), [127, 255, 127, 0]);
        assert_eq!(texel(&e, cube, 3, 64, 64), [127, 0, 127, 0]);
        assert_eq!(texel(&e, cube, 4, 64, 64), [255, 127, 127, 0]);
        assert_eq!(texel(&e, cube, 5, 64, 64), [0, 127, 127, 0]);
        // A corner: every component 1/sqrt(3) in size.
        assert_eq!(texel(&e, cube, 0, 0, 0), [201, 201, 201, 0]);
        assert_eq!(texel(&e, cube, 1, 0, 0), [53, 201, 53, 0]);
        assert_eq!(texel(&e, cube, 2, 0, 0), [53, 201, 53, 0]);
        assert_eq!(e.mem.u32(cube + REVISION), 1);

        // The solid textures.
        let error_map = constructed[1][0];
        assert_eq!(texel_of(&e, error_map, 16, 0, 15, 15), [0xff, 0, 0, 0xff]);
        assert_eq!(e.mem.u8(error_map + NO_CONVERT), 1);
        assert_eq!(e.mem.u32(error_map + REVISION), 1);
        let effect_map = constructed[6][0];
        assert_eq!(texel_of(&e, effect_map, 4, 0, 3, 3), [0, 0, 0, 0xff]);
        let height_map = constructed[4][0];
        let pixels = e.mem.u32(height_map + PIXELS);
        assert_eq!(e.mem.bytes(pixels, 0x300), vec![0; 0x300]);

        // Every texture global got its texture, held once (0x9534's twice: 0x9538 too).
        let made = calls(&e, SOURCE_TEXTURE_FROM_PIXELS);
        assert_eq!(made.len(), 8);
        for call in &made[..7] {
            assert_eq!(call[2], FORMAT_PREFS_011A9598);
        }
        assert_ne!(made[7][2], FORMAT_PREFS_011A9598);
        for global in [
            TEXTURE_011F9510,
            TEXTURE_011F9514,
            TEXTURE_011F9518,
            TEXTURE_011F951C,
            TEXTURE_011F9528,
            TEXTURE_011F952C,
            TEXTURE_011F9530,
            TEXTURE_011F953C,
            TEXTURE_011F9540,
            TEXTURE_011F9544,
            TEXTURE_011F95D4,
        ] {
            let texture = e.global::<u32>(global);
            assert_ne!(texture, 0, "{global:#x}");
            assert_eq!(e.mem.u32(texture + 4), 1, "{global:#x}");
        }
        let attenuation = e.global::<u32>(TEXTURE_011F9534);
        assert_eq!(e.global::<u32>(TEXTURE_011F9538), attenuation);
        assert_eq!(e.mem.u32(attenuation + 4), 2);

        assert_eq!(
            calls(&e, BS_SHADER_MANAGER_GET_TEXTURE),
            vec![
                vec![0x010a_d9a4, 1, TEXTURE_011F9520, 1, 0],
                vec![0x010a_d970, 1, TEXTURE_011F9524, 1, 0]
            ]
        );
        let formats: Vec<u32> = calls(&e, FORMAT_00406D00).iter().map(|c| c[2]).collect();
        assert_eq!(
            formats,
            vec![0x010a_d910, 0x010a_d8e4, 0x010a_d8c0, 0x010a_d8b0]
        );
        let cube_maps = calls(&e, SOURCE_CUBE_MAP_CREATE);
        assert_eq!(cube_maps.len(), 2);
        let buffer = cube_maps[0][0];
        assert_eq!(
            cube_maps[0][..7],
            [buffer, buffer, buffer, buffer, buffer, buffer, 0x0777_0000]
        );
        assert_eq!(cube_maps[0][8], FORMAT_PREFS_011A9598);
        let from_file = calls(&e, SOURCE_TEXTURE_FROM_FILE);
        assert_eq!(from_file[0][1..], [FORMAT_PREFS_011A9598, 1, 0]);

        // The attenuation map: full at the corners, empty at the centre.
        let attenuation_data = constructed[8][0];
        let pixels = e.mem.u32(attenuation_data + PIXELS);
        let at = |column: u32, row: u32| pixels + (row * 128 + column) * 3;
        assert_eq!(e.mem.bytes(at(0, 0), 3), vec![255; 3]);
        assert_eq!(e.mem.bytes(at(127, 127), 3), vec![255; 3]);
        assert_eq!(e.mem.bytes(at(63, 63), 3), vec![0; 3]);

        // Every pixel data deleted at the end (the normal map's when the height map
        // replaced it), every name's count back to 0.
        let deleted: Vec<u32> = calls(&e, PIXEL_DATA_DELETE).iter().map(|c| c[0]).collect();
        assert_eq!(deleted.len(), 9);
        assert_eq!(deleted[0], constructed[2][0]);
        assert_eq!(*deleted.last().unwrap(), cube);
        let names: Vec<u32> = made.iter().map(|c| c[1]).collect();
        for name in names {
            assert_eq!(e.mem.u32(name - 8), 0);
        }
    }

    #[test]
    fn fn_00b55bb0_inverts_the_attenuation_when_asked() {
        let mut e = textures_rig(1);
        e.call_log = Some(vec![]);
        e.call(0x00b5_5bb0, &args![]);
        let constructed = calls(&e, NI_PIXEL_DATA_CONSTRUCT);
        let pixels = e.mem.u32(constructed[8][0] + PIXELS);
        assert_eq!(e.mem.bytes(pixels, 3), vec![0; 3]);
        let centre = pixels + (63 * 128 + 63) * 3;
        assert_eq!(e.mem.bytes(centre, 3), vec![254; 3]);
    }

    #[test]
    fn register_streamables_registers_the_twenty_loaders_in_order() {
        let mut e = Engine::new();
        e.register(NI_STREAM_REGISTER_LOADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b4_f1a0, &args![]);
        let registered = calls(&e, NI_STREAM_REGISTER_LOADER);
        assert_eq!(registered.len(), 20);
        assert_eq!(registered[0], vec![0x010a_93a0, 0x00ba_9250]);
        assert_eq!(registered[5], vec![0x010a_9320, 0x00bb_45d0]);
        assert_eq!(registered[19], vec![0x010a_91dc, 0x00bb_7300]);
    }

    fn fog_rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x010a_d000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(FOG_DISTANCE_010AD6E0, 500_000.0f32);
        e.set_global(HALF_01016248, 0.5f32);
        counting(&mut e);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BS_FOG_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(0x0f00_2000, |_, _| Ret::default());
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_2000]);
        e
    }

    fn held(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x64);
        e.mem.set_u32(object, 0x0f00_1000);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn set_fog_property_without_one_makes_the_default_fog() {
        let mut e = fog_rig();
        let node = e.mem.alloc(0x200);
        let old = held(&mut e, 1);
        e.mem.set_u32(node + 0x134, old);
        e.call_log = Some(vec![]);
        e.call(0x00b5_4110, &args![node, 0u32]);
        assert_eq!(calls(&e, NEW_OBJECT), vec![vec![0x64]]);
        let fog = e.mem.u32(node + 0x134);
        assert_eq!(calls(&e, BS_FOG_PROPERTY_CONSTRUCT), vec![vec![fog]]);
        assert_eq!(calls(&e, 0x0f00_2000), vec![vec![old]]);
        assert_eq!(e.mem.u32(fog + 4), 1);
        for offset in [0x2c, 0x30, 0x34, 0x38] {
            assert_eq!(e.mem.f32(fog + offset), 500_000.0, "+{offset:#x}");
        }
        assert_eq!(e.mem.f32(fog + 0x60), 1.0);
        for offset in [0x20, 0x24, 0x28, 0x54, 0x58, 0x5c] {
            assert_eq!(e.mem.f32(fog + offset), 0.5, "+{offset:#x}");
        }
    }

    #[test]
    fn set_fog_property_assigns_a_given_fog_once() {
        let mut e = fog_rig();
        let node = e.mem.alloc(0x200);
        let fog = held(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b5_4110, &args![node, fog]);
        e.call(0x00b5_4110, &args![node, fog]);
        assert_eq!(e.mem.u32(node + 0x134), fog);
        assert_eq!(e.mem.u32(fog + 4), 1);
        assert!(calls(&e, NEW_OBJECT).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(e.mem.f32(fog + 0x2c), 0.0);
        // A second fog releases the first, which is deleted at 0.
        let other = held(&mut e, 0);
        e.call(0x00b5_4110, &args![node, other]);
        assert_eq!(calls(&e, 0x0f00_2000), vec![vec![fog]]);
        assert_eq!(e.mem.u32(node + 0x134), other);
    }
}

#[cfg(test)]
mod b000f_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const DELETE: u32 = 0x0f00_2004;
    const LEVELS: u32 = 0x0f00_2098;

    /// A texture object (vtable with slot +4 `DELETE` and +0x98 `LEVELS`, its own renderer
    /// data at +0x24 using the same vtable), held `count` times.
    fn texture(e: &mut Engine, count: u32, with_data: bool) -> u32 {
        let mut slots = vec![0u32; 0x27];
        slots[1] = DELETE;
        slots[0x26] = LEVELS;
        e.put_vtable(0x0f00_1000, &slots);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, 0x0f00_1000);
        e.mem.set_u32(object + 4, count);
        if with_data {
            let data = e.mem.alloc(8);
            e.mem.set_u32(data, 0x0f00_1000);
            e.mem.set_u32(object + 0x24, data);
        }
        e.mem.set_u32(object + 0x34, 0x5150);
        object
    }

    /// Doubles for every callee; the palette leaves the pointer as it is, the finder answers
    /// `found`, the makers answer `made`, the mipmap count is `levels`.
    fn rig(found: u32, made: u32, levels: u32, kind_ok: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x2000);
        e.set_global(FILE_FINDER_011F81DC, 0x0f00_9000u32);
        e.set_global(GLOBAL_011F9508, 0x0f00_9100u32);
        e.register(TEXTURE_PALETTE_GET_TEXTURE, |_, _| Ret::default());
        e.register(TEXTURE_PALETTE_SET_TEXTURE, |_, _| Ret::default());
        e.register_double(FILE_FINDER_FIND_00AFE0D0, move |_, _| found.into_ret());
        e.register(NAME_MAKE_00438170, |_, a| (a[0] + 0x100).into_ret());
        e.register(NAME_CLEAN_UP_004381B0, |_, _| Ret::default());
        e.register_double(SOURCE_CUBE_MAP_FROM_FILE_00A60F60, move |_, _| {
            made.into_ret()
        });
        e.register_double(SOURCE_TEXTURE_FROM_FILE, move |_, _| made.into_ret());
        e.register(NI_POINTER_ASSIGN_0066B0D0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register_double(KIND_TEST_00653290, move |_, a| {
            if kind_ok { a[0] } else { 0 }.into_ret()
        });
        e.register_double(LEVELS, move |_, _| levels.into_ret());
        e.register(FORMAT_00406D00, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e
    }

    #[test]
    fn get_texture_loads_a_missing_texture_from_its_file() {
        for cube in [false, true] {
            let mut e = rig(1, 0, 2, true);
            let made = texture(&mut e, 1, true);
            e.register_double(SOURCE_CUBE_MAP_FROM_FILE_00A60F60, move |_, _| {
                made.into_ret()
            });
            e.register_double(SOURCE_TEXTURE_FROM_FILE, move |_, _| made.into_ret());
            let slot = e.mem.alloc(4);
            e.call_log = Some(vec![]);
            e.call(
                0x00b5_5840,
                &args![0x7000u32, 1u32, slot, 1u32, u32::from(cube)],
            );
            assert_eq!(
                calls(&e, TEXTURE_PALETTE_GET_TEXTURE),
                vec![vec![0x7000, slot]]
            );
            assert_eq!(
                calls(&e, FILE_FINDER_FIND_00AFE0D0),
                vec![vec![0x0f00_9000, 0x7000, 0, 6, 2]]
            );
            let name = calls(&e, NAME_MAKE_00438170);
            assert_eq!(name[0][1], 0x7000);
            let local = name[0][0];
            if cube {
                assert_eq!(
                    calls(&e, SOURCE_CUBE_MAP_FROM_FILE_00A60F60),
                    vec![vec![local + 0x100, 0x0f00_9100, FORMAT_PREFS_011A9598]]
                );
                assert!(calls(&e, SOURCE_TEXTURE_FROM_FILE).is_empty());
            } else {
                assert_eq!(
                    calls(&e, SOURCE_TEXTURE_FROM_FILE),
                    vec![vec![local + 0x100, FORMAT_PREFS_011A9598, 1, 0]]
                );
            }
            assert_eq!(
                calls(&e, NI_POINTER_ASSIGN_0066B0D0),
                vec![vec![slot, made]]
            );
            assert_eq!(calls(&e, NAME_CLEAN_UP_004381B0), vec![vec![local]]);
            assert_eq!(calls(&e, TEXTURE_PALETTE_SET_TEXTURE), vec![vec![made, 0]]);
            // Two mipmap levels: kept.
            assert_eq!(
                calls(&e, KIND_TEST_00653290),
                vec![vec![made, RTTI_RECORD_011F444C]]
            );
            assert!(calls(&e, FORMAT_00406D00).is_empty());
            assert_eq!(e.mem.u32(slot), made);
        }
    }

    #[test]
    fn get_texture_refuses_a_texture_without_mipmaps() {
        for kind_ok in [true, false] {
            let mut e = rig(0, 0, 1, kind_ok);
            let held = texture(&mut e, 1, true);
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, held);
            e.call_log = Some(vec![]);
            e.call(0x00b5_5840, &args![0x7000u32, 1u32, slot, 1u32, 0u32]);
            // Already there: nothing looked up.
            assert!(calls(&e, FILE_FINDER_FIND_00AFE0D0).is_empty());
            let message = calls(&e, FORMAT_00406D00);
            assert_eq!(message.len(), 1);
            if kind_ok {
                assert_eq!(&message[0][1..], [500, NO_MIPMAPS_MESSAGE_010AD830, 0x5150]);
            } else {
                assert_eq!(&message[0][1..], [500, NO_MIPMAPS_PLAIN_MESSAGE_010AD7D8]);
            }
            assert_eq!(calls(&e, DELETE), vec![vec![held]]);
            assert_eq!(e.mem.u32(slot), 0);
        }
    }

    #[test]
    fn get_texture_keeps_what_the_checks_do_not_reach() {
        // Without the load byte: only the palette.
        let mut e = rig(1, 0, 0, true);
        let held = texture(&mut e, 1, true);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, held);
        e.call_log = Some(vec![]);
        e.call(0x00b5_5840, &args![0x7000u32, 0u32, slot, 1u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.mem.u32(slot), held);
        // Without the mipmap byte, or without renderer data: kept.
        for (refuse, with_data) in [(0u32, true), (1, false)] {
            let mut e = rig(1, 0, 0, true);
            let held = texture(&mut e, 1, with_data);
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, held);
            e.call_log = Some(vec![]);
            e.call(0x00b5_5840, &args![0x7000u32, 1u32, slot, refuse, 0u32]);
            assert!(calls(&e, LEVELS).is_empty());
            assert!(calls(&e, DELETE).is_empty());
            assert_eq!(e.mem.u32(slot), held);
        }
        // No file finder, or the file is not found: the pointer stays null.
        for (finder, found) in [(0u32, 1u32), (0x0f00_9000, 0)] {
            let mut e = rig(found, 0, 0, true);
            e.set_global(FILE_FINDER_011F81DC, finder);
            let slot = e.mem.alloc(4);
            e.call_log = Some(vec![]);
            e.call(0x00b5_5840, &args![0x7000u32, 1u32, slot, 1u32, 0u32]);
            assert!(calls(&e, NAME_MAKE_00438170).is_empty());
            assert_eq!(
                calls(&e, FILE_FINDER_FIND_00AFE0D0).len(),
                usize::from(finder != 0)
            );
            assert!(calls(&e, KIND_TEST_00653290).is_empty());
        }
        // Found but the maker answers none: the palette is not told.
        let mut e = rig(1, 0, 0, true);
        let slot = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00b5_5840, &args![0x7000u32, 1u32, slot, 1u32, 0u32]);
        assert!(calls(&e, TEXTURE_PALETTE_SET_TEXTURE).is_empty());
        assert!(calls(&e, KIND_TEST_00653290).is_empty());
    }

    #[test]
    fn get_texture_without_a_name_releases_the_pointer() {
        let mut e = rig(1, 0, 0, true);
        let held = texture(&mut e, 2, true);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, held);
        e.call_log = Some(vec![]);
        e.call(0x00b5_5840, &args![0u32, 1u32, slot, 1u32, 0u32]);
        assert!(calls(&e, TEXTURE_PALETTE_GET_TEXTURE).is_empty());
        assert!(calls(&e, DELETE).is_empty());
        assert_eq!(e.mem.u32(held + 4), 1);
        assert_eq!(e.mem.u32(slot), 0);
    }
}

/// Test support for the shader creators that `GetShader` (`00b55560`) calls (each in its own
/// unit file): an engine where the renderer, `NiXenonShaderDeclaration::Create`, the Ni
/// allocation, the shader's constructor and every slot of the declarations' and the shader's
/// vtables are doubles, so a test can compare a creator's whole call sequence.
#[cfg(test)]
pub(crate) mod creator_rig {
    use crate::prelude::*;

    /// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry
    /// count, stream count) and the Ni allocation (`00aa13e0`, `cdecl`: size).
    const DECLARATION_CREATE: u32 = 0x00e7_6700;
    const NI_ALLOCATE: u32 = 0x00aa_13e0;
    /// The renderer word (`011f9508`) and the value the rig puts there.
    pub(crate) const RENDERER_GLOBAL: u32 = 0x011f_9508;
    pub(crate) const RENDERER: u32 = 0x0f50_0000;
    /// The declarations the rig's `Create` answers, in order (0x100 bytes apart).
    pub(crate) const DECLARATION_0: u32 = 0x0f60_0000;
    pub(crate) const DECLARATION_1: u32 = 0x0f60_0100;
    /// The block the rig's allocation answers.
    pub(crate) const SHADER: u32 = 0x0f70_0000;
    /// The word where the rig's constructor double notes the thread's allocation context.
    pub(crate) const CONTEXT_SEEN: u32 = 0x0f80_0000;
    const DECLARATION_VTABLE: u32 = 0x0f10_0000;
    const SHADER_VTABLE: u32 = 0x0f20_0000;
    const DECLARATION_SLOTS: u32 = 0x0f30_0000;
    const SHADER_SLOTS: u32 = 0x0f40_0000;
    const SLOTS: u32 = 0x100;

    /// The double behind a declaration's vtable slot at byte offset `offset`.
    pub(crate) fn declaration_slot(offset: u32) -> u32 {
        DECLARATION_SLOTS + offset
    }

    /// The double behind the shader's vtable slot at byte offset `offset`.
    pub(crate) fn shader_slot(offset: u32) -> u32 {
        SHADER_SLOTS + offset
    }

    /// The rig: `constructor` (the shader's, at its address) sets the shader's vtable, notes
    /// the allocation context and answers `this`; every address of `direct` is a no-op.
    pub(crate) fn rig(constructor: u32, direct: &[u32]) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(RENDERER_GLOBAL, RENDERER);
        e.map(DECLARATION_0, 0x1000);
        e.map(SHADER, 0x1000);
        e.map(CONTEXT_SEEN, 4);
        let declaration_slots: Vec<u32> = (0..SLOTS).map(|i| DECLARATION_SLOTS + 4 * i).collect();
        let shader_slots: Vec<u32> = (0..SLOTS).map(|i| SHADER_SLOTS + 4 * i).collect();
        e.put_vtable(DECLARATION_VTABLE, &declaration_slots);
        e.put_vtable(SHADER_VTABLE, &shader_slots);
        for target in declaration_slots.iter().chain(&shader_slots).chain(direct) {
            e.register(*target, |_, _| Ret::default());
        }
        let mut made = 0u32;
        e.register_double(DECLARATION_CREATE, move |e, _| {
            let declaration = DECLARATION_0 + 0x100 * made;
            made += 1;
            e.mem.set_u32(declaration, DECLARATION_VTABLE);
            declaration.into_ret()
        });
        e.register(NI_ALLOCATE, |_, _| SHADER.into_ret());
        e.register(constructor, |e, a| {
            e.mem.set_u32(a[0], SHADER_VTABLE);
            let tls = e.tls();
            let context = e.mem.u32(tls + 0x2b4);
            e.mem.set_u32(CONTEXT_SEEN, context);
            a[0].into_ret()
        });
        e
    }

    /// Runs the creator at `creator` with the thread's allocation context at 0x99 and answers
    /// its result and the calls it made (without the top-level one). Checks the context is
    /// back to 0x99 afterwards.
    pub(crate) fn run(e: &mut Engine, creator: u32) -> (u32, Vec<(u32, Vec<u32>)>) {
        let tls = e.tls();
        e.mem.set_u32(tls + 0x2b4, 0x99);
        e.call_log = Some(vec![]);
        let answer = e.call(creator, &[]).u32();
        assert_eq!(e.mem.u32(tls + 0x2b4), 0x99);
        let mut log = e.call_log.take().unwrap();
        log.remove(0);
        (answer, log)
    }
}

#[cfg(test)]
mod profile_tests {
    use super::*;

    fn engine(first: u32, second: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(PROFILE_SETTING_011F91BC, first);
        e.set_global(PROFILE_SETTING_011F91C0, second);
        e
    }

    #[test]
    fn fn_00b4f380_picks_the_pixel_profile() {
        let expected = [
            (0u32, 0u32),
            (1, PS_1_3_010A93D4),
            (2, PS_2_0_010A93CC),
            (3, PS_2_A_010A93C4),
            (4, PS_2_B_010A93BC),
            (5, PS_2_A_010A93C4),
            (6, PS_2_B_010A93BC),
            (7, PS_3_0_010A93B4),
            (8, 0),
        ];
        for (setting, profile) in expected {
            let mut e = engine(setting, 0);
            assert_eq!(e.call(0x00b4_f380, &args![1u32]).u32(), profile);
            let mut e = engine(0, setting);
            assert_eq!(e.call(0x00b4_f380, &args![0u32]).u32(), profile);
        }
        // Only the low byte of the argument word counts.
        let mut e = engine(7, 1);
        assert_eq!(e.call(0x00b4_f380, &args![0x100u32]).u32(), PS_1_3_010A93D4);
    }

    #[test]
    fn fn_00b4f3e0_picks_the_vertex_profile() {
        let expected = [
            (0u32, 0u32),
            (1, VS_1_1_010A93EC),
            (2, VS_2_0_010A93E4),
            (6, VS_2_0_010A93E4),
            (7, VS_3_0_010A93DC),
            (8, 0),
        ];
        for (setting, profile) in expected {
            let mut e = engine(7, setting);
            assert_eq!(e.call(0x00b4_f3e0, &args![]).u32(), profile);
        }
    }
}

#[cfg(test)]
mod b000s_tests {
    use super::*;

    const DEVICE: u32 = 0x5000;
    const ATI: u32 = 0x5100;
    const NV: u32 = 0x5200;
    const IG: u32 = 0x5300;
    const OTHER: u32 = 0x5400;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9180, 0x80);
        e.map(0x011f_9400, 0x100);
        e.map(0x011a_d800, 0x80);
        e.map(FLOAT_TARGET_01200458, 4);
        e.map(FLOAT_SOURCE_010A9478, 4);
        e.set_global(FLOAT_SOURCE_010A9478, 0xb7a7_c5acu32);
        // `_strnicmp` double: the drivers are named by address.
        e.register(STRNICMP, |_, a| {
            let hit = matches!(
                (a[0], a[1]),
                (ATI, PREFIX_ATI_010A9484) | (NV, PREFIX_NV_010A9480) | (IG, PREFIX_IG_010A947C)
            );
            u32::from(!hit).into_ret()
        });
        e
    }

    fn configure(e: &mut Engine, versions: (u32, u32), force: u32, driver: u32, count: i32) {
        e.call(
            0x00b4_f710,
            &args![versions.0, versions.1, force, DEVICE, driver, count],
        );
    }

    fn state(e: &Engine) -> (u32, u32, u32, u8, u32) {
        (
            e.global(PROFILE_SETTING_011F91C0),
            e.global(PROFILE_SETTING_011F91BC),
            e.global(SHADER_FLAGS_011F91D8),
            e.global(SHADER_1D_SELECT_011F94A6),
            e.global(VENDOR_011F94B8),
        )
    }

    #[test]
    fn get_shader_level_string_names_levels_0_to_7() {
        let mut e = rig();
        for (level, name) in SHADER_LEVEL_NAMES.iter().enumerate() {
            e.set_global(PROFILE_SETTING_011F91C0, level as u32);
            assert_eq!(e.call(0x00b4_f4a0, &args![]).u32(), *name);
        }
        e.set_global(PROFILE_SETTING_011F91C0, 8u32);
        assert_eq!(e.call(0x00b4_f4a0, &args![]).u32(), SHADER_LEVEL_UNKNOWN);
    }

    #[test]
    fn fn_00b4f710_picks_the_level_by_driver() {
        let mut e = rig();
        configure(&mut e, (0x300, 0x300), 0, ATI, 0x200);
        assert_eq!(state(&e), (6, 6, 0x2f, 1, 2));
        assert_eq!(e.global::<u32>(VENDOR_SETTING_011AD870), 0x15);
        assert_eq!(e.global::<u8>(VERSION_3_BOTH_011F94A4), 1);
        assert_eq!(e.global::<u32>(FLOAT_TARGET_01200458), 0xb7a7_c5ac);

        let mut e = rig();
        configure(&mut e, (0x200, 0x300), 0, ATI, 0x60);
        assert_eq!(state(&e), (4, 6, 0x2f, 0, 2));
        assert_eq!(e.global::<u8>(VERSION_3_BOTH_011F94A4), 0);

        let mut e = rig();
        configure(&mut e, (0x300, 0x300), 0, NV, 0x100);
        assert_eq!(state(&e), (5, 5, 0x2f, 1, 1));
        let mut e = rig();
        configure(&mut e, (0x300, 0x300), 0, NV, 0x61);
        assert_eq!(state(&e), (5, 5, 0x2f, 0, 1));
        let mut e = rig();
        configure(&mut e, (0x300, 0x300), 0, IG, 0x10);
        assert_eq!(state(&e), (3, 5, 0x2f, 0, 3));
        assert_eq!(e.global::<u32>(VENDOR_SETTING_011AD870), 0);

        let mut e = rig();
        configure(&mut e, (0x300, 0x300), 0, OTHER, 0x200);
        assert_eq!(state(&e), (2, 2, 0x2f, 0, 0));
    }

    #[test]
    fn fn_00b4f710_without_names_keeps_level_2() {
        let mut e = rig();
        e.set_global(PROFILE_SETTING_011F91BC, 9u32);
        e.call(
            0x00b4_f710,
            &args![0x200u32, 0x200u32, 0u32, 0u32, ATI, 0x200u32],
        );
        assert_eq!(state(&e), (2, 9, 0x2f, 0, 0));
        assert_eq!(e.global::<u32>(FLOAT_TARGET_01200458), 0);
    }

    #[test]
    fn fn_00b4f710_low_versions_and_the_force_byte() {
        let mut e = rig();
        e.set_global(SHADER_FLAGS_011F91D8, 0xffu32);
        e.set_global(BELOW_LEVEL_2_FLAG_011F941E, 1u8);
        configure(&mut e, (0x100, 0x2ff), 0, ATI, 0x200);
        assert_eq!(state(&e).0, 1);
        assert_eq!(e.global::<u32>(SHADER_FLAGS_011F91D8), 0xf);
        assert_eq!(e.global::<u8>(BELOW_LEVEL_2_FLAG_011F941E), 0);

        let mut e = rig();
        e.set_global(SHADER_FLAGS_011F91D8, 0xffu32);
        configure(&mut e, (0x0ff, 0x300), 1, ATI, 0x200);
        // Level 0 stays 0 even when forced.
        assert_eq!(state(&e).0, 0);
        assert_eq!(e.global::<u32>(SHADER_FLAGS_011F91D8), 0xef);

        let mut e = rig();
        configure(&mut e, (0x300, 0x300), 1, ATI, 0x200);
        assert_eq!(state(&e).0, 1);
    }

    #[test]
    fn fn_00b4f710_keeps_011ad80e_only_with_both_bytes() {
        let mut e = rig();
        e.set_global(KEPT_FLAG_011AD80E, 1u8);
        e.set_global(REQUIRED_FLAG_011F94D8, 1u8);
        e.set_global(COPY_SOURCE_011F94E4, 0x5au8);
        configure(&mut e, (0x300, 0x300), 0, OTHER, 0);
        assert_eq!(e.global::<u8>(KEPT_FLAG_011AD80E), 1);
        assert_eq!(e.global::<u8>(COPIED_FLAG_011F91A4), 0x5a);
        configure(&mut e, (0x300, 0x300), 0, OTHER, 0);
        assert_eq!(e.global::<u8>(KEPT_FLAG_011AD80E), 1);

        let mut e = rig();
        e.set_global(KEPT_FLAG_011AD80E, 1u8);
        e.set_global(COPY_SOURCE_011F94E4, 7u8);
        configure(&mut e, (0x300, 0x300), 0, OTHER, 0);
        assert_eq!(e.global::<u8>(KEPT_FLAG_011AD80E), 0);
        assert_eq!(e.global::<u8>(COPIED_FLAG_011F91A4), 7);
    }
}

#[cfg(test)]
mod b047_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// An engine with the manager's globals mapped, the interlocked imports
    /// counting for real and the memory manager allocating.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011f_9000u32,
            0x011f_a000,
            0x011a_d000,
            0x0102_f000,
            0x0109_b000,
            0x0120_0000,
            0x011f_4000,
            0x0101_1000,
            0x0102_3000,
            0x010a_d000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            eax(value)
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            eax(value)
        });
        e.register(MEMORY_ALLOCATE, |e, a| eax(e.mem.alloc(a[1].max(4))));
        stub(&mut e, &[MEMORY_DEALLOCATE]);
        e
    }

    /// A reference-counted object (count `count`) whose vtable has the test
    /// functions `0x7000 + slot`.
    fn object(e: &mut Engine, count: u32) -> u32 {
        let slots: Vec<u32> = (0..0x50).map(|slot| 0x0000_7000 + 4 * slot).collect();
        e.put_vtable(0x0f00_0000, &slots);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, 0x0f00_0000);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn fn_00b4f300_centres_the_rectangle() {
        let mut e = engine();
        let rect = e.mem.alloc(0x10);
        for (offset, value) in [(0, 10i32), (4, 50), (8, 31), (0xc, 7)] {
            e.mem.set_i32(rect + offset, value);
        }
        e.call(0x00b4_f300, &args![rect]);
        // (31 - 10) / 2 = 10, (50 - 7) / 2 = 21.
        assert_eq!(e.global::<f32>(VIEWPORT_HALF_WIDTH_011F95FC), 10.0);
        assert_eq!(e.global::<f32>(VIEWPORT_HALF_HEIGHT_011F9600), 21.0);
        assert_eq!(e.global::<f32>(VIEWPORT_CENTRE_X_011F95F4), 20.0);
        assert_eq!(e.global::<f32>(VIEWPORT_CENTRE_Y_011F95F8), 28.0);
        // Negative extents divide toward zero.
        e.mem.set_i32(rect + 8, 5);
        e.call(0x00b4_f300, &args![rect]);
        assert_eq!(e.global::<f32>(VIEWPORT_HALF_WIDTH_011F95FC), -2.0);
        assert_eq!(e.global::<f32>(VIEWPORT_CENTRE_X_011F95F4), 8.0);
    }

    #[test]
    fn fn_00b4f430_passes_the_word_on() {
        let mut e = engine();
        stub(&mut e, &[KEEP_VALUE_00B8CAD0]);
        e.set_global(OBJECT_011F91AC, 0x4400u32);
        e.call_log = Some(vec![]);
        e.call(0x00b4_f430, &args![9u32]);
        assert_eq!(calls_to(&e, KEEP_VALUE_00B8CAD0), vec![vec![0x4400, 9]]);
    }

    #[test]
    fn fn_00b4f450_selects_with_fallbacks() {
        let mut e = engine();
        e.set_global(FIRST_TABLE_011F9F80, 0x100u32);
        e.set_global(FIRST_TABLE_011F9F80 + 8, 0x102u32);
        e.set_global(SECOND_TABLE_011F9F40, 0x200u32);
        e.call(0x00b4_f450, &args![2u32]);
        assert_eq!(e.global::<u32>(SELECTED_INDEX_011F91B8), 2);
        assert_eq!(e.global::<u32>(FIRST_SELECTED_011F9F3C), 0x102);
        assert_eq!(e.global::<u32>(SECOND_SELECTED_011F9F38), 0x200);
        e.set_global(SECOND_TABLE_011F9F40 + 4, 0x201u32);
        e.call(0x00b4_f450, &args![1u32]);
        assert_eq!(e.global::<u32>(FIRST_SELECTED_011F9F3C), 0x100);
        assert_eq!(e.global::<u32>(SECOND_SELECTED_011F9F38), 0x201);
    }

    #[test]
    fn small_getters_read_their_globals() {
        let mut e = engine();
        e.set_global(SECOND_ACCUMULATOR_011F95F0, 0x55u32);
        assert_eq!(e.call(0x00b4_f5d0, &args![]).u32(), 0x55);
        let node = e.mem.alloc(0x140);
        e.mem.set_u32(node + 0x134, 0x77);
        e.set_global(SHADOW_SCENE_NODES + 8, node);
        assert_eq!(e.call(0x00b5_5520, &args![2u32]).u32(), 0x77);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x88);
        e.set_global(ACCUMULATOR_ARRAY_011F9170, array);
        e.set_global(ACCUMULATOR_INDEX_011F9168, 1u32);
        assert_eq!(e.call(0x00b5_40f0, &args![]).u32(), 0x88);
        assert_eq!(e.global::<u32>(ACCUMULATOR_INDEX_011F9168), 2);
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 5);
        e.call(0x00b5_4620, &args![word]);
        assert_eq!(e.mem.u32(word), 0);
    }

    #[test]
    fn image_space_texture_jumps_use_the_texture_at_0x204() {
        let mut e = engine();
        e.register(IMAGE_SPACE_TEXTURE_GET_TEXTURE, |_, _| eax(0x66));
        e.register(IMAGE_SPACE_TEXTURE_RETURN_RENDERED, |_, _| eax(1));
        e.set_global(OBJECT_011F91AC, 0x4000u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_4090, &args![]).u32(), 0x66);
        assert_eq!(e.call(0x00b5_40b0, &args![]).u32(), 1);
        assert_eq!(
            calls_to(&e, IMAGE_SPACE_TEXTURE_GET_TEXTURE),
            vec![vec![0x4204]]
        );
        assert_eq!(
            calls_to(&e, IMAGE_SPACE_TEXTURE_RETURN_RENDERED),
            vec![vec![0x4204]]
        );
    }

    fn filename_engine() -> Engine {
        let mut e = engine();
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let found = text.iter().rposition(|&c| u32::from(c) == a[1]);
            eax(found.map_or(0, |at| a[0] + at as u32))
        });
        e.register(STRNICMP, |e, a| {
            let text = e.mem.cstr(a[0]);
            eax(u32::from(!text.to_ascii_lowercase().starts_with(b"data")))
        });
        e.register(FORMAT_00406D00, |_, _| Ret::default());
        e
    }

    #[test]
    fn modified_texture_filename_cuts_the_name_and_picks_the_format() {
        let mut e = filename_engine();
        let destination = e.mem.alloc(0x104);
        let source = e.mem.alloc(0x40);
        e.mem.set_cstr(source, b"textures\\rock_d.dds");
        e.mem.set_u8(destination, 0x41);
        e.call_log = Some(vec![]);
        e.call(0x00b4_f5e0, &args![destination, source, 0x5000u32, 1u32]);
        assert_eq!(e.mem.u8(destination), 0);
        let formats = calls_to(&e, FORMAT_00406D00);
        assert_eq!(formats.len(), 1);
        let call = &formats[0];
        assert_eq!(call[..3], [destination, 0x104, DATA_FORMAT_010A9464]);
        assert_eq!(e.mem.cstr(call[3]), b"textures\\rock".to_vec());
        assert_eq!(call[4], 0x5000);
        assert_eq!(e.mem.cstr(call[5]), b".dds".to_vec());

        // Already under data, or without the prefix flag: the plain format.
        e.mem.set_cstr(source, b"Data\\a.dds");
        e.call_log = Some(vec![]);
        e.call(0x00b4_f5e0, &args![destination, source, 0x5000u32, 1u32]);
        assert_eq!(calls_to(&e, FORMAT_00406D00)[0][2], PLAIN_FORMAT_0104F208);
        e.mem.set_cstr(source, b"x\\a_b.dds");
        e.call_log = Some(vec![]);
        e.call(0x00b4_f5e0, &args![destination, source, 0x5000u32, 0u32]);
        let call = calls_to(&e, FORMAT_00406D00)[0].clone();
        assert_eq!(call[2], PLAIN_FORMAT_0104F208);
        assert_eq!(e.mem.cstr(call[3]), b"x\\a".to_vec());
        // An underscore before the last backslash stays.
        e.mem.set_cstr(source, b"a_b\\c.dds");
        e.call_log = Some(vec![]);
        e.call(0x00b4_f5e0, &args![destination, source, 0x5000u32, 0u32]);
        let call = calls_to(&e, FORMAT_00406D00)[0].clone();
        assert_eq!(e.mem.cstr(call[3]), b"a_b\\c".to_vec());
        // No extension: nothing formatted.
        e.mem.set_cstr(source, b"plain");
        e.call_log = Some(vec![]);
        e.call(0x00b4_f5e0, &args![destination, source, 0x5000u32, 1u32]);
        assert!(calls_to(&e, FORMAT_00406D00).is_empty());
    }

    #[test]
    fn fn_00b54280_resets_and_releases() {
        let mut e = engine();
        e.map(FIRST_FACTOR, 8);
        e.map(REFERENCE_ANGLE, 8);
        e.set_global(FIRST_FACTOR, 0.5f64);
        e.set_global(SECOND_FACTOR, 2.0f64);
        e.set_global(REFERENCE_ANGLE, 3.0f64);
        e.register(CRT_SIN, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register(CRT_COS, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        stub(&mut e, &[0x0000_7004]);
        let shader = object(&mut e, 1);
        let texture = object(&mut e, 3);
        e.set_global(SHADER_TABLE_011F9548 + 0x10, shader);
        e.set_global(TEXTURE_011F9510, texture);
        e.set_global(ACCUMULATOR, texture);
        e.set_global(OBJECT_011F91AC, 0x99u32);
        e.set_global(DEFAULT_VALUE_0102F0F8, 0.25f32);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x00b5_4280, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![shader]]);
        assert_eq!(e.global::<u32>(SHADER_TABLE_011F9548 + 0x10), 0);
        assert_eq!(e.global::<u32>(TEXTURE_011F9510), 0);
        assert_eq!(e.global::<u32>(ACCUMULATOR), 0);
        assert_eq!(e.mem.u32(texture + 4), 1);
        assert_eq!(e.global::<u32>(SHADER_FLAGS_011F91D8), 0x2f);
        assert_eq!(e.global::<u32>(0x011f_91f8), 0x00b4_f2e0);
        assert_eq!(e.global::<f32>(0x011f_9184), 1.0);
        assert_eq!(e.global::<f32>(0x011f_9190), 1.0);
        assert_eq!(e.global::<u32>(OBJECT_011F91AC), 0);
        assert_eq!(e.global::<u8>(0x011a_d80c), 1);
        let sines = calls_to(&e, CRT_SIN);
        assert_eq!(sines.len(), 2);
    }

    #[test]
    fn fn_00b54630_shuts_everything_down() {
        let mut e = engine();
        let simple = [
            0x00b6_2a20u32,
            0x00b5_9940,
            0x00ba_50d0,
            0x00bc_0940,
            0x00bc_4610,
            0x00b9_7690,
            0x00b6_f0b0,
            FORMAT_00406D00,
            STOP_RENDER_THREADS,
            0x00a2_9680,
            0x00ec_5fce,
            0x0000_7004,
        ];
        stub(&mut e, &simple);
        let leaking = object(&mut e, 3);
        e.set_global(SHADER_TABLE_011F9548 + 4, leaking);
        e.set_global(OBJECT_011F91AC, 0x4400u32);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, 0x8008);
        e.mem.map(0x8000, 0x10);
        e.mem.set_u32(0x8000, 2);
        e.set_global(HOLDER_011F94BC, holder);
        let renderer = object(&mut e, 1);
        e.set_global(GLOBAL_011F9508, renderer);
        let accumulators = e.mem.alloc(0xc) + 4;
        e.mem.set_u32(accumulators - 4, 2);
        let accumulator = object(&mut e, 1);
        e.mem.set_u32(accumulators + 4, accumulator);
        e.set_global(ACCUMULATOR_ARRAY_011F9170, accumulators);
        e.set_global(ACCUMULATOR_COUNT_011F916C, 2u32);
        e.call_log = Some(vec![]);
        e.call(0x00b5_4630, &args![1u32]);
        let leak = calls_to(&e, FORMAT_00406D00);
        assert_eq!(leak.len(), 1);
        assert_eq!(leak[0][1..], [0xff, SHADER_LEAKING_010AD6E8, 1, 3]);
        assert_eq!(e.global::<u32>(SHADER_TABLE_011F9548 + 4), 0);
        assert_eq!(e.mem.u32(leaking + 4), 2);
        assert_eq!(calls_to(&e, 0x00b9_7690), vec![vec![0x4400]]);
        assert_eq!(e.global::<u32>(OBJECT_011F91AC), 0);
        assert_eq!(e.mem.u32(0x8000), 1);
        assert_eq!(e.global::<u32>(HOLDER_011F94BC), 0);
        assert_eq!(
            calls_to(&e, STOP_RENDER_THREADS),
            vec![vec![RENDER_THREADS_01200088]]
        );
        assert_eq!(e.global::<u32>(GLOBAL_011F9508), 0);
        assert_eq!(
            calls_to(&e, 0x0000_7004),
            vec![vec![renderer], vec![accumulator]]
        );
        assert_eq!(
            calls_to(&e, 0x00ec_5fce),
            vec![vec![accumulators, 4, 2, 0x0045_cec0]]
        );
        assert!(calls_to(&e, MEMORY_DEALLOCATE).contains(&vec![MEMORY_MANAGER, accumulators - 4]));

        // Without the flag only the always-part runs.
        e.set_global(ACCUMULATOR_ARRAY_011F9170, 0u32);
        e.set_global(ACCUMULATOR_COUNT_011F916C, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x00b5_4630, &args![0u32]);
        assert!(calls_to(&e, 0x00b6_2a20).is_empty());
        assert!(calls_to(&e, 0x00ec_5fce).is_empty());
    }

    #[test]
    fn accumulator_setters_hold_references() {
        let mut e = engine();
        stub(&mut e, &[0x0000_7004]);
        let first = object(&mut e, 0);
        let second = object(&mut e, 0);
        e.call(0x00b5_4ac0, &args![first]);
        assert_eq!(e.global::<u32>(ACCUMULATOR), first);
        assert_eq!(e.mem.u32(first + 4), 1);
        e.call_log = Some(vec![]);
        e.call(0x00b5_4ac0, &args![second]);
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![first]]);
        e.call(0x00b5_4b10, &args![second]);
        assert_eq!(e.global::<u32>(SECOND_ACCUMULATOR_011F95F0), second);
        assert_eq!(e.mem.u32(second + 4), 2);
        e.call(0x00b5_4b10, &args![0u32]);
        assert_eq!(e.global::<u32>(SECOND_ACCUMULATOR_011F95F0), 0);
        assert_eq!(e.mem.u32(second + 4), 1);
    }

    /// A node (vtable +0xC answers it) with the given children.
    fn node(e: &mut Engine, children: &[u32]) -> u32 {
        let node = object(e, 1);
        let list = e.mem.alloc(4 * children.len() as u32 + 4);
        for (index, &child) in children.iter().enumerate() {
            e.mem.set_u32(list + 4 * index as u32, child);
        }
        e.mem.set_u32(node + 0xa0, list);
        e.mem.set_u16(node + 0xa6, children.len() as u16);
        node
    }

    /// The vtable slots +0xC (node) and +0x18 (geometry): answers from
    /// marker words at +0xF0 (node) and +0xF4 (geometry).
    fn kind_slots(e: &mut Engine) {
        e.register(0x0000_700c, |e, a| {
            eax(if e.mem.u32(a[0] + 0xf0) != 0 { a[0] } else { 0 })
        });
        e.register(0x0000_7018, |e, a| eax(e.mem.u32(a[0] + 0xf4)));
    }

    fn geometry(e: &mut Engine) -> u32 {
        let geometry = object(e, 1);
        e.mem.set_u32(geometry + 0xf4, 1);
        geometry
    }

    #[test]
    fn set_object_type_walks_the_tree() {
        let mut e = engine();
        kind_slots(&mut e);
        e.register(GET_PROPERTY, |_, _| eax(0));
        let leaf = geometry(&mut e);
        let root = node(&mut e, &[0, leaf]);
        e.mem.set_u32(root + 0xf0, 1);
        e.register(0x0000_9000, |_, _| eax(4));
        e.set_global(OBJECT_TYPE_FUNCTION_011F91EC, 0x0000_9000u32);
        e.call_log = Some(vec![]);
        e.call(0x00b5_5b80, &args![root]);
        assert_eq!(calls_to(&e, 0x0000_9000), vec![vec![root]]);
        assert_eq!(calls_to(&e, GET_PROPERTY), vec![vec![leaf, 3]]);
        e.call_log = Some(vec![]);
        e.call(0x00b5_5b80, &args![0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // Without the function the type is 0; neither node nor geometry: nothing.
        e.set_global(OBJECT_TYPE_FUNCTION_011F91EC, 0u32);
        let other = object(&mut e, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b5_5b80, &args![other]);
        e.call(0x00b5_5360, &args![other, 9u32]);
        assert!(calls_to(&e, GET_PROPERTY).is_empty());
    }

    #[test]
    fn fn_00b55480_sets_the_word_on_flagged_geometry() {
        let mut e = engine();
        kind_slots(&mut e);
        let first = e.mem.alloc(0x40);
        let shader_property = e.mem.alloc(0x40);
        e.mem.set_u32(shader_property + 0x20, 0x2000_0000);
        e.register_double(GET_PROPERTY, move |_, a| {
            eax(if a[1] == 2 { first } else { shader_property })
        });
        let leaf = geometry(&mut e);
        let root = node(&mut e, &[leaf, 0]);
        e.mem.set_u32(root + 0xf0, 1);
        e.call(0x00b5_5480, &args![root, 0x1234u32]);
        assert_eq!(e.mem.u32(first + 0x34), 0x1234);
        e.mem.set_u32(shader_property + 0x20, 0);
        e.call(0x00b5_5480, &args![leaf, 0x99u32]);
        assert_eq!(e.mem.u32(first + 0x34), 0x1234);
    }

    #[test]
    fn fn_00b55a10_makes_and_runs_the_debug_shader() {
        let mut e = engine();
        e.register(DEBUG_SHADER_CREATE, |e, _| {
            let context = e.tls() + TLS_CONTEXT;
            assert_eq!(e.mem.u32(context), 5);
            eax(0x4400)
        });
        e.register(NI_POINTER_ASSIGN_00A5F860, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        stub(&mut e, &[DEBUG_SHADER_00BD1CF0]);
        e.call_log = Some(vec![]);
        e.call(0x00b5_5a10, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.set_global(GLOBAL_011F9508, 1u32);
        let context = e.tls() + TLS_CONTEXT;
        e.mem.set_u32(context, 3);
        e.call(0x00b5_5a10, &args![]);
        assert_eq!(e.mem.u32(context), 3);
        assert_eq!(e.global::<u32>(DEBUG_SHADER_011F956C), 0x4400);
        e.call(0x00b5_5a10, &args![]);
        assert_eq!(calls_to(&e, DEBUG_SHADER_CREATE).len(), 1);
        assert_eq!(
            calls_to(&e, DEBUG_SHADER_00BD1CF0),
            vec![vec![0x4400, 0], vec![0x4400, 0]]
        );
        // A failed creation leaves the shader unused.
        e.set_global(DEBUG_SHADER_011F956C, 0u32);
        e.register(DEBUG_SHADER_CREATE, |_, _| eax(0));
        e.call_log = Some(vec![]);
        e.call(0x00b5_5a10, &args![]);
        assert!(calls_to(&e, DEBUG_SHADER_00BD1CF0).is_empty());
    }

    #[test]
    fn fn_00b55ac0_queries_then_resets_eight_stages() {
        let mut e = engine();
        e.register(QUERY_00B97900, |e, a| {
            assert_eq!((e.mem.u32(a[2]), e.mem.u32(a[3])), (7, 8));
            Ret::default()
        });
        stub(&mut e, &[0x0000_70dc]);
        let state = object(&mut e, 1);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x8b8, state);
        e.set_global(GLOBAL_011F9508, renderer);
        e.set_global(OBJECT_011F91AC, 0x4400u32);
        e.call_log = Some(vec![]);
        e.call(0x00b5_5ac0, &args![2u32, 7u32, 8u32]);
        let query = calls_to(&e, QUERY_00B97900);
        assert_eq!(query[0][..2], [0x4400, 2]);
        let stages: Vec<Vec<u32>> = (0..8).map(|stage| vec![state, stage, 0]).collect();
        assert_eq!(calls_to(&e, 0x0000_70dc), stages);
    }

    /// A shader whose vtable slot +8 answers `class`.
    fn shader(e: &mut Engine, class: u32) -> u32 {
        let slots: Vec<u32> = (0..0x50).map(|slot| 0x0000_8000 + 4 * slot).collect();
        e.put_vtable(0x0f10_0000 + class % 0x100 * 0x400, &slots);
        let shader = e.mem.alloc(0x10);
        e.mem.set_u32(shader, 0x0f10_0000 + class % 0x100 * 0x400);
        e.mem.set_u32(shader + 4, class);
        shader
    }

    fn prepare_engine() -> Engine {
        let mut e = engine();
        e.register(0x0000_8008, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(0x0000_8118, |_, _| eax(0x5555));
        stub(
            &mut e,
            &[
                0x0000_8050,
                REMOVE_PROPERTY,
                ATTACH_PROPERTY,
                FORMAT_00406D00,
            ],
        );
        e.register(0x0000_7030, |_, _| eax(0));
        e.register(0x0000_70b0, |_, _| eax(0));
        e.register(0x0000_7f00, |_, _| Ret::default());
        e
    }

    #[test]
    fn prepare_geometry_sets_the_shader_and_tells_the_renderer() {
        let mut e = prepare_engine();
        let property = object(&mut e, 1);
        e.mem.set_u32(property + 0x58, 3);
        e.register_double(GET_PROPERTY, move |_, _| eax(property));
        let first = shader(&mut e, SHADER_RTTI_01202040);
        e.set_global(GLOBAL_011F9508, 1u32);
        e.set_global(SHADER_TABLE_011F9548 + 12, first);
        let geometry = object(&mut e, 1);
        let properties = e.mem.alloc(0x40);
        e.mem.set_u8(properties + 0x38, 0x80);
        e.mem.set_u32(geometry + 0xb8, properties);
        let renderer = e.mem.alloc(4);
        e.put_vtable(0x0f20_0000, &[0x0000_7f00u32; 0x3c]);
        e.mem.set_u32(renderer, 0x0f20_0000);
        e.set_global(RENDERER_OBJECT_011F4748, renderer);
        e.call_log = Some(vec![]);
        e.call(0x00b5_79e0, &args![geometry, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(geometry + 0xc0), first);
        assert_eq!(calls_to(&e, 0x0000_8050), vec![vec![first, geometry]]);
        assert_eq!(e.mem.u8(properties + 0x38), 0x80 | 0x33 | 4 | 8);
        assert_eq!(
            calls_to(&e, 0x0000_7f00),
            vec![vec![renderer, geometry, 0x12, 4, 0x5555]]
        );
        assert_eq!(
            calls_to(&e, 0x0000_8118),
            vec![vec![first, geometry, property]]
        );
        // `skip` set: no renderer call; a skipping class sets it too.
        e.call_log = Some(vec![]);
        e.call(0x00b5_79e0, &args![geometry, 0u32, 1u32, 1u32]);
        assert!(calls_to(&e, 0x0000_7f00).is_empty());
        let skipping = shader(&mut e, SHADER_RTTI_012021B8);
        e.set_global(SHADER_TABLE_011F9548 + 12, skipping);
        e.mem.set_u8(properties + 0x38, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b5_79e0, &args![geometry, 0u32, 0u32, 0u32]);
        assert!(calls_to(&e, 0x0000_7f00).is_empty());
        assert_eq!(e.mem.u8(properties + 0x38), 0x33);
    }

    #[test]
    fn prepare_geometry_replaces_or_drops_the_property() {
        let mut e = prepare_engine();
        let property = object(&mut e, 1);
        let replacement = object(&mut e, 1);
        e.mem.set_u32(property + 0x58, 0);
        e.register_double(GET_PROPERTY, move |_, _| eax(property));
        e.register_double(0x0000_70b0, move |_, _| eax(replacement));
        e.register(0x0000_708c, |_, _| eax(2));
        stub(&mut e, &[NI_POINTER_ASSIGN_0066B0D0]);
        let geometry = object(&mut e, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b5_79e0, &args![geometry, 0u32, 0u32, 0u32]);
        assert_eq!(
            calls_to(&e, ATTACH_PROPERTY),
            vec![vec![geometry, replacement]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_ASSIGN_0066B0D0),
            vec![vec![geometry + 0x9c + 8, replacement]]
        );
        // The replacement has no shader number: removed again.
        assert_eq!(
            calls_to(&e, REMOVE_PROPERTY),
            vec![vec![geometry, 3], vec![geometry, 3]]
        );
        // No usable property: only the message.
        e.mem.set_u32(property + 0x1c, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x00b5_79e0, &args![geometry, 0u32, 0u32, 0u32]);
        let message = calls_to(&e, FORMAT_00406D00);
        assert_eq!(message.len(), 1);
        assert_eq!(message[0][1..3], [0x3e8, NO_SHADER_PROPERTY_010ADA44]);
        assert!(calls_to(&e, 0x0000_70b0).is_empty());
    }

    #[test]
    fn prepare_object_recurses_into_geometry() {
        let mut e = engine();
        kind_slots(&mut e);
        stub(&mut e, &[UPDATE_PROPERTIES, FORMAT_00406D00]);
        e.register(GET_PROPERTY, |_, _| eax(0));
        let leaf = geometry(&mut e);
        let root = node(&mut e, &[leaf]);
        e.mem.set_u32(root + 0xf0, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b5_7e30, &args![root, 1u32, 0u32]);
        assert_eq!(calls_to(&e, UPDATE_PROPERTIES), vec![vec![root]]);
        // The leaf reached PrepareGeometry (no shader property: the message).
        assert_eq!(calls_to(&e, GET_PROPERTY), vec![vec![leaf, 3]]);
        let other = object(&mut e, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b5_7bd0, &args![other, 0u32, 0u32, 0u32]);
        assert!(calls_to(&e, GET_PROPERTY).is_empty());
    }

    #[test]
    fn clone_material_property_copies_into_a_new_property() {
        let mut e = engine();
        kind_slots(&mut e);
        let property = object(&mut e, 1);
        for (offset, value) in [
            (0x1c, 1u32),
            (0x20, 2),
            (0x24, 3),
            (0x28, 4),
            (0x2c, 5),
            (0x30, 6),
        ] {
            e.mem.set_u32(property + offset, value);
        }
        for (offset, value) in [(0x38, 0.5f32), (0x3c, 0.75), (0x40, 2.0)] {
            e.mem.set_f32(property + offset, value);
        }
        let leaf = object(&mut e, 1);
        e.register_double(GET_PROPERTY, move |_, a| {
            eax(if a[0] == leaf { property } else { 0 })
        });
        e.register(NEW_OBJECT, |e, a| eax(e.mem.alloc(a[0])));
        e.register(MATERIAL_PROPERTY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x44, 10);
            eax(a[0])
        });
        stub(&mut e, &[OBJECT_00654640, ATTACH_PROPERTY]);
        let held = object(&mut e, 1);
        e.register_double(SCOPE_CONSTRUCT_004AD0C0, move |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, held);
            Ret::default()
        });
        stub(&mut e, &[0x0000_7000]);
        let root = node(&mut e, &[leaf]);
        e.mem.set_u32(root + 0xf0, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b5_7c60, &args![root]);
        let attached = calls_to(&e, ATTACH_PROPERTY);
        assert_eq!(attached.len(), 1);
        let copy = attached[0][1];
        assert_eq!(attached[0][0], leaf);
        for (offset, value) in [
            (0x1c, 1u32),
            (0x20, 2),
            (0x24, 3),
            (0x28, 4),
            (0x2c, 5),
            (0x30, 6),
        ] {
            assert_eq!(e.mem.u32(copy + offset), value);
        }
        assert_eq!(e.mem.f32(copy + 0x38), 0.5);
        assert_eq!(e.mem.f32(copy + 0x3c), 0.75);
        assert_eq!(e.mem.f32(copy + 0x40), 2.0);
        assert_eq!(e.mem.u32(copy + 0x44), 15);
        assert_eq!(calls_to(&e, 0x0000_7000), vec![vec![held, 1]]);
        assert_eq!(calls_to(&e, OBJECT_00654640), vec![vec![leaf, property]]);
        // The reference taken is given back.
        assert_eq!(e.mem.u32(property + 4), 1);
        // A shared property (+0xC set) is attached as it is.
        e.mem.set_u32(property + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b5_7c60, &args![leaf]);
        assert_eq!(calls_to(&e, ATTACH_PROPERTY), vec![vec![leaf, property]]);
    }

    /// A triangle-shape geometry (vtable +0x1C answers itself, +0x20 the
    /// strips flag at +0xF8) with `count` vertices, its data at +0xB8.
    fn shape(e: &mut Engine, positions: &[[f32; 3]], coordinates: &[[f32; 2]]) -> (u32, u32) {
        let geometry = object(e, 1);
        e.register(0x0000_701c, |_, a| eax(a[0]));
        e.register(0x0000_7020, |e, a| eax(e.mem.u32(a[0] + 0xf8)));
        let data = object(e, 1);
        let count = positions.len() as u32;
        let vertices = e.mem.alloc(12 * count);
        let normals = e.mem.alloc(12 * count);
        let uv = e.mem.alloc(8 * count);
        for (index, position) in positions.iter().enumerate() {
            let index = index as u32;
            for axis in 0..3u32 {
                e.mem
                    .set_f32(vertices + 12 * index + 4 * axis, position[axis as usize]);
            }
            e.mem.set_f32(normals + 12 * index + 8, 1.0);
            e.mem
                .set_f32(uv + 8 * index, coordinates[index as usize][0]);
            e.mem
                .set_f32(uv + 8 * index + 4, coordinates[index as usize][1]);
        }
        e.mem.set_u16(data + 8, count as u16);
        e.mem.set_u32(data + 0x20, vertices);
        e.mem.set_u32(data + 0x24, normals);
        e.mem.set_u32(data + 0x28, 0x6600);
        e.mem.set_u32(data + 0x2c, uv);
        e.mem.set_u16(data + 0xc, 3);
        e.mem.set_u32(geometry + 0xb8, data);
        (geometry, data)
    }

    fn tangent_engine() -> Engine {
        let mut e = engine();
        e.set_global(FIXED_STRING_NONE, 0x5100u32);
        e.register(FIXED_STRING_MAKE, |_, _| eax(0x5100));
        e.register(NEW_OBJECT, |e, a| eax(e.mem.alloc(a[0])));
        e.register(TANGENT_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0f00_0000);
            eax(a[0])
        });
        e.register(MEMCPY_00EC44D0, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(MEMSET_00EC61C0, |e, a| {
            let bytes = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        stub(
            &mut e,
            &[
                STRNICMP,
                NORMALIZE_004A0C10,
                REPLACE_DATA,
                0x0000_7000,
                REMOVE_EXTRA_DATA,
            ],
        );
        e
    }

    fn floats(e: &Engine, at: u32) -> [f32; 3] {
        [0, 4, 8].map(|offset| e.mem.f32(at + offset))
    }

    #[test]
    fn create_tangent_space_builds_tangents_and_binormals() {
        let mut e = tangent_engine();
        e.register(GET_EXTRA_DATA, |_, _| eax(0));
        let (geometry, data) = shape(
            &mut e,
            &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
        );
        e.mem.set_u16(data + 0x40, 1);
        e.register(0x0000_70ac, |e, a| {
            e.mem.set_u16(a[2], 0);
            e.mem.set_u16(a[3], 1);
            e.mem.set_u16(a[4], 2);
            Ret::default()
        });
        let normals = e.mem.u32(data + 0x24);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_4b60, &args![geometry]).u32(), 0);
        let replaced = calls_to(&e, REPLACE_DATA);
        assert_eq!(replaced.len(), 1);
        let call = &replaced[0];
        assert_eq!(call[..3], [data, 3, e.mem.u32(data + 0x20)]);
        assert_eq!(call[4..], [0x6600, e.mem.u32(data + 0x2c), 1, 0x1000]);
        let output = call[3];
        for vertex in 0..3 {
            assert_eq!(floats(&e, output + 12 * vertex), [0.0, 0.0, 1.0]);
            assert_eq!(floats(&e, output + 36 + 12 * vertex), [0.0, 1.0, 0.0]);
            assert_eq!(floats(&e, output + 72 + 12 * vertex), [1.0, 0.0, 0.0]);
        }
        assert_eq!(calls_to(&e, NORMALIZE_004A0C10).len(), 6);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).contains(&vec![MEMORY_MANAGER, normals]));
        // The made object is deleted.
        assert_eq!(calls_to(&e, 0x0000_7000).len(), 1);
    }

    #[test]
    fn create_tangent_space_reads_strips_with_alternating_winding() {
        let mut e = tangent_engine();
        e.register(GET_EXTRA_DATA, |_, _| eax(0));
        let (geometry, data) = shape(
            &mut e,
            &[
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
            ],
            &[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
        );
        e.mem.set_u32(geometry + 0xf8, 1);
        e.mem.set_u16(data + 0x40, 3);
        let strip = e.mem.alloc(12);
        // 0 1 2 3 3: the third triangle (2, 3, 3) is degenerate.
        for (index, value) in [0u16, 1, 2, 3, 3].iter().enumerate() {
            e.mem.set_u16(strip + 2 * index as u32, *value);
        }
        e.register_double(GET_STRIP_LISTS, move |_, _| eax(strip));
        e.call_log = Some(vec![]);
        e.call(0x00b5_4b60, &args![geometry]);
        let output = calls_to(&e, REPLACE_DATA)[0][3];
        // The flipped second triangle (3, 2, 1) gives vertex 3 the same
        // tangent as the first triangle gives its vertices.
        assert_eq!(floats(&e, output + 96 + 36), [1.0, 0.0, 0.0]);
        assert_eq!(floats(&e, output + 96), [1.0, 0.0, 0.0]);
        // Vertices 1 and 2 sum both triangles.
        assert_eq!(floats(&e, output + 96 + 12), [2.0, 0.0, 0.0]);
        assert!(calls_to(&e, 0x0000_70ac).is_empty());
    }

    #[test]
    fn create_tangent_space_copies_existing_extra_data() {
        let mut e = tangent_engine();
        let extra = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(48);
        for word in 0..12u32 {
            e.mem.set_u32(buffer + 4 * word, word);
        }
        e.mem.set_u32(extra + 0xc, buffer);
        e.register_double(GET_EXTRA_DATA, move |_, _| eax(extra));
        let (geometry, data) = shape(
            &mut e,
            &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            &[[0.0, 0.0], [1.0, 0.0]],
        );
        let _ = data;
        e.call_log = Some(vec![]);
        let object = e.call(0x00b5_4b60, &args![geometry]).u32();
        assert_ne!(object, 0);
        let second = e.mem.u32(object + 0xc);
        let first = e.mem.u32(object + 0x10);
        assert_eq!(e.mem.u32(second), 6);
        assert_eq!(e.mem.u32(first + 4), 1);
        assert_eq!(calls_to(&e, REMOVE_EXTRA_DATA).len(), 1);
        assert_eq!(calls_to(&e, REMOVE_EXTRA_DATA)[0][0], geometry);
        assert!(calls_to(&e, REPLACE_DATA).is_empty());

        // Not a triangle shape: 0, nothing made.
        let mut e = tangent_engine();
        e.register(GET_EXTRA_DATA, |_, _| eax(0));
        let other = object_without_shape(&mut e);
        assert_eq!(e.call(0x00b5_4b60, &args![other]).u32(), 0);
    }

    fn object_without_shape(e: &mut Engine) -> u32 {
        let other = object(e, 1);
        e.register(0x0000_701c, |_, _| eax(0));
        other
    }

    #[test]
    fn create_tangent_space_needs_normals() {
        let mut e = tangent_engine();
        e.register(GET_EXTRA_DATA, |_, _| eax(0));
        stub(&mut e, &[FORMAT_00406D00]);
        let (geometry, data) = shape(&mut e, &[[0.0, 0.0, 0.0]], &[[0.0, 0.0]]);
        e.mem.set_u32(data + 0x24, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_4b60, &args![geometry]).u32(), 0);
        assert_eq!(
            calls_to(&e, FORMAT_00406D00)[0][1..],
            [0xff, NO_NORMALS_MESSAGE_010AD730]
        );
        assert!(calls_to(&e, REPLACE_DATA).is_empty());
        assert!(calls_to(&e, 0x0000_7000).is_empty());
    }
}

#[cfg(test)]
mod n012_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x2000);
        e.call_log = Some(vec![]);
        e
    }

    /// An object whose vtable answers slot `+0x18` (geometry) from its word `+0x10` and slot
    /// `+0xc` (node) from its word `+0x14`.
    fn scene_object(e: &mut Engine, geometry: bool, node: bool) -> u32 {
        let mut slots = [0u32; 8];
        slots[SLOT_IS_NODE as usize / 4] = 0x0f00_e000;
        slots[SLOT_IS_GEOMETRY as usize / 4] = 0x0f00_e010;
        e.put_vtable(0x0f00_e100, &slots);
        e.register(0x0f00_e000, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(0x0f00_e010, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        let object = e.mem.alloc(0xb0);
        e.mem.set_u32(object, 0x0f00_e100);
        e.mem.set_u32(object + 0x10, u32::from(geometry));
        if node {
            e.mem.set_u32(object + 0x14, object);
        }
        object
    }

    fn children(e: &mut Engine, node: u32, items: &[u32]) {
        let buffer = e.mem.alloc(4 * items.len() as u32);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *item);
        }
        e.mem.set_u32(node + 0xa0, buffer);
        e.mem.set_u16(node + 0xa6, items.len() as u16);
    }

    #[test]
    fn fn_00b4f2e0_jumps_to_get_file() {
        let mut e = rig();
        e.register(NI_FILE_GET_FILE, |_, a| (a[0] + a[1] + a[2]).into_ret());
        assert_eq!(
            e.call(0x00b4_f2e0, &args![0x100u32, 2u32, 3u32]).u32(),
            0x105
        );
        assert_eq!(calls(&e, NI_FILE_GET_FILE), vec![vec![0x100, 2, 3]]);
    }

    #[test]
    fn fn_00b4f9a0_calls_each_shader() {
        let mut e = rig();
        let mut slots = [0u32; 0x4a];
        slots[0x124 / 4] = 0x0f00_0124;
        e.put_vtable(0x0f00_1000, &slots);
        e.register(0x0f00_0124, |_, _| Ret::default());
        let first = e.mem.alloc(8);
        let last = e.mem.alloc(8);
        e.mem.set_u32(first, 0x0f00_1000);
        e.mem.set_u32(last, 0x0f00_1000);
        e.set_global(SHADER_TABLE_011F9548, first);
        e.set_global(SHADER_TABLE_011F9548 + 4 * 0x22, last);
        e.call(0x00b4_f9a0, &args![]);
        assert_eq!(calls(&e, 0x0f00_0124), vec![vec![first], vec![last]]);
    }

    #[test]
    fn get_pass_name_answers_the_arm_or_the_unknown_string() {
        let mut e = rig();
        e.register(SPRINTF_S_00406D00, |_, _| Ret::default());
        assert_eq!(e.call(0x00b4_f9d0, &args![1u32]).u32(), 0x010a_d6c8);
        assert_eq!(e.call(0x00b4_f9d0, &args![0x157u32]).u32(), 0x010a_b824);
        assert_eq!(e.call(0x00b4_f9d0, &args![0x2f5u32]).u32(), 0x010a_9490);
        assert!(calls(&e, SPRINTF_S_00406D00).is_empty());
        for pass in [0u32, 0x158, 0x17e, 0x237, 0x25b, 0x2f6, 0xffff_ffff] {
            assert_eq!(e.call(0x00b4_f9d0, &args![pass]).u32(), 0x010a_93f4);
            let last = calls(&e, SPRINTF_S_00406D00).pop().unwrap();
            assert_eq!(last[1..], [10, PASS_NAME_FORMAT_010A9488, pass]);
        }
    }

    #[test]
    fn fn_00b540d0_lowers_both_counters() {
        let mut e = rig();
        e.map(0x011f_f000, 0x1000);
        e.register(FUNCTION_00B984F0, |_, _| Ret::default());
        e.set_global(COUNTER_011FFA10, 5u32);
        e.set_global(COUNTER_011FFA0C, 0u32);
        e.call(0x00b5_40d0, &args![]);
        assert_eq!(e.global::<u32>(COUNTER_011FFA10), 4);
        assert_eq!(e.global::<u32>(COUNTER_011FFA0C), 0xffff_ffff);
        assert_eq!(calls(&e, FUNCTION_00B984F0), vec![vec![0, 0]]);
    }

    #[test]
    fn set_emittance_recurse_colours_flagged_geometry() {
        let mut e = rig();
        e.register(GET_PROPERTY, |e, a| {
            e.mem.u32(a[0] + 0x20 + 4 * (a[1] - 2)).into_ret()
        });
        let material = e.mem.alloc(0x50);
        e.mem.set_u32(material + 0x44, 7);
        let flagged = e.mem.alloc(0x60);
        e.mem.set_u32(flagged + 0x20, 0x2000_0000);
        let plain = e.mem.alloc(0x60);
        let lit = scene_object(&mut e, true, false);
        e.mem.set_u32(lit + 0x20, material);
        e.mem.set_u32(lit + 0x24, flagged);
        let unlit = scene_object(&mut e, true, false);
        let other = e.mem.alloc(0x50);
        e.mem.set_u32(unlit + 0x20, other);
        e.mem.set_u32(unlit + 0x24, plain);
        let node = scene_object(&mut e, false, true);
        children(&mut e, node, &[lit, 0, unlit]);
        let color = e.mem.alloc(12);
        e.mem.set_f32(color, 0.25);
        e.mem.set_f32(color + 4, 0.5);
        e.mem.set_f32(color + 8, 0.75);
        e.call(0x00b5_53d0, &args![node, color]);
        assert_eq!(e.mem.f32(material + 0x28), 0.25);
        assert_eq!(e.mem.f32(material + 0x2c), 0.5);
        assert_eq!(e.mem.f32(material + 0x30), 0.75);
        assert_eq!(e.mem.u32(material + 0x44), 8);
        assert_eq!(e.mem.u32(other + 0x28), 0);
        assert_eq!(calls(&e, GET_PROPERTY).len(), 4);
        e.call(0x00b5_53d0, &args![0u32, color]);
        assert_eq!(calls(&e, GET_PROPERTY).len(), 4);
    }

    #[test]
    fn fn_00b557d0_reaches_shaders_of_the_record_chain() {
        let mut e = rig();
        e.map(0x0126_f000, 0x1000);
        let mut slots = [0u32; 0x49];
        slots[2] = 0x0f00_0008;
        slots[0x120 / 4] = 0x0f00_0120;
        e.put_vtable(0x0f00_1000, &slots);
        e.register(0x0f00_0008, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0f00_0120, |_, _| Ret::default());
        e.register(IMAGE_SPACE_MANAGER_00B8B3B0, |_, _| Ret::default());
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + 4, RTTI_0126F554);
        let unrelated = e.mem.alloc(8);
        let shader = |e: &mut Engine, rtti: u32| {
            let s = e.mem.alloc(8);
            e.mem.set_u32(s, 0x0f00_1000);
            e.mem.set_u32(s + 4, rtti);
            s
        };
        let hit = shader(&mut e, derived);
        let miss = shader(&mut e, unrelated);
        let direct = shader(&mut e, RTTI_0126F554);
        e.set_global(SHADER_TABLE_011F9548 + 4, hit);
        e.set_global(SHADER_TABLE_011F9548 + 8, miss);
        e.set_global(SHADER_TABLE_011F9548 + 0x40, direct);
        e.set_global(OBJECT_011F91AC, 0x4444u32);
        e.call(0x00b5_57d0, &args![]);
        assert_eq!(calls(&e, 0x0f00_0120), vec![vec![hit], vec![direct]]);
        assert_eq!(calls(&e, IMAGE_SPACE_MANAGER_00B8B3B0), vec![vec![0x4444]]);
    }

    #[test]
    fn forced_darkness_sets_lit_types_and_walks_children() {
        let mut e = rig();
        e.register(GET_PROPERTY, |e, a| {
            assert_eq!(a[1], 3);
            e.mem.u32(a[0] + 0x24).into_ret()
        });
        let property = |e: &mut Engine, kind: u32| {
            let p = e.mem.alloc(0x80);
            e.mem.set_u32(p + 0x1c, kind);
            p
        };
        let first = property(&mut e, 1);
        let twelfth = property(&mut e, 12);
        let zero = property(&mut e, 0);
        let thirteenth = property(&mut e, 13);
        let leaf = |e: &mut Engine, p: u32| {
            let o = scene_object(e, true, false);
            e.mem.set_u32(o + 0x24, p);
            o
        };
        let a = leaf(&mut e, first);
        let b = leaf(&mut e, zero);
        let c = leaf(&mut e, thirteenth);
        let node = scene_object(&mut e, false, true);
        e.mem.set_u32(node + 0x24, twelfth);
        children(&mut e, node, &[a, 0, b, c]);
        e.call(0x00b5_5b00, &args![node, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(first + 0x6c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(twelfth + 0x6c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(zero + 0x6c), 0);
        assert_eq!(e.mem.u32(thirteenth + 0x6c), 0);
        assert_eq!(calls(&e, GET_PROPERTY).len(), 4);
    }

    #[test]
    fn fn_00b578a0_rebuilds_the_two_objects() {
        let mut e = rig();
        for addr in [
            RESET_00BA50D0,
            RESET_00BC0940,
            RESET_00BC4610,
            DESTRUCTOR_00B97690,
            DESTRUCTOR_00B6F0B0,
            METHOD_00B6E450,
            MEMORY_DEALLOCATE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(CONSTRUCTOR_00B6DD80, |_, a| a[0].into_ret());
        e.register(CONSTRUCTOR_00B97A80, |_, a| a[0].into_ret());
        e.set_global(SET_UP_DONE_011F91C6, 1u8);
        e.set_global(OBJECT_011F91AC, 0x5000u32);
        e.set_global(OBJECT_011F91A8, 0x6000u32);
        e.set_global(GLOBAL_011F91E0, 9u32);
        e.set_global(GLOBAL_011F9508, 0x7000u32);
        e.call(0x00b5_78a0, &args![]);
        assert_eq!(calls(&e, DESTRUCTOR_00B97690), vec![vec![0x5000]]);
        assert_eq!(calls(&e, DESTRUCTOR_00B6F0B0), vec![vec![0x6000]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x5000], vec![MEMORY_MANAGER, 0x6000]]
        );
        let small = e.global::<u32>(OBJECT_011F91A8);
        let large = e.global::<u32>(OBJECT_011F91AC);
        assert_eq!(calls(&e, CONSTRUCTOR_00B6DD80), vec![vec![small]]);
        assert_eq!(calls(&e, CONSTRUCTOR_00B97A80), vec![vec![large]]);
        assert_eq!(calls(&e, METHOD_00B6E450), vec![vec![small, 0x7000]]);
        assert_eq!(e.global::<u32>(GLOBAL_011F91E0), 0);
        assert_eq!(calls(&e, RESET_00BC4610).len(), 1);
        // Without the word at 011f9508 and with failing allocations: both stay null.
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.set_global(GLOBAL_011F9508, 0u32);
        e.set_global(OBJECT_011F91AC, 0u32);
        e.call(0x00b5_78a0, &args![]);
        assert_eq!(e.global::<u32>(OBJECT_011F91A8), 0);
        assert_eq!(e.global::<u32>(OBJECT_011F91AC), 0);
        assert_eq!(calls(&e, CONSTRUCTOR_00B97A80).len(), 1);
        assert_eq!(calls(&e, METHOD_00B6E450).last(), Some(&vec![0, 0]));
    }
}
