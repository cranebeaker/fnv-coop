//! `(unplaced)/run_00b79de0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The shader's constructor `00b79b00` (unnamed, `thiscall`, `RET 0x10`: four declarations).
const SHADER_CONSTRUCT_00B79B00: u32 = 0x00b7_9b00;

/// The entries (vtable slot +0x8c, five words each) of the first and fourth declarations
/// (six) and of the second (eight).
const SIX_ENTRIES: [[u32; 5]; 6] = [
    [0, 0, 0, 2, 0],
    [1, 3, 3, 2, 0],
    [2, 5, 4, 3, 0],
    [3, 7, 5, 1, 0],
    [4, 0xe, 0xd, 2, 0],
    [5, 0xf, 0xe, 2, 0],
];
const EIGHT_ENTRIES: [[u32; 5]; 8] = [
    [0, 0, 0, 2, 0],
    [1, 1, 1, 3, 0],
    [2, 2, 2, 4, 0],
    [3, 3, 3, 2, 0],
    [4, 5, 4, 3, 0],
    [5, 7, 5, 1, 0],
    [6, 0xe, 0xd, 2, 0],
    [7, 0xf, 0xe, 2, 0],
];

// Translated from 00b7a380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b79de0`): the creator `GetShader` (`00b55560`) calls for
/// shader 1. No arguments. Makes four declarations (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508` read each time, 1 stream each: 6, 8, 3 and 8 entries), then
/// through vtable slot +0x8c (five words each): the first gets (0, 0, 0, 2, 0), (1, 3, 3, 2,
/// 0), (2, 5, 4, 3, 0), (3, 7, 5, 1, 0), (4, 0xe, 0xd, 2, 0), (5, 0xf, 0xe, 2, 0) and ends
/// (slot +0xb4); the second (0, 0, 0, 2, 0), (1, 1, 1, 3, 0), (2, 2, 2, 4, 0), (3, 3, 3, 2,
/// 0), (4, 5, 4, 3, 0), (5, 7, 5, 1, 0), (6, 0xe, 0xd, 2, 0), (7, 0xf, 0xe, 2, 0) and ends; the
/// third (0, 0, 0, 2, 0), (1, 3, 5, 1, 0), slot +0x94 with (0, 2, 5, 0x80000000, 0, 5, 1, 0)
/// and ends; the fourth the first's six entries, slot +0x94 with (0, 6, 8, 0x80000000, 3, 5,
/// 1, 0) and with (0, 7, 9, 0x80000001, 3, 5, 2, 0) and ends. Allocates 0x8c bytes and runs
/// the constructor `00b79b00` with the four (a null allocation is used as it is), calls the
/// shader's slots +0x4c, +0x144, +0x150, +0x11c, then +0xc0 with the first declaration, and
/// answers the shader.
// Translation incomplete: 00b7a380 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00b7a380(e: &mut Engine) -> Ptr {
    let mut made = [0u32; 4];
    for (slot, count) in made.iter_mut().zip([6u32, 8, 3, 8]) {
        let renderer = e.global::<u32>(RENDERER_011F9508);
        *slot = e
            .call(DECLARATION_CREATE, &args![renderer, count, 1u32])
            .u32();
    }
    let [first, second, third, fourth] = made;
    for entry in &SIX_ENTRIES {
        e.vcall(first, 0x8c, entry);
    }
    e.vcall(first, 0xb4, &args![]);
    for entry in &EIGHT_ENTRIES {
        e.vcall(second, 0x8c, entry);
    }
    e.vcall(second, 0xb4, &args![]);
    e.vcall(third, 0x8c, &args![0u32, 0u32, 0u32, 2u32, 0u32]);
    e.vcall(third, 0x8c, &args![1u32, 3u32, 5u32, 1u32, 0u32]);
    e.vcall(
        third,
        0x94,
        &args![0u32, 2u32, 5u32, 0x8000_0000u32, 0u32, 5u32, 1u32, 0u32],
    );
    e.vcall(third, 0xb4, &args![]);
    for entry in &SIX_ENTRIES {
        e.vcall(fourth, 0x8c, entry);
    }
    e.vcall(
        fourth,
        0x94,
        &args![0u32, 6u32, 8u32, 0x8000_0000u32, 3u32, 5u32, 1u32, 0u32],
    );
    e.vcall(
        fourth,
        0x94,
        &args![0u32, 7u32, 9u32, 0x8000_0001u32, 3u32, 5u32, 2u32, 0u32],
    );
    e.vcall(fourth, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x8cu32]).u32();
    let shader = if block != 0 {
        e.call(
            SHADER_CONSTRUCT_00B79B00,
            &args![block, first, second, third, fourth],
        )
        .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x144, 0x150, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![first]);
    Ptr::new(shader)
}

/// The record `00b79dd0` answers the address of (`011fed88`).
const RECORD_011FED88: u32 = 0x011f_ed88;

// Translated from 00b79dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its
/// bytes): the slot `+8` of the vtable `010af2f8` (the slot `GetRTTI` has in
/// Gamebryo classes; the class has no RTTI locator, so this is unconfirmed):
/// the address `011fed88`.
pub fn fn_00b79dd0(_e: &mut Engine) -> Ptr {
    Ptr::new(RECORD_011FED88)
}

// Translated from 00b7a6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (disassembled from its bytes; a vtable slot
/// of three classes): `thiscall`, `RET 8` (two objects). Answers `this+0x70`
/// when the first object's `+0xbc` is set; else `this+0x74` when the second
/// object has bit 0x1000 at `+0x20`, `this+0x78` when it has 0x4000, else
/// `this+0x6c`. A null second object is read as it is (the exe reads
/// `+0x20` of it).
pub fn fn_00b7a6f0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let t = this.addr();
    if e.mem.u32(first + 0xbc) != 0 {
        return e.mem.u32(t + 0x70);
    }
    if second != 0 && e.mem.u32(second + 0x20) & 0x1000 != 0 {
        return e.mem.u32(t + 0x74);
    }
    if e.mem.u32(second.wrapping_add(0x20)) & 0x4000 != 0 {
        return e.mem.u32(t + 0x78);
    }
    e.mem.u32(t + 0x6c)
}

/// The shader slot table (`011fdff8`, 0x2f7 words), the pointer the passes
/// hand out (`011fdd88`), the shader-model word (`011f91c0`) and `00e804d0`
/// (`cdecl`: a slot's address).
const SHADERS: u32 = 0x011f_dff8;
const SHADER_COUNT: u32 = 0x2f7;
const PASS_POINTER_011FDD88: u32 = 0x011f_dd88;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
const PREPARE_SLOT_00E804D0: u32 = 0x00e8_04d0;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// The inline `NiPointer` release of `object` (when not null).
fn release_object(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// The set-up passes `00b7a730` runs on the shader (`thiscall`, no stack
/// words), always and by shader model.
const SET_UP_ALWAYS: [u32; 11] = [
    0x00be_b070,
    0x00be_b830,
    0x00be_22b0,
    0x00be_39e0,
    0x00be_7420,
    0x00be_78e0,
    0x00be_7f40,
    0x00be_8960,
    0x00be_8fb0,
    0x00be_9920,
    0x00be_ac70,
];
const SET_UP_MODEL_2: [u32; 13] = [
    0x00bf_66c0,
    0x00be_bd20,
    0x00bf_0720,
    0x00bf_8f70,
    0x00bf_96b0,
    0x00bf_bf60,
    0x00bf_c860,
    0x00c0_1020,
    0x00c0_17c0,
    0x00c0_1c20,
    0x00c0_1f60,
    0x00c0_0be0,
    0x00c0_2340,
];
const SET_UP_MODEL_1: u32 = 0x00be_a040;

// Translated from 00b7a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (disassembled from its bytes): `thiscall`,
/// no stack words. For each of the 0x2f7 shader slots (`011fdff8`):
/// `00e804d0` with the slot's address, then the slot's object's pointer at
/// `+0x5c` is assigned the one in `011fdd88` (the old one released, the new
/// one counted). Then the set-up passes on `this`: eleven always, and with a
/// shader model word (`011f91c0`) of 2 or more thirteen more, else one.
/// Answers true.
pub fn fn_00b7a730(e: &mut Engine, this: Ptr) -> bool {
    for index in 0..SHADER_COUNT {
        let slot = SHADERS + 4 * index;
        e.call(PREPARE_SLOT_00E804D0, &args![slot]);
        let object = e.mem.u32(slot);
        let pointer = e.global::<u32>(PASS_POINTER_011FDD88);
        let old = e.mem.u32(object + 0x5c);
        if old != pointer {
            release_object(e, old);
            e.mem.set_u32(object + 0x5c, pointer);
            if pointer != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![pointer.wrapping_add(4)]);
            }
        }
    }
    for pass in SET_UP_ALWAYS {
        e.call(pass, &args![this]);
    }
    if (e.global::<u32>(SHADER_MODEL_011F91C0) as i32) < 2 {
        e.call(SET_UP_MODEL_1, &args![this]);
    } else {
        for pass in SET_UP_MODEL_2 {
            e.call(pass, &args![this]);
        }
    }
    true
}

/// The two pass masks (`011fb4c0` and `011fa8e0`, a word per pass) and the
/// two switch lists (`011febd8..011fec30` and `011fec30..011fec88`, objects
/// with a byte at `+8`).
const MASKS_FIRST: u32 = 0x011f_b4c0;
const MASKS_SECOND: u32 = 0x011f_a8e0;

/// Turns off each switch in `[from, to)` that is on and whose bit (starting at
/// bit 1, two switches per step) is set in the mask word at `mask` (read for
/// each one).
fn apply_mask(e: &mut Engine, mask: u32, from: u32, to: u32) {
    let mut bit: u32 = 1;
    let mut at = from + 4;
    while at < to {
        let first = e.mem.u32(at - 4);
        if first != 0 && e.mem.u8(first + 8) != 0 {
            let on = e.mem.u32(mask) & bit.rotate_left(1) == 0;
            e.mem.set_u8(first + 8, u8::from(on));
        }
        let second = e.mem.u32(at);
        bit = bit.rotate_left(1);
        if second != 0 && e.mem.u8(second + 8) != 0 {
            let on = e.mem.u32(mask) & bit.rotate_left(1) == 0;
            e.mem.set_u8(second + 8, u8::from(on));
        }
        at += 8;
        bit = bit.rotate_left(1);
    }
}

// Translated from 00b7ac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (`thiscall`, `RET 4`: a pass index). With a
/// first-mask word for the pass (`011fb4c0`), the switches of the first list
/// are set from it and the object at `+0x34` runs its slot `+0x24`; with a
/// second-mask word (`011fa8e0`), the second list from that and the object
/// at `+0x30`'s slot `+0x24`.
pub fn fn_00b7ac20(e: &mut Engine, this: Ptr, pass: u32) {
    let first = MASKS_FIRST.wrapping_add(pass.wrapping_mul(4));
    if e.mem.u32(first) != 0 {
        apply_mask(e, first, 0x011f_ebd8, 0x011f_ec30);
        let object = e.mem.u32(this.addr() + 0x34);
        e.vcall(object, 0x24, &args![]);
    }
    let second = MASKS_SECOND.wrapping_add(pass.wrapping_mul(4));
    if e.mem.u32(second) != 0 {
        apply_mask(e, second, 0x011f_ec30, 0x011f_ec88);
        let object = e.mem.u32(this.addr() + 0x30);
        e.vcall(object, 0x24, &args![]);
    }
}

/// The renderer's `00e6fe30` (`thiscall` on `011f9508`, `RET 0x14`), the
/// object whose `+0x5c` the bone constants go to (`0126f74c`), the switch
/// object at `011fec00` and the constant name `"BoneMatrix3"` (`010a1cac`).
const RENDERER_SET_00E6FE30: u32 = 0x00e6_fe30;
const DEVICE_HOLDER_0126F74C: u32 = 0x0126_f74c;
const SWITCH_011FEC00: u32 = 0x011f_ec00;
const BONE_MATRIX_NAME: u32 = 0x010a_1cac;

// Translated from 00b7a2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (`thiscall`, `RET 0x20`: eight words; the
/// second, third and seventh are read). Runs only when (a third word, no
/// `+0x38`, and the switch `011fec00` present and on) or when `+0x34` equals
/// `+0x80`: the renderer's `00e6fe30` with the second and seventh words, 0,
/// 3 and 1; then the bone constants go to the object at `0126f74c`'s `+0x5c`
/// through its slot `+0x78` with the second word's `+0x28`, the third's
/// 16-bit `+0x20`, 3 and the third's `+4`, headed by the `"BoneMatrix3"`
/// handle (slot `+0x70` of `+0x80`) when `+0x34` equals `+0x80`, else by the
/// switch object. Answers 0.
#[allow(clippy::too_many_arguments)] // the exe's eight stack words
pub fn fn_00b7a2c0(
    e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    geometry: u32,
    bones: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    pass: u32,
    _unused_7: u32,
) -> u32 {
    let t = this.addr();
    let switch = e.global::<u32>(SWITCH_011FEC00);
    let switched =
        bones != 0 && e.mem.u32(t + 0x38) == 0 && switch != 0 && e.mem.u8(switch + 8) != 0;
    if !switched && e.mem.u32(t + 0x34) != e.mem.u32(t + 0x80) {
        return 0;
    }
    let renderer = e.global::<u32>(RENDERER_011F9508);
    e.call(
        RENDERER_SET_00E6FE30,
        &args![renderer, geometry, pass, 0u32, 3u32, 1u32],
    );
    let shader_bones = e.mem.u32(t + 0x80);
    let data = e.mem.u32(geometry + 0x28);
    let target = e.mem.u32(e.global::<u32>(DEVICE_HOLDER_0126F74C) + 0x5c);
    let count = u32::from(e.mem.u16(bones + 0x20));
    let matrices = e.mem.u32(bones + 4);
    let head = if e.mem.u32(t + 0x34) == shader_bones {
        e.vcall(shader_bones, 0x70, &args![BONE_MATRIX_NAME]).u32()
    } else {
        e.global::<u32>(SWITCH_011FEC00)
    };
    e.vcall(target, 0x78, &args![head, data, count, 3u32, matrices]);
    0
}

/// `00be0870` (`thiscall`: the shader set-up that calls the base initialization `00e80570`
/// and three virtual slots), the shader release `00e80190` (`thiscall`) and CRT `memset`
/// (`00ec61c0`, `cdecl`).
const SHADERS_RELEASE_00BE0870: u32 = 0x00be_0870;
const SHADER_DELETE_00E80190: u32 = 0x00e8_0190;
const MEMSET: u32 = 0x00ec_61c0;
/// The pointer tables both take-downs empty: `011fdd88..011fdff8`,
/// `011fda48..011fdd88`, then (after the shader slots) `011febd8..011fec30`
/// and `011fec30..011fec88`.
const POINTER_TABLES_FIRST: [(u32, u32); 2] =
    [(0x011f_dd88, 0x011f_dff8), (0x011f_da48, 0x011f_dd88)];
const POINTER_TABLES_LAST: [(u32, u32); 2] =
    [(0x011f_ebd8, 0x011f_ec30), (0x011f_ec30, 0x011f_ec88)];

/// Releases and clears every pointer in `[from, to)`.
fn release_table(e: &mut Engine, from: u32, to: u32) {
    let mut at = from;
    while at < to {
        let object = e.mem.u32(at);
        if object != 0 {
            release_object(e, object);
            e.mem.set_u32(at, 0);
        }
        at += 4;
    }
}

/// Gives up every shader slot (`011fdff8..011febd4`): its count at `+0x64`
/// goes down and the shader is deleted (`00e80190`) at zero; the slot is
/// cleared.
fn release_shader_slots(e: &mut Engine) {
    let mut at = SHADERS;
    while at < 0x011f_ebd4 {
        let shader = e.mem.u32(at);
        if shader != 0 {
            let count = e.mem.u32(shader + 0x64).wrapping_sub(1);
            e.mem.set_u32(shader + 0x64, count);
            if count == 0 {
                e.call(SHADER_DELETE_00E80190, &args![shader]);
            }
            e.mem.set_u32(at, 0);
        }
        at += 4;
    }
}

/// The tables both take-downs empty, in the exe's order.
fn release_static_tables(e: &mut Engine) {
    for (from, to) in POINTER_TABLES_FIRST {
        release_table(e, from, to);
    }
    release_shader_slots(e);
    for (from, to) in POINTER_TABLES_LAST {
        release_table(e, from, to);
    }
}

// Translated from 00b7a170 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map; a shader's virtual method (`thiscall`, no stack words):
/// `00be0870` on the same `this` (the exe leaves `ECX` as it came, 00b7a173; its answer is not
/// read), then empties the static pointer tables and shader slots, and zeroes the four
/// 0xbdc-byte tables at `011fcc80`, `011fc0a0`, `011fb4c0` and `011fa8e0`. Answers true.
pub fn fn_00b7a170(e: &mut Engine, this: Ptr) -> bool {
    e.call(SHADERS_RELEASE_00BE0870, &args![this]);
    release_static_tables(e);
    for table in [0x011f_cc80u32, 0x011f_c0a0, 0x011f_b4c0, 0x011f_a8e0] {
        e.call(MEMSET, &args![table, 0u32, 0xbdcu32]);
    }
    true
}

/// The vtable `00b79de0` stores (`010af2f8`) and the base destructor body
/// `00be0f10` (`thiscall`).
const SHADER_VTABLE_010AF2F8: u32 = 0x010a_f2f8;
const BASE_DESTRUCT_00BE0F10: u32 = 0x00be_0f10;

// Translated from 00b79de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b79de0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f28d99, state variable) and its stack cookie word are not translated
/// No function name in the engine map: a shader's destructor body
/// (`thiscall`, no stack words). Stores the vtable `010af2f8`, empties the
/// static pointer tables and shader slots, releases and clears the members
/// `+0x28`, `+0x6c`, `+0x70`, `+0x74`, `+0x78`, `+0x80`, `+0x7c`, `+0x88`,
/// `+0x84`, then runs the member pointers' own releases (`+0x88` down to
/// `+0x6c`, each reading the member again) and the base destructor body
/// (`00be0f10`).
pub fn fn_00b79de0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, SHADER_VTABLE_010AF2F8);
    release_static_tables(e);
    for at in [0x28u32, 0x6c, 0x70, 0x74, 0x78, 0x80, 0x7c, 0x88, 0x84] {
        let object = e.mem.u32(t + at);
        if object != 0 {
            release_object(e, object);
            e.mem.set_u32(t + at, 0);
        }
    }
    for at in [0x88u32, 0x84, 0x80, 0x7c, 0x78, 0x74, 0x70, 0x6c] {
        let object = e.mem.u32(t + at);
        release_object(e, object);
    }
    e.call(BASE_DESTRUCT_00BE0F10, &args![this]);
}

/// `BSShaderManager::GetShader` (`00b55560`, `cdecl`), the pass-texture
/// assignment `00c663d0` (`thiscall`, `RET 4`), the shader-model passes
/// `00c02ce0` and `00c02f70` (`cdecl`, the index), `00e7f5d0` (no
/// arguments; a new state block), `00e7f430` (`thiscall` on a state block,
/// `RET 0xc`), `00b71a10` (`thiscall` on a shader, `RET 0xc`) and
/// `00e7de90` (`thiscall`, `RET 4`).
const GET_SHADER: u32 = 0x00b5_5560;
const ASSIGN_00C663D0: u32 = 0x00c6_63d0;
const MODEL_2_PASS_00C02CE0: u32 = 0x00c0_2ce0;
const MODEL_1_PASS_00C02F70: u32 = 0x00c0_2f70;
const NEW_STATE_00E7F5D0: u32 = 0x00e7_f5d0;
const STATE_SET_00E7F430: u32 = 0x00e7_f430;
const SHADER_SET_00B71A10: u32 = 0x00b7_1a10;
const STAGE_SET_00E7DE90: u32 = 0x00e7_de90;
/// The flag word (`011f941c`, bit 0), the mode word compared with 6
/// (`011f91b8`), the stage object skipped (`011f9534`), and the four words
/// copied for pass 0x235 (`011ad850` to `011fa2f0`).
const FLAG_011F941C: u32 = 0x011f_941c;
const MODE_011F91B8: u32 = 0x011f_91b8;
const SKIPPED_STAGE_011F9534: u32 = 0x011f_9534;

/// The state block of the shader in slot `index` (`+0x30`, made by
/// `00e7f5d0` when missing), given `(state, value, 1)` through `00e7f430`.
fn set_state(e: &mut Engine, index: u32, state: u32, value: u32) {
    let shader = e.mem.u32(SHADERS + 4 * index);
    if e.mem.u32(shader + 0x30) == 0 {
        let block = e.call(NEW_STATE_00E7F5D0, &args![]).u32();
        e.mem.set_u32(shader + 0x30, block);
    }
    let block = e.mem.u32(shader + 0x30);
    e.call(STATE_SET_00E7F430, &args![block, state, value, 1u32]);
}

// Translated from 00b7a870 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (`cdecl`, one word: a pass index). The
/// shaders 1, 0xe, 0xf and 0xd (`GetShader`) each present get their two
/// textures (`+0x30` and `+0x34`) from `+0x7c`/`+0x80` for passes
/// 0x24e..0x251, else from `+0x84`/`+0x88` and then the shader-model pass
/// (`00c02ce0` from model 2, `00c02f70` at model 1). For a pass below 0x2f7
/// its slot's state block gets 0x1c (0), then 0xa8 either 0 (flag bit 0 of
/// `011f941c`) or through `00b71a10` with a value by pass (0xf for
/// 0x1e2..0x1e6, 0x1ec..0x1ee and 0x154..0x177; 8 for 0x14, 0x15, 0x2d6,
/// 0x2d7; 0 for 1..5 and 9; 0xf for 6..8; else 7); in mode 6 (`011f91b8`)
/// also 0x34 (1), 0x38 (8), 0x37 (7), 0x35 (1), 0x36 (1). Pass 0x235 copies
/// the four words at `011ad850` to `011fa2f0`; any other has each of its
/// stages (count `+0x18`, list `+0x24`) with an object other than the one at
/// `011f9534` given 3 (`00e7de90`), except stage 7.
pub fn fn_00b7a870(e: &mut Engine, pass: u32) {
    let shaders: Vec<u32> = [1u32, 0xe, 0xf, 0xd]
        .iter()
        .map(|&kind| e.call(GET_SHADER, &args![kind]).u32())
        .collect();
    let (first, second) = if pass.wrapping_sub(0x24e) <= 3 {
        (0x7cu32, 0x80u32)
    } else {
        (0x84, 0x88)
    };
    for &shader in &shaders {
        if shader != 0 {
            e.call(ASSIGN_00C663D0, &args![shader + 0x30, shader + first]);
            e.call(ASSIGN_00C663D0, &args![shader + 0x34, shader + second]);
        }
    }
    if pass.wrapping_sub(0x24e) > 3 {
        let model = e.global::<u32>(SHADER_MODEL_011F91C0) as i32;
        if model >= 2 {
            e.call(MODEL_2_PASS_00C02CE0, &args![pass]);
        } else if model == 1 {
            e.call(MODEL_1_PASS_00C02F70, &args![pass]);
        }
    }
    if pass >= SHADER_COUNT {
        return;
    }
    set_state(e, pass, 0x1c, 0);
    if e.mem.u8(FLAG_011F941C) & 1 != 0 {
        set_state(e, pass, 0xa8, 0);
    } else {
        let value = if (0x1e2..=0x1e6).contains(&pass)
            || pass.wrapping_sub(0x1ec) <= 2
            || pass.wrapping_sub(0x154) <= 0x23
        {
            0xf
        } else if pass == 0x2d6 || pass == 0x2d7 || pass == 0x14 || pass == 0x15 {
            8
        } else if pass.wrapping_sub(1) <= 4 {
            0
        } else if pass.wrapping_sub(6) <= 2 {
            0xf
        } else if pass == 9 {
            0
        } else {
            7
        };
        let shader = e.mem.u32(SHADERS + 4 * pass);
        e.call(SHADER_SET_00B71A10, &args![shader, 0xa8u32, value, 0u32]);
    }
    if e.global::<u32>(MODE_011F91B8) == 6 {
        for (state, value) in [(0x34u32, 1u32), (0x38, 8), (0x37, 7), (0x35, 1), (0x36, 1)] {
            set_state(e, pass, state, value);
        }
    }
    if pass == 0x235 {
        for word in 0..4u32 {
            let v = e.mem.u32(0x011a_d850 + 4 * word);
            e.mem.set_u32(0x011f_a2f0 + 4 * word, v);
        }
        return;
    }
    let mut stage = 0;
    while stage < e.mem.u32(e.mem.u32(SHADERS + 4 * pass) + 0x18) {
        let shader = e.mem.u32(SHADERS + 4 * pass);
        let entry = e.mem.u32(e.mem.u32(shader + 0x24) + 4 * stage);
        if entry != 0 {
            let object = e.mem.u32(entry + 8);
            if object != 0 && object != e.global::<u32>(SKIPPED_STAGE_011F9534) && stage != 7 {
                e.call(STAGE_SET_00E7DE90, &args![entry, 3u32]);
            }
        }
        stage += 1;
    }
}

// Translated from 00b79db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x138 of the shader virtual table `010af2f8` (the table `00b79b00` stores) and of many
/// other shader tables (`thiscall`, two words): stores the first word at +0x60 and the low
/// byte of the second at +0x64 (`BSShader::ePass` and `bFirstPass` in the Xbox PDB, which
/// puts another method in that slot, so the slot name is unconfirmed).
pub fn fn_00b79db0(e: &mut Engine, this: Ptr, pass: u32, first_pass: u8) {
    e.mem.set_u32(this.addr() + 0x60, pass);
    e.mem.set_u8(this.addr() + 0x64, first_pass);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b7_a380, fn_00b7a380() -> Ptr),
        entry!(0x00b7_9dd0, fn_00b79dd0() -> Ptr),
        entry!(0x00b7_a6f0, fn_00b7a6f0(Ptr, u32, u32) -> u32),
        entry!(0x00b7_a730, fn_00b7a730(Ptr) -> bool),
        entry!(0x00b7_ac20, fn_00b7ac20(Ptr, u32)),
        entry!(0x00b7_a2c0, fn_00b7a2c0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b7_a170, fn_00b7a170(Ptr) -> bool),
        entry!(0x00b7_9de0, fn_00b79de0(Ptr)),
        entry!(0x00b7_a870, fn_00b7a870(u32)),
        entry!(0x00b7_9db0, fn_00b79db0(Ptr, u32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const RENDERER: u32 = 0x0f50_0000;
    const DECLARATION_0: u32 = 0x0f60_0000;
    const DECLARATION_VTABLE: u32 = 0x0f10_0000;
    const DECLARATION_SLOTS: u32 = 0x0f30_0000;
    const SHADER: u32 = 0x0f70_0000;
    const SHADER_VTABLE: u32 = 0x0f20_0000;
    const SHADER_SLOTS: u32 = 0x0f40_0000;

    /// Every slot of both vtables is a no-op double at `<slots> + offset`; `Create` answers
    /// declarations 0x100 bytes apart; the allocation answers `SHADER`.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(RENDERER_011F9508, RENDERER);
        e.map(DECLARATION_0, 0x1000);
        e.map(SHADER, 0x1000);
        let declaration_slots: Vec<u32> = (0..0x60).map(|i| DECLARATION_SLOTS + 4 * i).collect();
        let shader_slots: Vec<u32> = (0..0x60).map(|i| SHADER_SLOTS + 4 * i).collect();
        e.put_vtable(DECLARATION_VTABLE, &declaration_slots);
        e.put_vtable(SHADER_VTABLE, &shader_slots);
        for target in declaration_slots.iter().chain(&shader_slots) {
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
        e.register(SHADER_CONSTRUCT_00B79B00, |e, a| {
            e.mem.set_u32(a[0], SHADER_VTABLE);
            a[0].into_ret()
        });
        e
    }

    fn entries(declaration: u32, list: &[[u32; 5]]) -> Vec<(u32, Vec<u32>)> {
        list.iter()
            .map(|entry| {
                let mut words = vec![declaration];
                words.extend_from_slice(entry);
                (DECLARATION_SLOTS + 0x8c, words)
            })
            .collect()
    }

    #[test]
    fn fn_00b7a380_makes_four_declarations_then_the_shader() {
        let mut e = rig();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b7_a380, &[]).u32(), SHADER);
        let mut log = e.call_log.take().unwrap();
        log.remove(0);
        let d = |n: u32| DECLARATION_0 + 0x100 * n;
        let slot = |offset: u32| DECLARATION_SLOTS + offset;
        let mut want: Vec<(u32, Vec<u32>)> = [6, 8, 3, 8]
            .iter()
            .map(|&n| (DECLARATION_CREATE, vec![RENDERER, n, 1]))
            .collect();
        want.extend(entries(d(0), &SIX_ENTRIES));
        want.push((slot(0xb4), vec![d(0)]));
        want.extend(entries(d(1), &EIGHT_ENTRIES));
        want.push((slot(0xb4), vec![d(1)]));
        want.push((slot(0x8c), vec![d(2), 0, 0, 0, 2, 0]));
        want.push((slot(0x8c), vec![d(2), 1, 3, 5, 1, 0]));
        want.push((slot(0x94), vec![d(2), 0, 2, 5, 0x8000_0000, 0, 5, 1, 0]));
        want.push((slot(0xb4), vec![d(2)]));
        want.extend(entries(d(3), &SIX_ENTRIES));
        want.push((slot(0x94), vec![d(3), 0, 6, 8, 0x8000_0000, 3, 5, 1, 0]));
        want.push((slot(0x94), vec![d(3), 0, 7, 9, 0x8000_0001, 3, 5, 2, 0]));
        want.push((slot(0xb4), vec![d(3)]));
        want.push((NI_ALLOCATE, vec![0x8c]));
        want.push((
            SHADER_CONSTRUCT_00B79B00,
            vec![SHADER, d(0), d(1), d(2), d(3)],
        ));
        for offset in [0x4c, 0x144, 0x150, 0x11c] {
            want.push((SHADER_SLOTS + offset, vec![SHADER]));
        }
        want.push((SHADER_SLOTS + 0xc0, vec![SHADER, d(0)]));
        assert_eq!(log, want);
    }
}

#[cfg(test)]
mod b008_tests {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()[1..].to_vec()
    }

    #[test]
    fn fn_00b79dd0_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b7_9dd0, &args![]).u32(), RECORD_011FED88);
    }

    #[test]
    fn fn_00b7a6f0_picks_by_flags() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        for (i, at) in [0x6cu32, 0x70, 0x74, 0x78].iter().enumerate() {
            e.mem.set_u32(this + at, 10 + i as u32);
        }
        let first = e.mem.alloc(0xc0);
        let second = e.mem.alloc(0x24);
        assert_eq!(e.call(0x00b7_a6f0, &args![this, first, second]).u32(), 10);
        e.mem.set_u32(second + 0x20, 0x4000);
        assert_eq!(e.call(0x00b7_a6f0, &args![this, first, second]).u32(), 13);
        e.mem.set_u32(second + 0x20, 0x5000);
        assert_eq!(e.call(0x00b7_a6f0, &args![this, first, second]).u32(), 12);
        e.mem.set_u32(first + 0xbc, 1);
        assert_eq!(e.call(0x00b7_a6f0, &args![this, first, second]).u32(), 11);
    }

    fn table_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_da48, 0x1250);
        e.map(0x011f_a8e0, 0x3200);
        e.map(SHADER_MODEL_011F91C0, 4);
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.register(INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_00b7a730_points_every_shader_at_the_pass_pointer() {
        let mut e = table_engine();
        e.register(PREPARE_SLOT_00E804D0, |_, _| Ret::default());
        for f in SET_UP_ALWAYS
            .iter()
            .chain(SET_UP_MODEL_2.iter())
            .chain([SET_UP_MODEL_1].iter())
        {
            e.register(*f, |_, _| Ret::default());
        }
        let block = e.mem.alloc(0x60 * SHADER_COUNT);
        for i in 0..SHADER_COUNT {
            e.mem.set_u32(SHADERS + 4 * i, block + 0x60 * i);
        }
        e.mem.set_u32(block + 0x5c, 0x5550);
        let pointer = e.mem.alloc(8);
        e.set_global::<u32>(PASS_POINTER_011FDD88, pointer);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00b7_a730, &args![0x9000u32]).bool());
        assert_eq!(e.mem.u32(block + 0x5c), pointer);
        assert_eq!(e.mem.u32(block + 0x60 * (SHADER_COUNT - 1) + 0x5c), pointer);
        let log = calls(&mut e);
        assert_eq!(
            log.iter().filter(|c| c.0 == INTERLOCKED_INCREMENT).count(),
            SHADER_COUNT as usize
        );
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![0x5554])));
        assert!(log.contains(&(SET_UP_MODEL_1, vec![0x9000])));
        assert!(log.iter().all(|c| c.0 != SET_UP_MODEL_2[0]));
        e.set_global::<u32>(SHADER_MODEL_011F91C0, 2);
        e.call_log = Some(vec![]);
        e.call(0x00b7_a730, &args![0x9000u32]);
        let log = calls(&mut e);
        assert!(log.contains(&(SET_UP_MODEL_2[12], vec![0x9000])));
        // Already pointing there: no reference changes.
        assert!(log.iter().all(|c| c.0 != INTERLOCKED_INCREMENT));
    }

    #[test]
    fn fn_00b7ac20_switches_by_mask() {
        const SLOT: u32 = 0x013c_0000;
        let mut e = table_engine();
        e.register(SLOT, |_, _| Ret::default());
        let table = e.mem.alloc(0x28);
        e.mem.set_u32(table + 0x24, SLOT);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, table);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, object);
        e.mem.set_u32(this + 0x30, object);
        let switches: Vec<u32> = (0..3).map(|_| e.mem.alloc(0xc)).collect();
        for (i, s) in switches.iter().enumerate() {
            e.mem.set_u8(s + 8, 1);
            e.mem.set_u32(0x011f_ebd8 + 4 * i as u32, *s);
        }
        // Pass 2's first mask has bit 2 (the second switch).
        e.mem.set_u32(MASKS_FIRST + 8, 4);
        e.call_log = Some(vec![]);
        e.call(0x00b7_ac20, &args![this, 2u32]);
        let bytes: Vec<u8> = switches.iter().map(|s| e.mem.u8(s + 8)).collect();
        assert_eq!(bytes, vec![1, 0, 1]);
        assert_eq!(calls(&mut e), vec![(SLOT, vec![object])]);
        // A switch already off stays off; no second mask, no second call.
        e.mem.set_u32(MASKS_FIRST + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b7_ac20, &args![this, 2u32]);
        assert!(calls(&mut e).is_empty());
    }

    #[test]
    fn fn_00b7a2c0_sends_the_bone_matrices() {
        const BONES_SLOT: u32 = 0x013d_0000;
        const TARGET_SLOT: u32 = 0x013d_0010;
        let mut e = Engine::new();
        e.map(RENDERER_011F9508, 4);
        e.set_global::<u32>(RENDERER_011F9508, 0x7700);
        e.map(DEVICE_HOLDER_0126F74C, 4);
        e.map(SWITCH_011FEC00, 4);
        e.register(RENDERER_SET_00E6FE30, |_, _| Ret::default());
        e.register(BONES_SLOT, |_, _| 0x4400u32.into_ret());
        e.register(TARGET_SLOT, |_, _| Ret::default());
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(table + 0x70, BONES_SLOT);
        e.mem.set_u32(table + 0x78, TARGET_SLOT);
        let target = e.mem.alloc(4);
        e.mem.set_u32(target, table);
        let holder = e.mem.alloc(0x60);
        e.mem.set_u32(holder + 0x5c, target);
        e.set_global::<u32>(DEVICE_HOLDER_0126F74C, holder);
        let shader_bones = e.mem.alloc(4);
        e.mem.set_u32(shader_bones, table);
        let this = e.mem.alloc(0x84);
        e.mem.set_u32(this + 0x34, shader_bones);
        e.mem.set_u32(this + 0x80, shader_bones);
        let geometry = e.mem.alloc(0x2c);
        e.mem.set_u32(geometry + 0x28, 0x2828);
        let bones = e.mem.alloc(0x24);
        e.mem.set_u32(bones + 4, 0x404);
        e.mem.set_u16(bones + 0x20, 12);
        e.call_log = Some(vec![]);
        let words = args![this, 0u32, geometry, bones, 0u32, 0u32, 0u32, 0x77u32, 0u32];
        assert_eq!(e.call(0x00b7_a2c0, &words).u32(), 0);
        let log = calls(&mut e);
        assert_eq!(
            log[0],
            (RENDERER_SET_00E6FE30, vec![0x7700, geometry, 0x77, 0, 3, 1])
        );
        assert_eq!(log[1], (BONES_SLOT, vec![shader_bones, BONE_MATRIX_NAME]));
        assert_eq!(
            log[2],
            (TARGET_SLOT, vec![target, 0x4400, 0x2828, 12, 3, 0x404])
        );
        // Different `+0x34`, no switch: nothing.
        e.mem.set_u32(this + 0x34, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b7_a2c0, &words);
        assert!(calls(&mut e).is_empty());
        // The switch on: the switch object heads the constants.
        let switch = e.mem.alloc(0xc);
        e.mem.set_u8(switch + 8, 1);
        e.set_global::<u32>(SWITCH_011FEC00, switch);
        e.call_log = Some(vec![]);
        e.call(0x00b7_a2c0, &words);
        let log = calls(&mut e);
        assert_eq!(
            log[1],
            (TARGET_SLOT, vec![target, switch, 0x2828, 12, 3, 0x404])
        );
    }

    #[test]
    fn fn_00b7a170_and_00b79de0_empty_the_tables() {
        let mut e = table_engine();
        for f in [
            SHADERS_RELEASE_00BE0870,
            SHADER_DELETE_00E80190,
            MEMSET,
            BASE_DESTRUCT_00BE0F10,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        let shader = e.mem.alloc(0x68);
        e.mem.set_u32(shader + 0x64, 1);
        e.mem.set_u32(SHADERS + 8, shader);
        e.mem.set_u32(0x011f_da48, 0x6660);
        e.mem.set_u32(0x011f_ec84, 0x6670);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00b7_a170, &args![0x5550u32]).bool());
        let log = calls(&mut e);
        assert!(
            log.contains(&(SHADERS_RELEASE_00BE0870, vec![0x5550])),
            "the same this"
        );
        assert!(log.contains(&(SHADER_DELETE_00E80190, vec![shader])));
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![0x6664])));
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![0x6674])));
        assert_eq!(log.iter().filter(|c| c.0 == MEMSET).count(), 4);
        assert_eq!(e.mem.u32(SHADERS + 8), 0);
        assert_eq!(e.mem.u32(0x011f_da48), 0);

        let this = e.mem.alloc(0x8c);
        e.mem.set_u32(this + 0x28, 0x6680);
        e.mem.set_u32(this + 0x84, 0x6690);
        e.call_log = Some(vec![]);
        e.call(0x00b7_9de0, &args![this]);
        let log = calls(&mut e);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010AF2F8);
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![0x6684])));
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![0x6694])));
        assert_eq!(e.mem.u32(this + 0x84), 0);
        assert_eq!(log.last().unwrap(), &(BASE_DESTRUCT_00BE0F10, vec![this]));
    }

    #[test]
    fn fn_00b7a870_sets_up_a_pass() {
        let mut e = table_engine();
        e.map(FLAG_011F941C, 4);
        e.map(MODE_011F91B8, 4);
        e.map(SKIPPED_STAGE_011F9534, 4);
        e.map(0x011a_d850, 0x10);
        for f in [
            ASSIGN_00C663D0,
            MODEL_2_PASS_00C02CE0,
            MODEL_1_PASS_00C02F70,
            STATE_SET_00E7F430,
            SHADER_SET_00B71A10,
            STAGE_SET_00E7DE90,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(NEW_STATE_00E7F5D0, |_, _| 0x5150u32.into_ret());
        e.register(GET_SHADER, |_, a| {
            (if a[0] == 1 { 0x6000u32 } else { 0 }).into_ret()
        });
        let shader = e.mem.alloc(0x40);
        let stages = e.mem.alloc(0x20);
        let stage = e.mem.alloc(0xc);
        e.mem.set_u32(stage + 8, 0x77);
        for i in [0u32, 7] {
            e.mem.set_u32(stages + 4 * i, stage);
        }
        e.mem.set_u32(shader + 0x18, 8);
        e.mem.set_u32(shader + 0x24, stages);
        e.mem.set_u32(SHADERS + 4 * 0x10, shader);
        e.set_global::<u32>(SHADER_MODEL_011F91C0, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b7_a870, &args![0x10u32]);
        let log = calls(&mut e);
        assert!(log.contains(&(ASSIGN_00C663D0, vec![0x6030, 0x6084])));
        assert!(log.contains(&(MODEL_1_PASS_00C02F70, vec![0x10])));
        assert!(log.contains(&(STATE_SET_00E7F430, vec![0x5150, 0x1c, 0, 1])));
        assert!(log.contains(&(SHADER_SET_00B71A10, vec![shader, 0xa8, 7, 0])));
        // Stage 0 is set, stage 7 skipped.
        assert_eq!(log.iter().filter(|c| c.0 == STAGE_SET_00E7DE90).count(), 1);
        assert_eq!(e.mem.u32(shader + 0x30), 0x5150);

        // Pass 0x235 copies the four words and touches no stage.
        e.mem.set_u32(0x011a_d850, 0x1234);
        e.mem.set_u32(SHADERS + 4 * 0x235, shader);
        e.mem.set_u32(MODE_011F91B8, 6);
        e.call_log = Some(vec![]);
        e.call(0x00b7_a870, &args![0x235u32]);
        let log = calls(&mut e);
        assert_eq!(e.mem.u32(0x011f_a2f0), 0x1234);
        assert!(log.iter().all(|c| c.0 != STAGE_SET_00E7DE90));
        assert_eq!(log.iter().filter(|c| c.0 == STATE_SET_00E7F430).count(), 6);
        // Passes 0x24e..0x251 take the other textures and skip the model pass.
        e.mem.set_u32(SHADERS + 4 * 0x24e, shader);
        e.call_log = Some(vec![]);
        e.call(0x00b7_a870, &args![0x24eu32]);
        let log = calls(&mut e);
        assert!(log.contains(&(ASSIGN_00C663D0, vec![0x6030, 0x607c])));
        assert!(log.iter().all(|c| c.0 != MODEL_1_PASS_00C02F70));
    }

    #[test]
    fn pass_setter_stores_the_pass_and_the_flag_byte() {
        let mut e = Engine::new();
        let shader = e.mem.alloc(0x6c);
        e.mem.set_u32(shader + 0x64, 0xaaaa_aa00);
        e.call(0x00b7_9db0, &args![shader, 0x1c8u32, 0x1234_5601u32]);
        assert_eq!(e.mem.u32(shader + 0x60), 0x1c8);
        assert_eq!(e.mem.u32(shader + 0x64), 0xaaaa_aa01);
    }
}
