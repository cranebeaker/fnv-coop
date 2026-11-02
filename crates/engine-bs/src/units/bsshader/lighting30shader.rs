//! `BSShader/lighting30shader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The shader's constructor `00bbc640` (engine map unit `lighting30shader.cpp`, unnamed,
/// `thiscall`, `RET 0x10`: four declarations).
const SHADER_CONSTRUCT_00BBC640: u32 = 0x00bb_c640;

/// The entries (vtable slot +0x8c, five words each) of the first two declarations (5
/// entries made), the next two (7 of 8), the two after (6 of 6) and the last two (8 of 8).
const FIVE_ENTRIES: [[u32; 5]; 5] = [
    [0, 0, 0, 2, 0],
    [1, 3, 3, 2, 0],
    [2, 7, 5, 1, 0],
    [3, 0xe, 0xd, 2, 0],
    [4, 0xf, 0xe, 2, 0],
];
const SEVEN_ENTRIES: [[u32; 5]; 7] = [
    [0, 0, 0, 2, 0],
    [1, 1, 1, 3, 0],
    [2, 2, 2, 4, 0],
    [3, 3, 3, 2, 0],
    [4, 7, 5, 1, 0],
    [5, 0xe, 0xd, 2, 0],
    [6, 0xf, 0xe, 2, 0],
];
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

/// The shader's `NiPointer` members the code assigns after the constructor (+0x80, +0x88,
/// +0x90, +0x98), inlined: when the member does not hold `object` already, the old one is
/// released (count at +4 through `InterlockedDecrement`, deleted through vtable slot +4 at
/// 0) when not null, the member set and `object` held (`InterlockedIncrement` on its +4; the
/// code makes no null test on this side).
fn assign_member(e: &mut Engine, member: u32, object: u32) {
    let old = e.mem.u32(member);
    if old == object {
        return;
    }
    if old != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.mem.set_u32(member, object);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

// Translated from 00bbd0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`): the creator `GetShader`
/// (`00b55560`) calls for shader 0x1d when the byte at `011f94a6` is set. No arguments.
///
/// Makes eight declarations (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`
/// read each time, 1 stream each): two of 6 entries (`a1`, `a2`), two of 8 (`b1`, `b2`), two
/// of 6 (`c1`, `c2`), two of 8 (`d1`, `d2`), in that order. Fills them through vtable slot
/// +0x8c and ends each pair (slot +0xb4, first then second) in the order a, c, b, d: the a
/// pair gets 5 entries ((0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 7, 5, 1, 0), (3, 0xe, 0xd, 2,
/// 0), (4, 0xf, 0xe, 2, 0)), the c pair 6 ((0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 5, 4, 3, 0),
/// (3, 7, 5, 1, 0), (4, 0xe, 0xd, 2, 0), (5, 0xf, 0xe, 2, 0)), the b pair 7 ((0, 0, 0, 2, 0),
/// (1, 1, 1, 3, 0), (2, 2, 2, 4, 0), (3, 3, 3, 2, 0), (4, 7, 5, 1, 0), (5, 0xe, 0xd, 2, 0),
/// (6, 0xf, 0xe, 2, 0)), the d pair 8 (the b list with (4, 5, 4, 3, 0) and (5, 7, 5, 1, 0) in
/// place of its fifth entry, then the last two). Allocates 0xa4 bytes and runs the
/// constructor `00bbc640` with (a1, b1, c1, d1) (a null allocation is used as it is), calls
/// the shader's slot +0x4c, assigns a2, c2, b2, d2 to its `NiPointer`s at +0x80, +0x88,
/// +0x90, +0x98, calls slots +0x148, +0x11c, +0x154, then +0xc0 with a1, and answers the
/// shader.
// Translation incomplete: 00bbd0d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bbd0d0(e: &mut Engine) -> Ptr {
    let make = |e: &mut Engine, entries: u32| {
        let renderer = e.global::<u32>(RENDERER_011F9508);
        e.call(DECLARATION_CREATE, &args![renderer, entries, 1u32])
            .u32()
    };
    let a1 = make(e, 6);
    let a2 = make(e, 6);
    let b1 = make(e, 8);
    let b2 = make(e, 8);
    let c1 = make(e, 6);
    let c2 = make(e, 6);
    let d1 = make(e, 8);
    let d2 = make(e, 8);
    let fill = |e: &mut Engine, first: u32, second: u32, entries: &[[u32; 5]]| {
        for declaration in [first, second] {
            for entry in entries {
                e.vcall(declaration, 0x8c, entry);
            }
        }
        e.vcall(first, 0xb4, &args![]);
        e.vcall(second, 0xb4, &args![]);
    };
    fill(e, a1, a2, &FIVE_ENTRIES);
    fill(e, c1, c2, &SIX_ENTRIES);
    fill(e, b1, b2, &SEVEN_ENTRIES);
    fill(e, d1, d2, &EIGHT_ENTRIES);
    let block = e.call(NI_ALLOCATE, &args![0xa4u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BBC640, &args![block, a1, b1, c1, d1])
            .u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    assign_member(e, shader.wrapping_add(0x80), a2);
    assign_member(e, shader.wrapping_add(0x88), c2);
    assign_member(e, shader.wrapping_add(0x90), b2);
    assign_member(e, shader.wrapping_add(0x98), d2);
    for slot in [0x148u32, 0x11c, 0x154] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![a1]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The lighting 3.0 shader's vtable (`010ba0f8`; no RTTI locator before it).
const LIGHTING30_VTABLE_010BA0F8: u32 = 0x010b_a0f8;
/// The shader model setting (`011f91c0`) and the word set to 5 below model 7 (`011ae60c`).
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
const WORD_SETTING_011AE60C: u32 = 0x011a_e60c;

// Translated from 00bbc640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`): the lighting 3.0 shader's
/// constructor (the creator `00bbd0d0` runs it), `thiscall` with four stack words (four
/// declarations). Runs `BSShader::BSShader` (`00be2200`), stores the vtable `010ba0f8`, clears
/// +0x6c to +0x98, sets +0x68 to 0x11; assigns the first declaration to the `NiPointer`s at
/// +0x28, +0x7c and +0x80, the second to +0x8c and +0x90, the third to +0x84 and +0x88 and the
/// fourth to +0x94 and +0x98; clears the `NiPointer`s at +0x6c, +0x70, +0x74 and +0x78; clears
/// the words `01200854` to `01200860`, the byte `01200864`, the words `01200848` and
/// `0120084c` and the 16-bit `01200850`; sets `011ae60c` to 5 when the shader model setting
/// (`011f91c0`) is below 7 (signed); clears +0x9c and sets +0xa0 to 1. Answers `this`.
// Translation incomplete: 00bbc640 - the C++ exception frame (FS:[0] chain, unwind state 0xc for the base and the pointers) is not translated.
pub fn fn_00bbc640(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    fourth: Ptr,
) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, LIGHTING30_VTABLE_010BA0F8);
    for offset in (0x6cu32..=0x98).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x68, 0x11);
    for (declaration, slots) in [
        (first, &[0x28u32, 0x7c, 0x80][..]),
        (second, &[0x8c, 0x90][..]),
        (third, &[0x84, 0x88][..]),
        (fourth, &[0x94, 0x98][..]),
    ] {
        for slot in slots {
            ni_pointer_assign(e, base + slot, declaration.addr());
        }
    }
    for offset in [0x6cu32, 0x70, 0x74, 0x78] {
        ni_pointer_clear(e, base + offset);
    }
    for word in [0x0120_0854u32, 0x0120_0858, 0x0120_085c, 0x0120_0860] {
        e.set_global::<u32>(word, 0);
    }
    e.set_global::<u8>(0x0120_0864, 0);
    e.set_global::<u32>(0x0120_0848, 0);
    e.set_global::<u32>(0x0120_084c, 0);
    e.set_global::<u16>(0x0120_0850, 0);
    if e.global::<i32>(SHADER_MODEL_011F91C0) < 7 {
        e.set_global::<u32>(WORD_SETTING_011AE60C, 5);
    }
    e.mem.set_u32(base + 0x9c, 0);
    e.mem.set_u32(base + 0xa0, 1);
    this
}

use crate::units::bsshader::particleshader::constant_maps::{
    add_entry, add_predefined_entry, make_constant_map,
};
use crate::units::bsshader::particleshader::reference_release;

/// The ambient colour constant (`01201b30` to `01201b3c`), the fog colour copied into it by
/// `00bbb840` (`011a9bd0` to `011a9bdc`), the flag objects the shader's constants are enabled
/// through (each a `bool` at +0x8: `012010c4`, `012010cc`, `012010d8`, `012010e0`) and the two
/// bytes `00bbb790` and `00bbb840` keep (`01200bf4`, `01200bf5`).
const AMBIENT_01201B30: u32 = 0x0120_1b30;
const FOG_COLOR_011A9BD0: u32 = 0x011a_9bd0;
const AMBIENT_ENABLE_012010C4: u32 = 0x0120_10c4;
const EMIT_ENABLE_012010CC: u32 = 0x0120_10cc;
const GLOW_ENABLE_012010D8: u32 = 0x0120_10d8;
const ALPHA_ENABLE_012010E0: u32 = 0x0120_10e0;
const AMBIENT_SET_01200BF4: u32 = 0x0120_0bf4;
const AMBIENT_FOG_01200BF5: u32 = 0x0120_0bf5;

// Translated from 00bbb790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `cdecl` with two words: a colour
/// (four words) and a byte flag. Copies the colour to `01201b30`; the ambient enable flag (the
/// object at `012010c4`, +0x8) becomes 0 when the flag and the byte `01200bf4` are both clear,
/// else 1; `01200bf4` then takes the flag.
pub fn fn_00bbb790(e: &mut Engine, color: Ptr, flag: u8) {
    for index in 0..4u32 {
        let word = e.mem.u32(color.addr() + 4 * index);
        e.set_global::<u32>(AMBIENT_01201B30 + 4 * index, word);
    }
    let enable = e.global::<u32>(AMBIENT_ENABLE_012010C4);
    let value = if flag == 0 && e.global::<u8>(AMBIENT_SET_01200BF4) == flag {
        0
    } else {
        1
    };
    e.mem.set_u8(enable + 8, value);
    e.set_global::<u8>(AMBIENT_SET_01200BF4, flag);
}

// Translated from 00bbb840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), no arguments: copies the fog
/// colour `011a9bd0` to `01201b30`, sets the ambient enable flag (`012010c4`, +0x8) and the
/// bytes `01200bf4` and `01200bf5`.
pub fn fn_00bbb840(e: &mut Engine) {
    for index in 0..4u32 {
        let word = e.global::<u32>(FOG_COLOR_011A9BD0 + 4 * index);
        e.set_global::<u32>(AMBIENT_01201B30 + 4 * index, word);
    }
    let enable = e.global::<u32>(AMBIENT_ENABLE_012010C4);
    e.mem.set_u8(enable + 8, 1);
    e.set_global::<u8>(AMBIENT_SET_01200BF4, 1);
    e.set_global::<u8>(AMBIENT_FOG_01200BF5, 1);
}

/// The emittance colour constant (`01201b50` to `01201b5c`).
const EMIT_01201B50: u32 = 0x0120_1b50;

// Translated from 00bbbda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall` with one stack word (the
/// shader property; `this` is not read): with the flag of `012010cc` set, copies the
/// property's words +0x84 to +0x90 to `01201b50`.
pub fn fn_00bbbda0(e: &mut Engine, _this: Ptr, property: Ptr) {
    let enable = e.global::<u32>(EMIT_ENABLE_012010CC);
    if e.mem.u8(enable + 8) == 0 {
        return;
    }
    for index in 0..4u32 {
        let word = e.mem.u32(property.addr() + 0x84 + 4 * index);
        e.set_global::<u32>(EMIT_01201B50 + 4 * index, word);
    }
}

/// The glow constants (`01201b90`, `01201b94`) and the `float` used for properties with bit 0x10
/// of +0x24 (`010177e8`).
const GLOW_01201B90: u32 = 0x0120_1b90;
const GLOW_SCALE_010177E8: u32 = 0x0101_77e8;

// Translated from 00bbbdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall` with one stack word (the
/// shader property; `this` is not read): with the flag of `012010d8` set, `01201b90` gets the
/// property's `float` +0xe0 and `01201b94` the `float` `010177e8` (bit 0x10 of the byte
/// +0x24) or 1.0.
pub fn fn_00bbbdf0(e: &mut Engine, _this: Ptr, property: Ptr) {
    let enable = e.global::<u32>(GLOW_ENABLE_012010D8);
    if e.mem.u8(enable + 8) == 0 {
        return;
    }
    let base = property.addr();
    let value = x87_quiet_bits(e.mem.u32(base + 0xe0));
    e.set_global::<u32>(GLOW_01201B90, value);
    let scale = if e.mem.u8(base + 0x24) & 0x10 != 0 {
        x87_quiet_bits(e.global::<u32>(GLOW_SCALE_010177E8))
    } else {
        1.0f32.to_bits()
    };
    e.set_global::<u32>(GLOW_01201B90 + 4, scale);
}

/// The alpha constants (`01201be8`, `01201bec`).
const ALPHA_01201BE8: u32 = 0x0120_1be8;

// Translated from 00bbbe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `stdcall` with one word (the shader
/// property): with the flag of `012010e0` set, `01201bec` gets 1.0 when the property's slot +0xf4
/// answers non-zero for (5, 0), else 0.0, and `01201be8` the property's `float` +0x30.
pub fn fn_00bbbe30(e: &mut Engine, property: Ptr) {
    let enable = e.global::<u32>(ALPHA_ENABLE_012010E0);
    if e.mem.u8(enable + 8) == 0 {
        return;
    }
    let base = property.addr();
    let answer = e.vcall(base, 0xf4, &args![5u32, 0u32]).u32();
    let flag = if answer != 0 { 1.0f32 } else { 0.0 };
    e.set_global::<u32>(ALPHA_01201BE8 + 4, flag.to_bits());
    let alpha = x87_quiet_bits(e.mem.u32(base + 0x30));
    e.set_global::<u32>(ALPHA_01201BE8, alpha);
}

/// The sized free `00aa1460` (`cdecl`: block, size).
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00bbef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`): the lighting 3.0 shader's scalar
/// deleting destructor, `thiscall`, one stack word (the flags). Runs the destructor
/// `00bbc9c0`; with bit 0 set, frees the 0xa4 bytes (`00aa1460`). Answers `this`.
pub fn fn_00bbef60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bbc9c0(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0xa4u32]);
    }
    this
}

// Translated from 00bbc9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the vtable `010ba0f8`, slot +0x8), `thiscall`, no stack
/// words: answers `01201f20`.
pub fn fn_00bbc9b0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_1f20
}

// Translated from 00bbd820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`; the vtable `010ba0f8`, slot +0x118),
/// `thiscall` with two stack words: the geometry (its word +0xb8 holds an object with a word
/// +0x28) and the shader property. Answers the declaration: +0x94 / +0x8c with bit 0x2 of the
/// property's byte +0x20, +0x84 / +0x7c without, the first of each pair when the object's +0x28
/// is set or the property has bit 0x20 of its byte +0x24.
pub fn fn_00bbd820(e: &mut Engine, this: Ptr, geometry: Ptr, property: Ptr) -> u32 {
    let base = this.addr();
    let holder = e.mem.u32(geometry.addr() + 0xb8);
    let mut skinned = e.mem.u32(holder + 0x28) != 0;
    skinned |= e.mem.u8(property.addr() + 0x24) & 0x20 != 0;
    let offset = if e.mem.u8(property.addr() + 0x20) & 2 != 0 {
        if skinned {
            0x94
        } else {
            0x8c
        }
    } else if skinned {
        0x84
    } else {
        0x7c
    };
    e.mem.u32(base + offset)
}

/// The pass number of the frame being drawn (`011f91e4`) and the shader's common set-up the
/// lighting shader falls back to (`00be1ff0`, `thiscall`, `RET 4`).
const PASS_011F91E4: u32 = 0x011f_91e4;
const SHADER_CALL_00BE1FF0: u32 = 0x00be_1ff0;

// Translated from 00bbc5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the vtable `010ba0f8`, slot +0x80), `thiscall` with one
/// stack word (the geometry's set-up record). For the passes 0x1cc to 0x1dc, 0x1bd to 0x1c3,
/// 0x1e0, 0x1e1 and 0x2d8 to 0x2ef nothing (answers the pass number, left in `EAX`); otherwise a
/// tail jump to `00be1ff0` with the record (answers what it answers).
pub fn fn_00bbc5c0(e: &mut Engine, this: Ptr, record: Ptr) -> u32 {
    let pass = e.global::<u32>(PASS_011F91E4);
    let own = (0x1cc..=0x1dc).contains(&pass)
        || (0x1bd..=0x1c3).contains(&pass)
        || (0x1e0..=0x1e1).contains(&pass)
        || (0x2d8..=0x2ef).contains(&pass);
    if own {
        return pass;
    }
    e.call(SHADER_CALL_00BE1FF0, &args![this, record]).u32()
}

/// The render state maker `00c0abd0` (unnamed; `thiscall` on the shader, RET 4, one word: the
/// pass; answers the state object stored in `0126f74c`) and the map selection `00c663d0`
/// (unnamed; `thiscall` on a constant map `NiPointer`, one word).
const RENDER_STATE_0126F74C: u32 = 0x0126_f74c;
const MAKE_RENDER_STATE_00C0ABD0: u32 = 0x00c0_abd0;
const SELECT_MAP_00C663D0: u32 = 0x00c6_63d0;

// Translated from 00bbe020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the vtable `010ba0f8`, slot +0xf0), `thiscall` with one
/// stack word: the pass. Stores `00c0abd0`'s state for the pass in `0126f74c`, copies the fog
/// colour to the ambient constant (as `00bbb840`, its flags set), then for the passes 0x1e0 and
/// 0x1e1 assigns the maps at +0x6c and +0x70 to the `NiPointer`s at +0x30 and +0x34, otherwise
/// runs `00c663d0` on +0x30 with +0x74 and on +0x34 with +0x78.
pub fn fn_00bbe020(e: &mut Engine, this: Ptr, pass: u32) {
    let base = this.addr();
    // 00bbe029: `00c0abd0` reads ECX (this shader, not changed before the call).
    let state = e.call(MAKE_RENDER_STATE_00C0ABD0, &args![this, pass]).u32();
    e.set_global::<u32>(RENDER_STATE_0126F74C, state);
    fn_00bbb840(e);
    if pass == 0x1e0 || pass == 0x1e1 {
        let map = e.mem.u32(base + 0x6c);
        ni_pointer_assign(e, base + 0x30, map);
        let map = e.mem.u32(base + 0x70);
        ni_pointer_assign(e, base + 0x34, map);
    } else {
        e.call(SELECT_MAP_00C663D0, &args![base + 0x30, base + 0x74]);
        e.call(SELECT_MAP_00C663D0, &args![base + 0x34, base + 0x78]);
    }
}

/// What `00bbe120` reads and writes: the fog property getter (`00b55520`, `cdecl`, the byte
/// kind at `011f91c4`), the objects whose byte +8 enables the fog (`012010bc`) and the eye
/// constant (`012010c0`) (entries 15 and 16 of the 17 at `01201080`, their bytes just set), the `float` 500000.0 (`010ad6e0`), the four words at `011a9bd0`, the
/// eye position (`011fa2a0`) and the world origin (`011f474c`), and the constants it fills
/// (`012013b0` fog, `012013c0` fog colour, `012013d0` eye).
const FOG_ENABLE_012010BC: u32 = 0x0120_10bc;
const EYE_ENABLE_012010C0: u32 = 0x0120_10c0;
const FAR_010AD6E0: u32 = 0x010a_d6e0;
const DEFAULT_COLOUR_011A9BD0: u32 = 0x011a_9bd0;
const EYE_011FA2A0: u32 = 0x011f_a2a0;
const FOG_CONSTANT_012013B0: u32 = 0x0120_13b0;
const FOG_COLOUR_012013C0: u32 = 0x0120_13c0;
const EYE_CONSTANT_012013D0: u32 = 0x0120_13d0;
/// The three jump tables of `00bbe120`'s pass switch (read from the exe): passes 10 to 15
/// (`00bbe5f4`), 0x1d4 to 0x1da (`00bbe60c`), and 0x2db to 0x2ed through the byte table at
/// `00bbe634` into the three targets at `00bbe628`.
const PASSES_00BBE5F4: [u32; 6] = [
    0x00bb_e15d,
    0x00bb_e167,
    0x00bb_e171,
    0x00bb_e17b,
    0x00bb_e185,
    0x00bb_e18f,
];
const PASSES_00BBE60C: [u32; 7] = [
    0x00bb_e1bb,
    0x00bb_e1c2,
    0x00bb_e1c9,
    0x00bb_e1d0,
    0x00bb_e1d7,
    0x00bb_e1de,
    0x00bb_e1e5,
];
const TARGETS_00BBE628: [u32; 3] = [0x00bb_e20c, 0x00bb_e1ec, 0x00bb_e213];
const BYTES_00BBE634: [u8; 19] = [0, 2, 2, 2, 2, 1, 0, 2, 2, 2, 2, 1, 0, 2, 2, 2, 2, 1, 0];

/// The pass index `00bbe120` picks for a pass (`ESI`), following its compares and jump tables.
fn pass_index_00bbe120(pass: u32) -> u32 {
    let target = if pass <= 0x1d3 {
        if pass == 0x1d3 {
            0x00bb_e199
        } else {
            let case = pass.wrapping_sub(10);
            if case > 5 {
                0x00bb_e213
            } else {
                PASSES_00BBE5F4[case as usize]
            }
        }
    } else if pass <= 0x2da {
        if pass == 0x2da {
            0x00bb_e1ec
        } else {
            let case = pass.wrapping_sub(0x1d4);
            if case > 6 {
                0x00bb_e213
            } else {
                PASSES_00BBE60C[case as usize]
            }
        }
    } else {
        let case = pass.wrapping_sub(0x2db);
        if case > 0x12 {
            0x00bb_e213
        } else {
            TARGETS_00BBE628[usize::from(BYTES_00BBE634[case as usize])]
        }
    };
    match target {
        0x00bb_e15d => 1,
        0x00bb_e167 => 2,
        0x00bb_e171 => 3,
        0x00bb_e17b => 4,
        0x00bb_e185 => 5,
        0x00bb_e18f => 6,
        0x00bb_e199 => 0x59,
        0x00bb_e1bb => 0x5a,
        0x00bb_e1c2 => 0x5b,
        0x00bb_e1c9 => 0x5c,
        0x00bb_e1d0 => 0x5d,
        0x00bb_e1d7 => 0x5e,
        0x00bb_e1de => 0x5f,
        0x00bb_e1e5 => 0x60,
        0x00bb_e1ec => 0x68,
        0x00bb_e20c => 0x69,
        // 00bbe213: LEA ESI, [EAX - 0x178].
        _ => pass.wrapping_sub(0x178),
    }
}

// Translated from 00bbe120 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00bbe120 to the `RET 4` at 00bbe5ef, jump tables after
/// it): slot 63 (+0xfc) of the vtable `010ba0f8`, `thiscall` with one stack word: the pass.
/// Nothing for the passes 0x1e0 and 0x1e1. Otherwise the pass picks an index (10..15: 1..6,
/// 0x1d3..0x1da: 0x59..0x60, 0x2da/0x2e0/0x2e6/0x2ec: 0x68, 0x2db/0x2e1/0x2e7/0x2ed: 0x69, any
/// other: pass - 0x178); for the 17 shaders at `01201080` the byte +8 (when set) and the bytes
/// `01200854 + i` become bit `i + 1` of the index's mask at `01200a28`; for the 10 objects at
/// `012010c4` and the bytes `01200848 + j` bit `j + 1` of the mask at `01200868`. Then slot +0x24
/// of the shader's `+0x34` and `+0x30`. With the byte +8 of `012010bc`'s object set and a fog
/// property of kind `011f91c4` (start +0x2c, end +0x30, colour +0x20 to +0x28, power +0x60):
/// when end and start are both 0 (`FUCOM`; NaN counts as not 0) `012013b0` = (500000, 0, power,
/// 0) and `012013c0` the words at `011a9bd0`; otherwise (end, end - start, power, 0) and (colour,
/// 1.0). With the byte +8 of `012010c0`'s object set, `012013d0` = the eye `011fa2a0` less the
/// origin `011f474c`, and 1.0.
pub fn fn_00bbe120(e: &mut Engine, this: Ptr, pass: u32) {
    if pass == 0x1e0 || pass == 0x1e1 {
        return;
    }
    let index = pass_index_00bbe120(pass);
    let mut bit = 1u32;
    for i in 0..0x11u32 {
        let object = e.global::<u32>(SHADERS_01201080 + 4 * i);
        if object != 0 {
            let mask = e.global::<u32>(MASKS_01200A28.wrapping_add(index.wrapping_mul(4)));
            e.mem
                .set_u8(object + 8, u8::from(mask & bit.rotate_left(1) != 0));
        }
        bit = bit.rotate_left(1);
        let mask = e.global::<u32>(MASKS_01200A28.wrapping_add(index.wrapping_mul(4)));
        e.set_global(FLAGS_01200854 + i, u8::from(mask & bit != 0));
    }
    let masks = MASKS_01200868.wrapping_add(index.wrapping_mul(4));
    for j in 1..=10u32 {
        let object = e.global::<u32>(FLAGS_012010C4 + 4 * (j - 1));
        if object != 0 {
            let mask = e.global::<u32>(masks);
            e.mem.set_u8(object + 8, (mask >> j) as u8 & 1);
        }
        let mask = e.global::<u32>(masks);
        e.set_global(FLAGS_01200848 + (j - 1), (mask >> j) as u8 & 1);
    }
    let base = this.addr();
    let first = e.mem.u32(base + 0x34);
    e.vcall(first, 0x24, &args![]);
    let second = e.mem.u32(base + 0x30);
    e.vcall(second, 0x24, &args![]);
    let fog_enable = e.global::<u32>(FOG_ENABLE_012010BC);
    if e.mem.u8(fog_enable + 8) != 0 {
        let kind = u32::from(e.global::<u8>(FOG_KIND_011F91C4));
        let fog = e.call(GET_FOG_PROPERTY_00B55520, &args![kind]).u32();
        if fog != 0 {
            let start_bits = x87_quiet_bits(e.mem.u32(fog + 0x2c));
            let colour = [0x20u32, 0x24, 0x28].map(|offset| e.mem.u32(fog + offset));
            let end_bits = x87_quiet_bits(e.mem.u32(fog + 0x30));
            let power_bits = x87_quiet_bits(e.mem.u32(fog + 0x60));
            let start = x87_load32(start_bits);
            let end = x87_load32(end_bits);
            if end == 0.0 && start == 0.0 {
                let far = x87_quiet_bits(e.global::<u32>(FAR_010AD6E0));
                e.set_global(FOG_CONSTANT_012013B0, far);
                e.set_global(FOG_CONSTANT_012013B0 + 4, x87_store32(0.0));
                e.set_global(FOG_CONSTANT_012013B0 + 8, power_bits);
                e.set_global(FOG_CONSTANT_012013B0 + 0xc, x87_store32(0.0));
                for i in 0..4u32 {
                    let word = e.global::<u32>(DEFAULT_COLOUR_011A9BD0 + 4 * i);
                    e.set_global(FOG_COLOUR_012013C0 + 4 * i, word);
                }
            } else {
                e.set_global(FOG_CONSTANT_012013B0, x87_store32(end));
                e.set_global(FOG_CONSTANT_012013B0 + 4, x87_store32(x87_sub(end, start)));
                e.set_global(FOG_CONSTANT_012013B0 + 8, power_bits);
                e.set_global(FOG_CONSTANT_012013B0 + 0xc, x87_store32(0.0));
                for (i, word) in colour.iter().enumerate() {
                    e.set_global(FOG_COLOUR_012013C0 + 4 * i as u32, x87_quiet_bits(*word));
                }
                e.set_global(FOG_COLOUR_012013C0 + 0xc, x87_store32(1.0));
            }
        }
    }
    let eye_enable = e.global::<u32>(EYE_ENABLE_012010C0);
    if e.mem.u8(eye_enable + 8) != 0 {
        for i in 0..3u32 {
            let eye = x87_m32(e.global::<u32>(EYE_011FA2A0 + 4 * i));
            let origin = x87_m32(e.global::<u32>(ORIGIN_011F474C + 4 * i));
            let value = x87_quiet_bits(x87_store32(x87_sub(eye, origin)));
            e.set_global(EYE_CONSTANT_012013D0 + 4 * i, value);
        }
        e.set_global(EYE_CONSTANT_012013D0 + 0xc, x87_store32(1.0));
    }
}

/// The shader-constant setters `00c0cf10` calls (unit `run_00c048b0`, `thiscall` on the shader):
/// `00c0bae0` (state, property, a count), `00c0bb20` (state, property), `00c0bb50` (state,
/// property, a byte), `00c0bba0` (state, property), `00c0bbd0` (state), `00c0bc00` (state,
/// property, pass), `00c0bc90` (state, property), `00c0bcf0` (state, property, a word), and the
/// word `011f9530` it copies.
const CONSTANTS_00C0BAE0: u32 = 0x00c0_bae0;
const CONSTANTS_00C0BB20: u32 = 0x00c0_bb20;
const CONSTANTS_00C0BB50: u32 = 0x00c0_bb50;
const CONSTANTS_00C0BBA0: u32 = 0x00c0_bba0;
const CONSTANTS_00C0BBD0: u32 = 0x00c0_bbd0;
const CONSTANTS_00C0BC00: u32 = 0x00c0_bc00;
const CONSTANTS_00C0BC90: u32 = 0x00c0_bc90;
const CONSTANTS_00C0BCF0: u32 = 0x00c0_bcf0;
const STATE_WORD_SOURCE_011F9530: u32 = 0x011f_9530;
/// The pass switch of `00c0cf10` for 0x182 to 0x1e1: the byte table at `00c0d110` into the 17
/// code addresses at `00c0d0cc` (read from the exe).
const TARGETS_00C0D0CC: [u32; 17] = [
    0x00c0_cf8e,
    0x00c0_d035,
    0x00c0_d06a,
    0x00c0_cfa2,
    0x00c0_d0c6,
    0x00c0_cffa,
    0x00c0_cfed,
    0x00c0_d00c,
    0x00c0_cfbc,
    0x00c0_d02d,
    0x00c0_d062,
    0x00c0_cfd8,
    0x00c0_d026,
    0x00c0_d07e,
    0x00c0_d0b9,
    0x00c0_d0a7,
    0x00c0_d0c6,
];
const BYTES_00C0D110: [u8; 96] = [
    0x00, 0x00, 0x01, 0x02, 0x03, 0x03, 0x10, 0x10, 0x00, 0x00, 0x03, 0x03, 0x04, 0x04, 0x05, 0x00,
    0x00, 0x01, 0x02, 0x03, 0x03, 0x04, 0x04, 0x00, 0x00, 0x03, 0x03, 0x06, 0x06, 0x07, 0x08, 0x08,
    0x09, 0x0a, 0x0b, 0x0b, 0x06, 0x06, 0x08, 0x08, 0x0b, 0x0b, 0x06, 0x06, 0x0c, 0x08, 0x08, 0x09,
    0x0a, 0x0b, 0x0b, 0x06, 0x06, 0x08, 0x08, 0x0b, 0x0b, 0x10, 0x10, 0x0d, 0x0d, 0x0d, 0x0d, 0x0d,
    0x0d, 0x0d, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x0e, 0x10, 0x0e, 0x10, 0x0e, 0x10,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x0f, 0x0f,
];

/// The word at `[[state + 0x24] + slot] + 8` set to `value` (a pass state's constant entry).
fn set_state_entry(e: &mut Engine, state: u32, slot: u32, value: u32) {
    let entries = e.mem.u32(state + 0x24);
    let entry = e.mem.u32(entries + slot);
    e.mem.set_u32(entry + 8, value);
}

// Translated from 00c0cf10 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00c0cf10 to the `RET 8` at 00c0d0c8, its jump tables
/// after it): slot 80 (+0x140) of the vtable `010ba0f8`, `thiscall` with two stack words: the
/// pass and the property. With the pass state `0126f74c` (read once, first) as `state`: passes
/// 0xe and 0xf run `00bbdda0` (property, state); 0x181 sets the state's constant entries +0x10
/// (the word `011f9530`) and +0x14 (the word at the property's +0xb4); 0x182 to 0x1e1 go through
/// the jump table to the shader-constant setters `00c0bae0` .. `00c0bcf0`, `00bbdcd0` and the
/// entries +0x10, +0x14 and +0x18 (the property's +0xac, +0xb0 and +0xb4 words); other passes:
/// nothing.
pub fn fn_00c0cf10(e: &mut Engine, this: Ptr, pass: u32, property: u32) {
    let state = e.global::<u32>(RENDER_STATE_0126F74C);
    if pass <= 0x181 {
        if pass == 0x181 {
            let word = e.global::<u32>(STATE_WORD_SOURCE_011F9530);
            set_state_entry(e, state, 0x10, word);
            let source = e.mem.u32(property + 0xb4);
            let value = e.mem.u32(source);
            set_state_entry(e, state, 0x14, value);
        } else if (0xe..=0xf).contains(&pass) {
            e.call(SET_UP_00BBDDA0, &args![this, property, state]);
        }
        return;
    }
    let case = pass.wrapping_sub(0x182);
    if case > 0x5f {
        return;
    }
    let target = TARGETS_00C0D0CC[usize::from(BYTES_00C0D110[case as usize])];
    match target {
        0x00c0_cf8e => {
            e.call(CONSTANTS_00C0BAE0, &args![this, state, property, 4u32]);
        }
        0x00c0_d035 => mapped_entries(e, state, property),
        0x00c0_d06a => {
            e.call(CONSTANTS_00C0BB50, &args![this, state, property, 1u32]);
        }
        0x00c0_cfa2 => property_entry_18(e, state, property),
        0x00c0_cffa => {
            e.call(CONSTANTS_00C0BB20, &args![this, state, property]);
        }
        0x00c0_cfed => {
            e.call(CONSTANTS_00C0BBD0, &args![this, state]);
        }
        // 00c0d026 pushes the property itself and joins 00c0d00c's calls.
        0x00c0_d00c | 0x00c0_d026 => {
            e.call(CONSTANTS_00C0BB20, &args![this, state, property]);
            e.call(CONSTANTS_00C0BBD0, &args![this, state]);
        }
        0x00c0_cfbc => {
            e.call(CONSTANTS_00C0BBD0, &args![this, state]);
            e.call(CONSTANTS_00C0BAE0, &args![this, state, property, 4u32]);
        }
        0x00c0_d02d => {
            e.call(CONSTANTS_00C0BBD0, &args![this, state]);
            mapped_entries(e, state, property);
        }
        0x00c0_d062 => {
            e.call(CONSTANTS_00C0BBD0, &args![this, state]);
            e.call(CONSTANTS_00C0BB50, &args![this, state, property, 1u32]);
        }
        0x00c0_cfd8 => {
            property_entry_18(e, state, property);
            e.call(CONSTANTS_00C0BBD0, &args![this, state]);
        }
        0x00c0_d07e => {
            e.call(CONSTANTS_00C0BC00, &args![this, state, property, pass]);
            e.call(CONSTANTS_00C0BC90, &args![this, state, property]);
            e.call(CONSTANTS_00C0BCF0, &args![this, state, property, 0u32]);
        }
        0x00c0_d0b9 => {
            e.call(CONSTANTS_00C0BBA0, &args![this, state, property]);
        }
        0x00c0_d0a7 => {
            e.call(SET_UP_00BBDCD0, &args![this, property, state]);
        }
        // 00c0d0c6: nothing.
        _ => {}
    }
}

/// `00c0cfa2`: the state's entry +0x18 takes the word at the property's +0xb4.
fn property_entry_18(e: &mut Engine, state: u32, property: u32) {
    let source = e.mem.u32(property + 0xb4);
    let value = e.mem.u32(source);
    set_state_entry(e, state, 0x18, value);
}

/// `00c0d035`: the state's entries +0x10 and +0x14 take the words +4 of the property's +0xac
/// and +0xb0.
fn mapped_entries(e: &mut Engine, state: u32, property: u32) {
    let first = e.mem.u32(property + 0xac);
    let value = e.mem.u32(first + 4);
    set_state_entry(e, state, 0x10, value);
    let second = e.mem.u32(property + 0xb0);
    let value = e.mem.u32(second + 4);
    set_state_entry(e, state, 0x14, value);
}

// Translated from 00bbe650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the vtable `010ba0f8`, slot +0x78), `thiscall` with one
/// stack word: the geometry's set-up record (the property at +0xc). Except for the passes 0x1e0,
/// 0x1e1, 0xe and 0xf: its own slots +0x13c and +0x140 with (pass, property), then +0x10c.
pub fn fn_00bbe650(e: &mut Engine, this: Ptr, record: Ptr) {
    let base = this.addr();
    let property = e.mem.u32(record.addr() + 0xc);
    let pass = e.global::<u32>(PASS_011F91E4);
    if matches!(pass, 0x1e0 | 0x1e1 | 0xe | 0xf) {
        return;
    }
    e.vcall(base, 0x13c, &args![pass, property]);
    e.vcall(base, 0x140, &args![pass, property]);
    e.vcall(base, 0x10c, &args![]);
}

/// The unnamed `cdecl` state setters `00b97e80` (unit `bsrenderstate_xenon.cpp`) and `00b98070`,
/// `BSRenderState::SetZWriteEnable` (engine map, `00b97e30`) and the counter `00bbeef0` lowers
/// (`011ff9dc`).
const STATE_CALL_00B97E80: u32 = 0x00b9_7e80;
const STATE_CALL_00B98070: u32 = 0x00b9_8070;
const SET_Z_WRITE_ENABLE: u32 = 0x00b9_7e30;
const COUNTER_011FF9DC: u32 = 0x011f_f9dc;

// Translated from 00bbeef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the vtable `010ba0f8`, slot +0x8c), `thiscall` with one
/// stack word: the geometry's set-up record. Undoes the set-up's states: `00b97e80(3, 0)` for the
/// passes 0x1e0 and 0x1e1; `00b98070(0, 0)` when the object at +0x10 has flags bit 0; for the
/// passes 0x1d3 to 0x1d6 with bit 0 of the flags of the object at +0 (no null test), lowers the
/// counter `011ff9dc` and `SetZWriteEnable(1, 0)`.
pub fn fn_00bbeef0(e: &mut Engine, _this: Ptr, record: Ptr) {
    let pass = e.global::<u32>(PASS_011F91E4);
    if pass == 0x1e0 || pass == 0x1e1 {
        e.call(STATE_CALL_00B97E80, &args![3u32, 0u32]);
    }
    let record = record.addr();
    let other = e.mem.u32(record + 0x10);
    if other != 0 && e.mem.u8(other + 0x18) & 1 != 0 {
        e.call(STATE_CALL_00B98070, &args![0u32, 0u32]);
    }
    if pass.wrapping_sub(0x1d3) <= 3 {
        let flags = e.mem.u32(record);
        if e.mem.u8(flags + 0x18) & 1 != 0 {
            let count = e.global::<u32>(COUNTER_011FF9DC).wrapping_sub(1);
            e.set_global::<u32>(COUNTER_011FF9DC, count);
            e.call(SET_Z_WRITE_ENABLE, &args![1u32, 0u32]);
        }
    }
}

/// The flag objects enabling the light position constants (`012010a4`, `012010c8`), the light
/// direction (`011fa2a0` to `011fa2ac`), the scene camera holder (`011f91c8`, its position at
/// +0x1e4 to +0x1ec), `D3DXVec3TransformCoord` (`00ee6df8`, `stdcall`: out, vector, matrix), and
/// the constants written: `01201370` to `0120137c` and `01201b40` to `01201b4c`.
const LIGHT_ENABLE_012010A4: u32 = 0x0120_10a4;
const LIGHT_ENABLE_012010C8: u32 = 0x0120_10c8;
const LIGHT_DIRECTION_011FA2A0: u32 = 0x011f_a2a0;
const CAMERA_011F91C8: u32 = 0x011f_91c8;
const VEC3_TRANSFORM_COORD: u32 = 0x00ee_6df8;
const LIGHT_POSITION_01201370: u32 = 0x0120_1370;
const LIGHT_POSITION_01201B40: u32 = 0x0120_1b40;

// Translated from 00bbb890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall` with seventeen stack words
/// (`RET 0x44`): a matrix (sixteen `float`s, by value) and an object with a `float` at +0x30.
/// With one of the flags of `012010a4` and `012010c8` set: the light direction `011fa2a0` plus
/// the camera position (`[011f91c8]` +0x1e4 to +0x1ec; x87, each sum rounded to a `float`) is
/// transformed by the matrix (`D3DXVec3TransformCoord` on the stack: the result over the sum,
/// the matrix in place) and stored in `01201370` (with 1.0 in `0120137c`) and in `01201b40` (with
/// the object's +0x30 in `01201b4c`).
pub fn fn_00bbb890(e: &mut Engine, _this: Ptr, matrix: [u32; 16], holder: Ptr) {
    let first = e.global::<u32>(LIGHT_ENABLE_012010A4);
    if e.mem.u8(first + 8) == 0 {
        let second = e.global::<u32>(LIGHT_ENABLE_012010C8);
        if e.mem.u8(second + 8) == 0 {
            return;
        }
    }
    // The frame: the locals (0x1c bytes), the return address, then the stack words.
    e.with_stack(0x64, |e, frame| {
        let frame = frame.addr();
        for (index, word) in matrix.iter().enumerate() {
            e.mem.set_u32(frame + 0x20 + 4 * index as u32, *word);
        }
        e.mem.set_u32(frame + 0x60, holder.addr());
        let direction =
            [0u32, 4, 8].map(|offset| e.global::<u32>(LIGHT_DIRECTION_011FA2A0 + offset));
        let quiet = direction.map(x87_quiet_bits);
        let camera = e.global::<u32>(CAMERA_011F91C8);
        let position = [0x1e4u32, 0x1e8, 0x1ec].map(|offset| e.mem.u32(camera + offset));
        let sum0 = x87_store32(x87_add(x87_load32(direction[0]), x87_m32(position[0])));
        let sum1 = x87_store32(x87_add(x87_load32(position[1]), x87_m32(quiet[1])));
        let sum2 = x87_store32(x87_add(x87_load32(position[2]), x87_m32(quiet[2])));
        for (index, word) in [sum0, sum1, sum2].iter().enumerate() {
            e.mem.set_u32(frame + 4 * index as u32, *word);
            e.mem
                .set_u32(frame + 0x10 + 4 * index as u32, x87_quiet_bits(*word));
        }
        e.call(
            VEC3_TRANSFORM_COORD,
            &args![frame, frame + 0x10, frame + 0x20],
        );
        let out = [0u32, 4, 8].map(|offset| x87_quiet_bits(e.mem.u32(frame + offset)));
        for (index, word) in out.iter().enumerate() {
            e.set_global::<u32>(LIGHT_POSITION_01201370 + 4 * index as u32, *word);
        }
        e.set_global::<u32>(LIGHT_POSITION_01201370 + 0xc, 1.0f32.to_bits());
        for (index, word) in out.iter().enumerate() {
            e.set_global::<u32>(LIGHT_POSITION_01201B40 + 4 * index as u32, *word);
        }
        let holder = e.mem.u32(frame + 0x60);
        let value = x87_quiet_bits(e.mem.u32(holder + 0x30));
        e.set_global::<u32>(LIGHT_POSITION_01201B40 + 0xc, value);
    });
}

/// The uniform-form entry of [`fn_00bbb890`]: `this`, the sixteen matrix words, the object.
fn fn_00bbb890_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let mut matrix = [0u32; 16];
    matrix.copy_from_slice(&a[1..17]);
    fn_00bbb890(e, Ptr::new(a[0]), matrix, Ptr::new(a[17]));
    Ret::default()
}

/// The scene origin offset added to positions (`011f474c` to `011f4754`), the flag objects of
/// the eye position (`012010b0`), the world matrix (`012010b4`) and the fog (`012010e4`)
/// constants, and the constants: the eye position (`01201360` to `01201368`), the world matrix
/// (`012012f0` to `0120132c`) followed by the eight words copied from `011fd874` (`01201330` to
/// `0120134c`), the fog (`01201bc0` to `01201bdc`).
const ORIGIN_011F474C: u32 = 0x011f_474c;
const EYE_ENABLE_012010B0: u32 = 0x0120_10b0;
const WORLD_ENABLE_012010B4: u32 = 0x0120_10b4;
const FOG_ENABLE_012010E4: u32 = 0x0120_10e4;
const EYE_01201360: u32 = 0x0120_1360;
const WORLD_012012F0: u32 = 0x0120_12f0;
const WORLD_EXTRA_011FD874: u32 = 0x011f_d874;
const FOG_01201BC0: u32 = 0x0120_1bc0;

// Translated from 00bbbcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall` with one stack word: a
/// position (three `float`s). With the flag of `012010b0` set, stores the position minus the
/// origin offset `011f474c` minus the camera position (`[011f91c8]` +0x1e4 to +0x1ec) in
/// `01201360` (x87, each difference rounded to a `float`).
pub fn fn_00bbbcf0(e: &mut Engine, _this: Ptr, position: Ptr) {
    let enable = e.global::<u32>(EYE_ENABLE_012010B0);
    if e.mem.u8(enable + 8) == 0 {
        return;
    }
    let camera = e.global::<u32>(CAMERA_011F91C8);
    for index in 0..3u32 {
        let value = e.mem.u32(position.addr() + 4 * index);
        let origin = e.global::<u32>(ORIGIN_011F474C + 4 * index);
        let first = x87_store32(x87_sub(x87_load32(value), x87_m32(origin)));
        let camera_value = e.mem.u32(camera + 0x1e4 + 4 * index);
        let second = x87_store32(x87_sub(x87_load32(first), x87_m32(camera_value)));
        e.set_global::<u32>(EYE_01201360 + 4 * index, x87_quiet_bits(second));
    }
}

// Translated from 00bbba00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall` with two stack words: a
/// flag and a transform (a rotation of nine `float`s, a translation at +0x24, a scale at +0x30).
/// With the flag of `012010b4` set, builds the world matrix: without the flag word the
/// transform's rotation rows with the translation plus the camera position (`[011f91c8]`
/// +0x1e4..) in the fourth column, (0, 0, 0, scale) as the last row; with it the identity with
/// the origin offset `011f474c` plus the camera position. The sixteen `float`s go to `012012f0`,
/// the eight words of `011fd874` to `01201330`.
pub fn fn_00bbba00(e: &mut Engine, _this: Ptr, flag: u32, transform: Ptr) {
    let enable = e.global::<u32>(WORLD_ENABLE_012010B4);
    if e.mem.u8(enable + 8) == 0 {
        return;
    }
    let camera = e.global::<u32>(CAMERA_011F91C8);
    let position = [0x1e4u32, 0x1e8, 0x1ec].map(|offset| e.mem.u32(camera + offset));
    let mut matrix = [0u32; 16];
    let base = transform.addr();
    if flag == 0 {
        for row in 0..3u32 {
            for column in 0..3u32 {
                let word = e.mem.u32(base + 0xc * row + 4 * column);
                matrix[(4 * row + column) as usize] = x87_quiet_bits(word);
            }
            let translation = e.mem.u32(base + 0x24 + 4 * row);
            matrix[(4 * row + 3) as usize] = x87_store32(x87_add(
                x87_load32(translation),
                x87_m32(position[row as usize]),
            ));
        }
        let scale = e.mem.u32(base + 0x30);
        matrix[15] = x87_quiet_bits(scale);
    } else {
        let one = 1.0f32.to_bits();
        for row in 0..3u32 {
            matrix[(5 * row) as usize] = one;
            let origin = e.global::<u32>(ORIGIN_011F474C + 4 * row);
            matrix[(4 * row + 3) as usize] =
                x87_store32(x87_add(x87_load32(origin), x87_m32(position[row as usize])));
        }
        matrix[15] = one;
    }
    for (index, word) in matrix.iter().enumerate() {
        e.set_global::<u32>(WORLD_012012F0 + 4 * index as u32, x87_quiet_bits(*word));
    }
    for index in 0..8u32 {
        let word = e.global::<u32>(WORLD_EXTRA_011FD874 + 4 * index);
        e.set_global::<u32>(WORLD_012012F0 + 0x40 + 4 * index, word);
    }
}

/// The fog plane and distances (`011f9604` to `011f9610`, `011f9614` the near and `011f9618`
/// the far distance) and the -1.0 stored when both are 0 (`01012054`).
const FOG_PLANE_011F9604: u32 = 0x011f_9604;
const FOG_OFFSET_011F9610: u32 = 0x011f_9610;
const FOG_NEAR_011F9614: u32 = 0x011f_9614;
const FOG_FAR_011F9618: u32 = 0x011f_9618;
const MINUS_ONE_01012054: u32 = 0x0101_2054;

// Translated from 00bbbe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall`, no stack words. With the
/// flag of `012010e4` set: `01201bc0` to `01201bc8` get the fog plane `011f9604` to `011f960c`,
/// `01201bcc` its dot product with the light direction `011fa2a0` plus the camera position
/// (each sum rounded to a `float`) minus `011f9610`; then, when the far (`011f9618`) and near
/// (`011f9614`) distances are both 0.0 (`FUCOM`, NaN is not), `01201bd0` to `01201bdc` get (0.0,
/// 0.0, `01012054`, 0.0), else (far, far - near, 1.0, 1.0). x87 in the exe's order.
pub fn fn_00bbbe80(e: &mut Engine, _this: Ptr) {
    let enable = e.global::<u32>(FOG_ENABLE_012010E4);
    if e.mem.u8(enable + 8) == 0 {
        return;
    }
    let direction = [0u32, 4, 8]
        .map(|offset| x87_quiet_bits(e.global::<u32>(LIGHT_DIRECTION_011FA2A0 + offset)));
    let camera = e.global::<u32>(CAMERA_011F91C8);
    let position = [0x1e4u32, 0x1e8, 0x1ec].map(|offset| e.mem.u32(camera + offset));
    let plane = [0u32, 4, 8].map(|offset| e.global::<u32>(FOG_PLANE_011F9604 + offset));
    let offset = e.global::<u32>(FOG_OFFSET_011F9610);
    let sums = [0usize, 1, 2].map(|index| {
        x87_store32(x87_add(
            x87_load32(position[index]),
            x87_m32(direction[index]),
        ))
    });
    for (index, word) in plane.iter().enumerate() {
        e.set_global::<u32>(FOG_01201BC0 + 4 * index as u32, x87_quiet_bits(*word));
    }
    let first = x87_mul(x87_load32(plane[0]), x87_m32(sums[0]));
    let second = x87_mul(x87_load32(plane[1]), x87_m32(sums[1]));
    let sum = x87_add(first, second);
    let third = x87_mul(x87_load32(plane[2]), x87_m32(sums[2]));
    let dot = x87_store32(x87_add(sum, third));
    let distance = x87_store32(x87_sub(x87_load32(dot), x87_m32(offset)));
    e.set_global::<u32>(FOG_01201BC0 + 0xc, distance);
    let far_bits = x87_quiet_bits(e.global::<u32>(FOG_FAR_011F9618));
    let near_bits = x87_quiet_bits(e.global::<u32>(FOG_NEAR_011F9614));
    let far = x87_load32(far_bits);
    let near = x87_load32(near_bits);
    let words = if far == 0.0 && near == 0.0 {
        [0, 0, x87_quiet_bits(e.global::<u32>(MINUS_ONE_01012054)), 0]
    } else {
        [
            far_bits,
            x87_store32(x87_sub(far, near)),
            1.0f32.to_bits(),
            1.0f32.to_bits(),
        ]
    };
    for (index, word) in words.iter().enumerate() {
        e.set_global::<u32>(FOG_01201BC0 + 0x10 + 4 * index as u32, *word);
    }
}

/// The shader's shared `NiPointer`s and passes `00bbce70` and `00bbc9c0` release: the shaders
/// at `01200dd8` to `01200eb8` (57), the pairs at `01200c48` and `01200d10` (50 each), the 0x70
/// passes at `01200ec0`, the shaders at `01201080` to `012010c0` (17), the constant flag objects
/// at `012010c4` to `012010e8` (10); then the constant block from `012010f0` (the fog colour,
/// repeated over 0x28c words by an overlapping forward copy) and from `01201b30` (0xf8 words).
const SHADERS_01200DD8: u32 = 0x0120_0dd8;
const PAIRS_01200C48: u32 = 0x0120_0c48;
const PAIRS_01200D10: u32 = 0x0120_0d10;
const PASSES_01200EC0: u32 = 0x0120_0ec0;
const SHADERS_01201080: u32 = 0x0120_1080;
const FLAGS_012010C4: u32 = 0x0120_10c4;
const CONSTANTS_012010F0: u32 = 0x0120_10f0;

/// `REP MOVSD` from `source` to `destination` (`count` words, ascending), as the exe copies
/// them: an overlapping copy forward repeats the first words.
fn copy_words(e: &mut Engine, destination: u32, source: u32, count: u32) {
    for index in 0..count {
        let word = e.mem.u32(source + 4 * index);
        e.mem.set_u32(destination + 4 * index, word);
    }
}

/// The shared part of `00bbce70` and `00bbc9c0` (the shaders, passes and constant blocks above).
fn release_shared(e: &mut Engine) {
    for index in 0..57u32 {
        ni_pointer_clear(e, SHADERS_01200DD8 + 4 * index);
    }
    for index in 0..50u32 {
        ni_pointer_clear(e, PAIRS_01200C48 + 4 * index);
        ni_pointer_clear(e, PAIRS_01200D10 + 4 * index);
    }
    for index in 0..0x70u32 {
        pass_pointer_clear(e, PASSES_01200EC0 + 4 * index);
    }
    for index in 0..17u32 {
        ni_pointer_clear(e, SHADERS_01201080 + 4 * index);
    }
    for index in 0..10u32 {
        ni_pointer_clear(e, FLAGS_012010C4 + 4 * index);
    }
    let color = [0u32, 4, 8, 0xc].map(|offset| e.global::<u32>(FOG_COLOR_011A9BD0 + offset));
    for (index, word) in color.iter().enumerate() {
        e.set_global::<u32>(CONSTANTS_012010F0 + 4 * index as u32, *word);
    }
    copy_words(e, CONSTANTS_012010F0 + 0x10, CONSTANTS_012010F0, 0x28c);
    for (index, word) in color.iter().enumerate() {
        e.set_global::<u32>(AMBIENT_01201B30 + 4 * index as u32, *word);
    }
    copy_words(e, AMBIENT_01201B30 + 0x10, AMBIENT_01201B30, 0xf8);
}

/// `00be0870` (unnamed, `thiscall`, no stack words).
const SHADER_CALL_00BE0870: u32 = 0x00be_0870;

// Translated from 00bbce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`; the vtable `010ba0f8`, slot +0x4c),
/// `thiscall`, no stack words; answers true. Runs `00be0870` (its answer is not used), then
/// releases the shared shaders and passes and resets the constant blocks (`release_shared`).
pub fn fn_00bbce70(e: &mut Engine, this: Ptr) -> bool {
    e.call(SHADER_CALL_00BE0870, &args![this]);
    release_shared(e);
    true
}

/// `BSShader::~BSShader` (engine map, `00be0f10`).
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;

// Translated from 00bbc9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`): the lighting 3.0 shader's destructor,
/// `thiscall`, no stack words. Stores the vtable `010ba0f8`, releases the shared shaders and
/// passes and resets the constant blocks (`release_shared`), clears the `NiPointer`s at +0x28 and
/// +0x7c to +0x98 and the word +0x9c; then the members' destructors run on +0x98 down to +0x6c
/// and `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00bbc9c0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xc for the members) is not translated.
pub fn fn_00bbc9c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, LIGHTING30_VTABLE_010BA0F8);
    release_shared(e);
    ni_pointer_clear(e, base + 0x28);
    for offset in (0x7cu32..=0x98).step_by(4) {
        ni_pointer_clear(e, base + offset);
    }
    e.mem.set_u32(base + 0x9c, 0);
    for offset in (0x6cu32..=0x98).rev().step_by(4) {
        let object = e.mem.u32(base + offset);
        reference_release(e, object);
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The flag objects of the view projection (`01201098`) and the projection (`012010b8`)
/// constants, those constants (`01201170` to `012011ac`, `012012b0` to `012012ec`), CRT `memcpy`
/// (`00ec44d0`), `D3DXMatrixMultiply` (`00ee6de0`, `stdcall`: out, a, b), `D3DXMatrixTranspose`
/// (`00ee6dd4`, `stdcall`: out, in) and `NiD3DUtility::GetD3DFromNi` (engine map, `00b799a0`,
/// `cdecl`: the D3D matrix, the Ni transform).
const VIEW_ENABLE_01201098: u32 = 0x0120_1098;
const PROJECTION_ENABLE_012010B8: u32 = 0x0120_10b8;
const VIEW_01201170: u32 = 0x0120_1170;
const PROJECTION_012012B0: u32 = 0x0120_12b0;
const MEMCPY: u32 = 0x00ec_44d0;
const MATRIX_MULTIPLY: u32 = 0x00ee_6de0;
const MATRIX_TRANSPOSE: u32 = 0x00ee_6dd4;
const GET_D3D_FROM_NI: u32 = 0x00b7_99a0;

/// The sixteen words of the identity matrix as `float`s.
fn identity() -> [u32; 16] {
    let mut matrix = [0u32; 16];
    for index in 0..4 {
        matrix[5 * index] = 1.0f32.to_bits();
    }
    matrix
}

// Translated from 00bbe6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall` with four stack words
/// (`RET 0x10`; a 16-byte aligned frame): the scene context (a byte at +0x9, the light list at
/// +0xc), the pass, a flag word and a transform. With the flag of `01201098` set: when there is
/// no light list, the context's byte with the first light's byte +0xed set, or the passes
/// 0x1bd to 0x1c3 and 0x2d8 to 0x2ef, the shader's matrix at +0x940 of its +0x18 object is
/// transposed (`D3DXMatrixTranspose` in place) and copied to `01201170` (through the x87);
/// otherwise, with the context's byte set, the identity (or the D3D form of the transform,
/// `GetD3DFromNi`, without the flag word) with the origin offset `011f474c` added to its
/// translation is multiplied by the first light's (passes 0x1c4 to 0x1c7) or the second light's
/// matrix at +0x10 (`D3DXMatrixMultiply`), transposed and copied (`memcpy`). With the flag of
/// `012010b8` set and the flag word set, the identity is copied to `012012b0`.
pub fn fn_00bbe6b0(e: &mut Engine, this: Ptr, context: Ptr, pass: u32, flag: u32, transform: Ptr) {
    e.with_stack(0xd0, |e, frame| {
        let frame = frame.addr();
        let matrix = frame + 0x10;
        let local = frame + 0x50;
        let result = frame + 0x90;
        let enable = e.global::<u32>(VIEW_ENABLE_01201098);
        if e.mem.u8(enable + 8) != 0 {
            let lights = e.mem.u32(context.addr() + 0xc);
            let selected = e.mem.u8(context.addr() + 9);
            let copy = lights == 0
                || (selected != 0 && {
                    let first = e.mem.u32(lights);
                    e.mem.u8(first + 0xed) != 0
                })
                || (0x1bd..=0x1c3).contains(&pass)
                || (0x2d8..=0x2ef).contains(&pass);
            if copy {
                let source = e.mem.u32(this.addr() + 0x18) + 0x940;
                copy_words(e, matrix, source, 16);
                e.call(MATRIX_TRANSPOSE, &args![matrix, matrix]);
                for index in 0..16u32 {
                    let word = x87_quiet_bits(e.mem.u32(matrix + 4 * index));
                    e.set_global::<u32>(VIEW_01201170 + 4 * index, word);
                }
            } else if selected != 0 {
                let unit = identity();
                for index in 0..16u32 {
                    e.mem.set_u32(local + 4 * index, unit[index as usize]);
                    e.mem.set_u32(matrix + 4 * index, unit[index as usize]);
                }
                if flag == 0 {
                    e.call(GET_D3D_FROM_NI, &args![local, transform]);
                }
                for index in 0..3u32 {
                    let origin = e.global::<u32>(ORIGIN_011F474C + 4 * index);
                    let word = e.mem.u32(local + 0x30 + 4 * index);
                    let sum = x87_store32(x87_add(x87_load32(origin), x87_m32(word)));
                    e.mem.set_u32(local + 0x30 + 4 * index, sum);
                }
                let light = if pass.wrapping_sub(0x1c4) <= 3 {
                    e.mem.u32(lights)
                } else {
                    e.mem.u32(lights + 4)
                };
                e.call(MATRIX_MULTIPLY, &args![result, local, light + 0x10]);
                copy_words(e, matrix, result, 16);
                e.call(MATRIX_TRANSPOSE, &args![matrix, matrix]);
                e.call(MEMCPY, &args![VIEW_01201170, matrix, 0x40u32]);
            }
        }
        let enable = e.global::<u32>(PROJECTION_ENABLE_012010B8);
        if e.mem.u8(enable + 8) != 0 && flag != 0 {
            let unit = identity();
            for index in 0..16u32 {
                e.mem.set_u32(matrix + 4 * index, unit[index as usize]);
            }
            e.call(MEMCPY, &args![PROJECTION_012012B0, matrix, 0x40u32]);
        }
    });
}

/// The material constants (`01201b60` to `01201b6c`, set first from `011a9be0` to `011a9bec`),
/// the lighting mode (`011f91b8`), the flag object of the alpha test constant (`012010d0`), the
/// `double` 255.0 (`0101e568`), and the callees: the property's alpha test `004a2020`
/// (`thiscall`, `RET 4`), `BSShaderLightingProperty::GetFirstActiveNonShadowLight` (engine map,
/// `00b70600`, `RET 4`), the vector length `00b719e0` (`thiscall`, answers in `ST0`), the
/// transform `00439ef0` (`RET 8`), `00633b40` (`RET 8`), `00457990` (`thiscall`, `ST0`) and the
/// `cdecl` clamps `0068b050` (a `double`, a `float`) and `00b719c0` (two `double`s).
const MATERIAL_01201B60: u32 = 0x0120_1b60;
const MATERIAL_DEFAULT_011A9BE0: u32 = 0x011a_9be0;
const MODE_011F91B8: u32 = 0x011f_91b8;
const ALPHA_TEST_ENABLE_012010D0: u32 = 0x0120_10d0;
const TWO_FIFTY_FIVE_0101E568: u32 = 0x0101_e568;
const ALPHA_TEST_004A2020: u32 = 0x004a_2020;
const FIRST_ACTIVE_LIGHT: u32 = 0x00b7_0600;
const VECTOR_LENGTH_00B719E0: u32 = 0x00b7_19e0;
const TRANSFORM_00439EF0: u32 = 0x0043_9ef0;
const CALL_00633B40: u32 = 0x0063_3b40;
const LENGTH_00457990: u32 = 0x0045_7990;
const CLAMP_0068B050: u32 = 0x0068_b050;
const CLAMP_00B719C0: u32 = 0x00b7_19c0;

/// The light-scaled alpha of `00bbe9c0` (passes 0x1bd to 0x1c3, modes other than 8 and 10):
/// answers the x87 stack's 1.0 path as the value it leaves for the code after it.
fn light_alpha(e: &mut Engine, fallback: u32, property: u32) {
    let factor = x87_store32(e.vcall(property, 0x110, &args![]).f64());
    let alpha_bits = e.mem.u32(property + 0x28);
    let alpha = x87_store32(x87_mul(x87_load32(alpha_bits), x87_m32(factor)));
    let value = x87_load32(alpha);
    if value < 1.0 {
        e.set_global::<u32>(MATERIAL_01201B60, x87_store32(value));
    } else {
        e.set_global::<u32>(MATERIAL_01201B60, 1.0f32.to_bits());
    }
    if e.mem.u32(property + 0x24) & 0x8000 == 0 {
        return;
    }
    if e.call(ALPHA_TEST_004A2020, &args![property, 0x11u32]).u32() as u8 != 0 {
        return;
    }
    // `other` is a whole 12-byte vector (00633b40 writes three floats), so the light
    // iterator word follows it. In the exe (00bbe9c0, ESP-based) the vector, the scaled
    // vector and the iterator are separate locals (ESP+0x8, ESP+0x18, ESP+0x34).
    e.with_stack(0x1c, |e, frame| {
        let frame = frame.addr();
        let vector = frame;
        let other = frame + 0xc;
        let out = frame + 0x18;
        let mut best = 0.0f32.to_bits();
        e.mem.set_u32(out, 0);
        let light = e.call(FIRST_ACTIVE_LIGHT, &args![property, out]).u32();
        let camera = e.global::<u32>(CAMERA_011F91C8);
        let node = e.mem.u32(camera + 0xe0);
        let target = e.mem.u32(node + 0xf8);
        for index in 0..3u32 {
            let word = e.mem.u32(target + 0xd4 + 4 * index);
            e.mem.set_u32(vector + 4 * index, word);
        }
        let length = x87_store32(e.call(VECTOR_LENGTH_00B719E0, &args![vector]).f64());
        if light != 0 {
            let data = e.mem.u32(light + 0xf8);
            e.call(TRANSFORM_00439EF0, &args![data + 0x8c, vector, fallback]);
            let radius = x87_quiet_bits(e.mem.u32(data + 0xc4));
            e.call(CALL_00633B40, &args![data + 0xd4, other, radius]);
            let distance = e.call(LENGTH_00457990, &args![vector]).f64();
            let ratio = x87_store32(x87_div(distance, x87_m32(e.mem.u32(data + 0xe0))));
            let low = e
                .call(CLAMP_0068B050, &args![0.0f64, x87_quiet_bits(ratio)])
                .f64();
            let clamped = x87_store32(e.call(CLAMP_00B719C0, &args![1.0f64, low]).f64());
            let rest = x87_store32(x87_sub(1.0, x87_load32(clamped)));
            let other_length = e.call(VECTOR_LENGTH_00B719E0, &args![other]).f64();
            best = x87_store32(x87_mul(other_length, x87_m32(rest)));
        }
        let best_value = x87_load32(best);
        let length_value = x87_load32(length);
        let chosen = if length_value < best_value {
            best_value
        } else {
            length_value
        };
        let chosen = x87_store32(chosen);
        let scaled = x87_store32(x87_mul(
            x87_load32(chosen),
            x87_m32(e.global::<u32>(MATERIAL_01201B60)),
        ));
        e.set_global::<u32>(MATERIAL_01201B60, scaled);
    });
}

// Translated from 00bbe9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall` with four stack words: a
/// fallback object, the shader property, the geometry's set-up record and the pass. Resets the
/// material constant `01201b60` from `011a9be0`. In lighting mode 8 (`011f91b8`): `01201b60` =
/// 0.0 with bit 0x20000 of the property's +0x24, else 1.0; `01201b64` = 0.0 when the record's
/// flags object (+0) has bit 0 or bit 9, else 1.0; done. Otherwise, for the passes 0x1bd to 0x1c3
/// outside mode 10, the alpha scaled by the property's slot +0x110 (`light_alpha`); else
/// `01201b60` = the property's alpha +0x28 when below 1.0, else 1.0. Then `01201b64` = the flags'
/// byte +0x1a / 255.0 with bit 9 (else 0.0), `01201b68` / `01201b6c` = 1.0 for the flags' field
/// 0 / 2 (bits 5 to 8; else 0.0), and the byte `01200bf5` becomes 1 when `01201b60` is below
/// 1.0; the flag of `012010d0` is set unless both the old and the new byte are clear.
pub fn fn_00bbe9c0(
    e: &mut Engine,
    _this: Ptr,
    fallback: u32,
    property: Ptr,
    record: Ptr,
    pass: u32,
) {
    let property = property.addr();
    for index in 0..4u32 {
        let word = e.global::<u32>(MATERIAL_DEFAULT_011A9BE0 + 4 * index);
        e.set_global::<u32>(MATERIAL_01201B60 + 4 * index, word);
    }
    let mode = e.global::<u32>(MODE_011F91B8);
    if mode == 8 {
        let alpha = if e.mem.u32(property + 0x24) & 0x2_0000 != 0 {
            0.0f32
        } else {
            1.0
        };
        e.set_global::<u32>(MATERIAL_01201B60, alpha.to_bits());
        let flags = e.mem.u32(record.addr());
        let off = flags != 0 && {
            let bits = e.mem.u16(flags + 0x18);
            bits & 1 != 0 || (bits >> 9) & 1 != 0
        };
        let value = if off { 0.0f32 } else { 1.0 };
        e.set_global::<u32>(MATERIAL_01201B60 + 4, value.to_bits());
        return;
    }
    if mode != 10 && pass.wrapping_sub(0x1bd) <= 6 {
        light_alpha(e, fallback, property);
    } else {
        let alpha = x87_load32(e.mem.u32(property + 0x28));
        let value = if 1.0 > alpha {
            x87_store32(alpha)
        } else {
            1.0f32.to_bits()
        };
        e.set_global::<u32>(MATERIAL_01201B60, value);
    }
    let flags = e.mem.u32(record.addr());
    let bits = e.mem.u16(flags + 0x18);
    let fade = if (bits >> 9) & 1 != 0 {
        let byte = f64::from(e.mem.u8(flags + 0x1a));
        let divisor = f64::from_bits(e.global::<u64>(TWO_FIFTY_FIVE_0101E568));
        x87_store32(x87_div(byte, divisor))
    } else {
        0.0f32.to_bits()
    };
    e.set_global::<u32>(MATERIAL_01201B60 + 4, fade);
    e.set_global::<u32>(MATERIAL_01201B60 + 8, 0);
    e.set_global::<u32>(MATERIAL_01201B60 + 0xc, 0);
    let field = (u32::from(e.mem.u16(flags + 0x18)) >> 5) & 0xf;
    if field == 0 {
        e.set_global::<u32>(MATERIAL_01201B60 + 8, 1.0f32.to_bits());
    } else if field == 2 {
        e.set_global::<u32>(MATERIAL_01201B60 + 0xc, 1.0f32.to_bits());
    }
    let old = e.global::<u8>(AMBIENT_FOG_01200BF5);
    let alpha = x87_m32(e.global::<u32>(MATERIAL_01201B60));
    let below = u8::from(1.0 > alpha);
    e.set_global::<u8>(AMBIENT_FOG_01200BF5, below);
    let enable = e.global::<u32>(ALPHA_TEST_ENABLE_012010D0);
    let value = if old == 0 && below == 0 { 0 } else { 1 };
    e.mem.set_u8(enable + 8, value);
}

/// The scale setting (`011f941e`), the colour meaning "none" (`011f4998` to `011f49a0`), the
/// light scale holder chain (`011f91ac` +0x8, +0x4, +0x6c, the `float` +0xc) and the diffuse
/// constant (`01201b70` to `01201b7c`).
const ALWAYS_SCALE_011F941E: u32 = 0x011f_941e;
const NO_COLOR_011F4998: u32 = 0x011f_4998;
const SCALE_HOLDER_011F91AC: u32 = 0x011f_91ac;
const DIFFUSE_01201B70: u32 = 0x0120_1b70;

// Translated from 00bbecd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall` with two stack words: the
/// material (its colour pointer +0x34, else its own colour +0x28; a scale at +0x40) and the shader
/// property (its light list at +0xb4). Nothing when the colour equals `011f4998` and there are
/// no lights. With the setting `011f941e`, the colour is scaled by the material's scale times
/// the light scale (each rounded to a `float`). With lights: the colour (clamped to 1.0 without
/// the setting) goes to `01201b70` with 0.0. Without: it is added to the ambient constant
/// `01201b30` (each sum rounded; clamped to 1.0 without the setting), whose flag and byte
/// `01200bf4` are set.
pub fn fn_00bbecd0(e: &mut Engine, _this: Ptr, material: Ptr, property: Ptr) {
    let material = material.addr();
    let mut source = e.mem.u32(material + 0x34);
    if source == 0 {
        source = material + 0x28;
    }
    let words = [0u32, 4, 8].map(|offset| e.mem.u32(source + offset));
    let mut color = words.map(x87_load32);
    let none = [0u32, 4, 8].map(|offset| x87_load32(e.global::<u32>(NO_COLOR_011F4998 + offset)));
    let list = e.mem.u32(property.addr() + 0xb4);
    let lights = e.mem.u32(list) != 0;
    if none[0] == color[0] && none[1] == color[1] && none[2] == color[2] && !lights {
        return;
    }
    let scaled = e.global::<u8>(ALWAYS_SCALE_011F941E) != 0;
    if scaled {
        let holder = e.global::<u32>(SCALE_HOLDER_011F91AC);
        let first = e.mem.u32(holder + 8);
        let second = e.mem.u32(first + 4);
        let third = e.mem.u32(second + 0x6c);
        let light_scale = e.mem.u32(third + 0xc);
        let factor = x87_store32(x87_mul(
            x87_load32(light_scale),
            x87_m32(e.mem.u32(material + 0x40)),
        ));
        let factor = x87_load32(factor);
        color = color.map(|value| x87_load32(x87_store32(x87_mul(value, factor))));
    }
    if !lights {
        let ambient = [0u32, 4, 8, 0xc].map(|offset| e.global::<u32>(AMBIENT_01201B30 + offset));
        let mut sums = [
            x87_store32(x87_add(color[0], x87_load32(ambient[0]))),
            x87_store32(x87_add(color[1], x87_load32(ambient[1]))),
            x87_store32(x87_add(color[2], x87_m32(ambient[2]))),
        ];
        if !scaled {
            for sum in sums.iter_mut() {
                if !(1.0 > x87_m32(*sum)) {
                    *sum = 1.0f32.to_bits();
                }
            }
        }
        for (index, word) in sums.iter().enumerate() {
            e.set_global::<u32>(AMBIENT_01201B30 + 4 * index as u32, *word);
        }
        e.set_global::<u32>(AMBIENT_01201B30 + 0xc, ambient[3]);
        let enable = e.global::<u32>(AMBIENT_ENABLE_012010C4);
        e.mem.set_u8(enable + 8, 1);
        e.set_global::<u8>(AMBIENT_SET_01200BF4, 1);
        return;
    }
    if !scaled {
        for value in color.iter_mut() {
            if !(1.0 > *value) {
                *value = 1.0;
            }
        }
    }
    for (index, value) in color.iter().enumerate() {
        e.set_global::<u32>(DIFFUSE_01201B70 + 4 * index as u32, x87_store32(*value));
    }
    e.set_global::<u32>(DIFFUSE_01201B70 + 0xc, 0);
}

/// `BSShaderManager::GetAccumulator`'s neighbour `00b4f5d0` (unnamed, unit `bsshadermanager.cpp`;
/// no arguments; answers an object whose +0x154 to +0x160 hold the ambient colour of lighting
/// mode 0xf).
const SHADER_MANAGER_CALL_00B4F5D0: u32 = 0x00b4_f5d0;

// Translated from 00bbc050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall` with seven stack words
/// (`RET 0x1c`): the shader property, the scene context, the pass, the material, the light
/// matrix, a flag word and a transform. Copies the property's `float` +0x6c into the first stack
/// word; in lighting mode 0xf (`011f91b8`) the ambient constant `01201b30` gets the four words at
/// +0x154 of `00b4f5d0`'s object, otherwise the code continues at `00bbc0a4` (the light set-up).
pub fn fn_00bbc050(
    e: &mut Engine,
    _this: Ptr,
    property: Ptr,
    context: Ptr,
    pass: u32,
    material: Ptr,
    matrix: u32,
    flag: u32,
    transform: Ptr,
) {
    let scale = x87_quiet_bits(e.mem.u32(property.addr() + 0x6c));
    if e.global::<u32>(MODE_011F91B8) == 0xf {
        let source = e.call(SHADER_MANAGER_CALL_00B4F5D0, &args![]).u32();
        for index in 0..4u32 {
            let word = e.mem.u32(source + 0x154 + 4 * index);
            e.set_global::<u32>(AMBIENT_01201B30 + 4 * index, word);
        }
        return;
    }
    fn_00bbc0a4(
        e, property, scale, context, pass, material, matrix, flag, transform,
    );
}

/// The light setters `00b70de0` and `00b71130` (unnamed; `cdecl`: index, light, a `float`), the
/// light colour and its scale (`011fa0d0` to `011fa0dc`, `011f9184`), the settings that scale it
/// (`011f941e`, with bit 0x400 of the property's +0x20) and replace it by the fog colour
/// (`011f91a7`), the light count limit (`011ae60c`), the setting clearing the fourth light word
/// (`011f9427`), the 0.5 of mode 10 (`01016248`), the light constants (`01201c70` to `01201c8c`,
/// then 0x20-byte entries from `01201c90`), the light data flag object (`012010e8`, its count at
/// +0x20), `BSShaderLightingProperty::GetNumberofActiveLights` (engine map, `00b70790`),
/// `D3DXVec3TransformNormal` (`00ee6df2`), `D3DXVec3Normalize` (`00ee6dec`), the vector
/// normalisation `004a0c10` (unnamed; `thiscall`) and the `_ftol2` helper (`00ec62c0`).
const LIGHT_SET_00B70DE0: u32 = 0x00b7_0de0;
const LIGHT_SET_00B71130: u32 = 0x00b7_1130;
const LIGHT_COLOR_011FA0D0: u32 = 0x011f_a0d0;
const LIGHT_SCALE_011F9184: u32 = 0x011f_9184;
const FOG_LIGHTS_011F91A7: u32 = 0x011f_91a7;
const LIGHT_LIMIT_011AE60C: u32 = 0x011a_e60c;
const NO_FOURTH_011F9427: u32 = 0x011f_9427;
const HALF_01016248: u32 = 0x0101_6248;
const LIGHT_DATA_01201C70: u32 = 0x0120_1c70;
const LIGHT_ENTRIES_01201C90: u32 = 0x0120_1c90;
const LIGHT_DATA_ENABLE_012010E8: u32 = 0x0120_10e8;
const ACTIVE_LIGHTS: u32 = 0x00b7_0790;
const VEC3_TRANSFORM_NORMAL: u32 = 0x00ee_6df2;
const VEC3_NORMALIZE: u32 = 0x00ee_6dec;
const UNITIZE_004A0C10: u32 = 0x004a_0c10;
const FTOL: u32 = 0x00ec_62c0;

// Translated from 00bbc0a4 (decompiled, FalloutNV.exe 1.4.0.525)
/// The light set-up after `00bbc050`'s mode test (Ghidra splits it into its own function; it is
/// entered only by `00bbc050`'s jump). Its inputs are the property (`EBP`), the property's +0x6c
/// `float` (the first stack word) and `00bbc050`'s other six stack words. Passes the context's
/// lights (count at +0x9, the array at +0xc) to `00b70de0` (index 0) and `00b71130` (the others;
/// for the context types 0x1c4 to 0x1cb at +0x4 only `00b71130` with light 0), each with the
/// `float`. Scales the light colour `011fa0d0` by `011f9184` (the setting `011f941e` and bit
/// 0x400 of the property's +0x20) and replaces it by the fog colour with `011f91a7`. Then the
/// light data header: below pass 0x1bd (1.0, the light count limited by `011ae60c` minus 1,
/// the material's +0x38, 1.0 or 0.0 with `011f9427`; the count's flag word 2n + 3), for pass
/// 0x1c3 (with no active lights: 0.5 / 1.0 in mode 10, else the ambient constant, into
/// `01201c80`; the word 2), else (0.0, 1.0 for the passes 0x1c4 to 0x1c7 else 0.0; the word 3).
/// The first `_ftol2` count (`01201c70`) of directional entries (none for the passes 0x1c4 to
/// 0x1c7) get their negated direction transformed by the matrix as a normal, normalised twice;
/// the second count (`01201c74`) of point entries after them get their position plus the camera
/// position transformed by the matrix as a point, and, without the flag word, their fourth word
/// divided by the transform's scale (+0x30), or, for the passes 0x1c4 and 0x1c5, the position
/// scaled by it. x87 in the exe's order.
pub fn fn_00bbc0a4(
    e: &mut Engine,
    property: Ptr,
    scale: u32,
    context: Ptr,
    pass: u32,
    material: Ptr,
    matrix: u32,
    flag: u32,
    transform: Ptr,
) {
    let context = context.addr();
    let kind = e.mem.u16(context + 4);
    let count = u32::from(e.mem.u8(context + 9));
    if (0x1c8..=0x1cb).contains(&kind) || (0x1c4..=0x1c7).contains(&kind) {
        let lights = e.mem.u32(context + 0xc);
        let light = e.mem.u32(lights);
        e.call(LIGHT_SET_00B71130, &args![0u32, light, scale]);
    } else {
        if (count as i32) > 0 {
            let lights = e.mem.u32(context + 0xc);
            let light = e.mem.u32(lights);
            e.call(LIGHT_SET_00B70DE0, &args![0u32, light, scale]);
        }
        let mut index = 1u32;
        while (index as i32) < (count as i32) {
            let lights = e.mem.u32(context + 0xc);
            let light = e.mem.u32(lights + 4 * index);
            e.call(LIGHT_SET_00B71130, &args![index, light, scale]);
            index += 1;
        }
    }
    if e.global::<u8>(ALWAYS_SCALE_011F941E) != 0 && e.mem.u32(property.addr() + 0x20) & 0x400 != 0
    {
        let color = [0u32, 4, 8, 0xc].map(|offset| e.global::<u32>(LIGHT_COLOR_011FA0D0 + offset));
        let factor = x87_load32(e.global::<u32>(LIGHT_SCALE_011F9184));
        let first = x87_store32(x87_mul(x87_load32(color[0]), factor));
        e.set_global::<u32>(LIGHT_COLOR_011FA0D0, first);
        let second = x87_store32(x87_mul(x87_load32(color[1]), factor));
        e.set_global::<u32>(LIGHT_COLOR_011FA0D0 + 4, second);
        let third = x87_store32(x87_mul(x87_load32(color[2]), factor));
        e.set_global::<u32>(LIGHT_COLOR_011FA0D0 + 8, third);
        let fourth = x87_store32(x87_mul(factor, x87_m32(color[3])));
        e.set_global::<u32>(LIGHT_COLOR_011FA0D0 + 0xc, fourth);
    }
    if e.global::<u8>(FOG_LIGHTS_011F91A7) != 0 {
        for index in 0..4u32 {
            let word = e.global::<u32>(FOG_COLOR_011A9BD0 + 4 * index);
            e.set_global::<u32>(LIGHT_COLOR_011FA0D0 + 4 * index, word);
        }
    }
    let data_enable = e.global::<u32>(LIGHT_DATA_ENABLE_012010E8);
    if pass < 0x1bd {
        let limited = if count != 0 {
            let limit = e.global::<i32>(LIGHT_LIMIT_011AE60C);
            let mut value = count as i32;
            if limit < value {
                value = limit;
            }
            (value.wrapping_sub(1) as u32) & 0xffff
        } else {
            0
        };
        e.set_global::<u32>(LIGHT_DATA_01201C70, 1.0f32.to_bits());
        let as_float = x87_store32(f64::from(limited as i32));
        e.set_global::<u32>(LIGHT_DATA_01201C70 + 4, as_float);
        let extra = x87_quiet_bits(e.mem.u32(material.addr() + 0x38));
        e.set_global::<u32>(LIGHT_DATA_01201C70 + 8, extra);
        let fourth = if e.global::<u8>(NO_FOURTH_011F9427) != 0 {
            0.0f32
        } else {
            1.0
        };
        e.set_global::<u32>(LIGHT_DATA_01201C70 + 0xc, fourth.to_bits());
        e.mem
            .set_u32(data_enable + 0x20, limited.wrapping_mul(2).wrapping_add(3));
    } else if pass == 0x1c3 {
        let active = e.call(ACTIVE_LIGHTS, &args![property]).u32() as u16;
        if active == 0 {
            if e.global::<u32>(MODE_011F91B8) == 0xa {
                let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
                for index in 0..3u32 {
                    e.set_global::<u32>(LIGHT_DATA_01201C70 + 0x10 + 4 * index, half);
                }
                e.set_global::<u32>(LIGHT_DATA_01201C70 + 0x1c, 1.0f32.to_bits());
            } else {
                for index in 0..4u32 {
                    let word = e.global::<u32>(AMBIENT_01201B30 + 4 * index);
                    e.set_global::<u32>(LIGHT_DATA_01201C70 + 0x10 + 4 * index, word);
                }
            }
        }
        let data_enable = e.global::<u32>(LIGHT_DATA_ENABLE_012010E8);
        e.mem.set_u32(data_enable + 0x20, 2);
    } else {
        e.mem.set_u32(data_enable + 0x20, 3);
        e.set_global::<u32>(LIGHT_DATA_01201C70, 0);
        let second = if pass.wrapping_sub(0x1c4) <= 3 {
            1.0f32
        } else {
            0.0
        };
        e.set_global::<u32>(LIGHT_DATA_01201C70 + 4, second.to_bits());
    }
    let directional = e
        .call(
            FTOL,
            &args![x87_load32(e.global::<u32>(LIGHT_DATA_01201C70))],
        )
        .u32();
    let points = e
        .call(
            FTOL,
            &args![x87_load32(e.global::<u32>(LIGHT_DATA_01201C70 + 4))],
        )
        .u32();
    let mut done = 0u32;
    e.with_stack(0x54, |e, frame| {
        let frame = frame.addr();
        let input = frame;
        let output = frame + 0x24;
        let unit = frame + 0xc;
        if !(0x1c4..=0x1c7).contains(&pass) && (directional as i32) > 0 {
            done = directional;
            for index in 0..directional {
                let entry = LIGHT_ENTRIES_01201C90 + 0x20 * index;
                for component in 0..3u32 {
                    let value = x87_load32(e.mem.u32(entry + 4 * component));
                    e.mem.set_u32(input + 4 * component, x87_store32(-value));
                }
                e.call(VEC3_TRANSFORM_NORMAL, &args![output, input, matrix]);
                e.call(VEC3_NORMALIZE, &args![input, output]);
                for component in 0..3u32 {
                    let word = x87_quiet_bits(e.mem.u32(input + 4 * component));
                    e.mem.set_u32(unit + 4 * component, word);
                }
                e.call(UNITIZE_004A0C10, &args![unit]);
                for component in 0..3u32 {
                    let word = x87_quiet_bits(e.mem.u32(unit + 4 * component));
                    e.mem.set_u32(entry + 4 * component, word);
                }
            }
        }
        if (points as i32) > 0 {
            let camera = e.global::<u32>(CAMERA_011F91C8);
            for index in 0..points {
                let entry = LIGHT_ENTRIES_01201C90 + 0x20 * (done + index);
                let words = [0u32, 4, 8, 0xc].map(|offset| e.mem.u32(entry + offset));
                let position = [0x1e4u32, 0x1e8, 0x1ec].map(|offset| e.mem.u32(camera + offset));
                let sums = [
                    x87_store32(x87_add(x87_load32(words[0]), x87_load32(position[0]))),
                    x87_store32(x87_add(x87_load32(words[1]), x87_m32(position[1]))),
                    x87_store32(x87_add(x87_load32(words[2]), x87_m32(position[2]))),
                ];
                for (component, word) in sums.iter().enumerate() {
                    e.mem.set_u32(unit + 4 * component as u32, *word);
                }
                e.call(VEC3_TRANSFORM_COORD, &args![output, unit, matrix]);
                let mut out = [0u32, 4, 8].map(|offset| x87_load32(e.mem.u32(output + offset)));
                let mut fourth = x87_load32(words[3]);
                if flag == 0 {
                    let radius_bits = e.mem.u32(transform.addr() + 0x30);
                    if pass == 0x1c4 || pass == 0x1c5 {
                        let radius = x87_load32(x87_quiet_bits(radius_bits));
                        out = [
                            x87_load32(x87_store32(x87_mul(out[0], radius))),
                            x87_load32(x87_store32(x87_mul(out[1], radius))),
                            x87_load32(x87_store32(x87_mul(radius, out[2]))),
                        ];
                    } else {
                        fourth = x87_load32(x87_store32(x87_div(fourth, x87_m32(radius_bits))));
                    }
                }
                for (component, value) in out.iter().enumerate() {
                    e.mem
                        .set_u32(entry + 4 * component as u32, x87_store32(*value));
                }
                e.mem.set_u32(entry + 0xc, x87_store32(fourth));
            }
        }
    });
}

/// The scene holder (`011f91e0`), the fallback for its +0x20 (`011f4288`), the light matrix
/// builder `00c4c2d0` (unnamed; `cdecl`: the transform, the scene word, the matrix to fill), the
/// renderer call on the shader's +0x18 (`00e6fbb0`, `thiscall`, `RET 8`), the lighting set-ups of
/// the passes 0xe / 0xf (`00bbdda0`, `RET 8`) and 0x1e0 / 0x1e1 (`00bbdcd0`, `RET 8`;
/// `00bbde20`, `RET 0xc`) (unnamed, unit `lighting30shader.cpp`), the refraction constant
/// (`01201350`, its flag object `012010ac`, the time `011f940c`, the `double`s 3600.0 at
/// `01012640` and 2 pi at `0104eea0`), the parallax constants (`01201ba0`, flag object
/// `012010dc`), `BSShaderManager::GetAccumulator` (engine map, `00b4f5c0`), the camera's
/// position call `00b71b40` (`thiscall`, `RET 4`), `NiPoint3::Dot` (engine map, `004b6190`,
/// `thiscall`, `RET 4`, the result in `ST0`), `D3DXMatrixInverse` (`00ee6dda`, `stdcall`: out,
/// determinant, matrix), the plane calls `00ee6dce` (`stdcall`: out, plane) and `00ee6dc8`
/// (`stdcall`: out, plane, matrix), the renderer (`011f4748`, its device at +0x288 and the
/// device's slot +0xdc taking an index and a plane) and the light's plane store `00b71ad0`
/// (unnamed; `thiscall`, `RET 4`).
const SCENE_HOLDER_011F91E0: u32 = 0x011f_91e0;
const DEFAULT_011F4288: u32 = 0x011f_4288;
const LIGHT_MATRIX_00C4C2D0: u32 = 0x00c4_c2d0;
const RENDERER_CALL_00E6FBB0: u32 = 0x00e6_fbb0;
const SET_UP_00BBDDA0: u32 = 0x00bb_dda0;
const SET_UP_00BBDCD0: u32 = 0x00bb_dcd0;
const SET_UP_00BBDE20: u32 = 0x00bb_de20;
const REFRACTION_01201350: u32 = 0x0120_1350;
const REFRACTION_ENABLE_012010AC: u32 = 0x0120_10ac;
const TIME_011F940C: u32 = 0x011f_940c;
const SECONDS_PER_HOUR_01012640: u32 = 0x0101_2640;
const TWO_PI_0104EEA0: u32 = 0x0104_eea0;
const PARALLAX_01201BA0: u32 = 0x0120_1ba0;
const PARALLAX_ENABLE_012010DC: u32 = 0x0120_10dc;
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const CAMERA_POSITION_00B71B40: u32 = 0x00b7_1b40;
const DOT: u32 = 0x004b_6190;
const MATRIX_INVERSE: u32 = 0x00ee_6dda;
const PLANE_CALL_00EE6DCE: u32 = 0x00ee_6dce;
const PLANE_CALL_00EE6DC8: u32 = 0x00ee_6dc8;
const RENDERER_011F4748: u32 = 0x011f_4748;
const STORE_PLANES_00B71AD0: u32 = 0x00b7_1ad0;

/// The clip planes of a shadow-casting light (`00bbef90`, passes 0x1c4 to 0x1c7): the
/// shader's light matrix (+0x980 of its +0x18 object) gets the camera position plus the origin
/// offset folded into its last row (minus its dot product with each of the first three columns,
/// `NiPoint3::Dot`), is multiplied by the matrix at +0x9c0, inverted and transposed; the light's
/// six planes (+0x144, the fourth word negated) are normalised and transformed by it (`00ee6dce`,
/// `00ee6dc8`), set on the device (slot +0xdc with the index) and stored on the light
/// (`00b71ad0`).
fn shadow_clip_planes(e: &mut Engine, this: u32, light: u32) {
    e.with_stack(0x200, |e, frame| {
        let frame = frame.addr();
        let position = frame + 0x50;
        let column = frame + 0x34;
        let point = frame + 0x24;
        let light_matrix = frame + 0x60;
        let product = frame + 0x140;
        let other = frame + 0x180;
        let inverse = frame + 0x1c0;
        let planes = frame + 0xe0;
        let renderer = e.mem.u32(this + 0x18);
        copy_words(e, light_matrix, renderer + 0x980, 16);
        let camera = e.global::<u32>(CAMERA_011F91C8);
        e.call(CAMERA_POSITION_00B71B40, &args![camera, position]);
        let sums = [0u32, 1, 2].map(|index| {
            let value = e.mem.u32(position + 4 * index);
            let origin = e.global::<u32>(ORIGIN_011F474C + 4 * index);
            x87_store32(x87_add(x87_load32(value), x87_m32(origin)))
        });
        for index in 0..3u32 {
            for (row, word) in [0u32, 4, 8].iter().enumerate() {
                let value = e.mem.u32(light_matrix + 4 * (index + word));
                let quiet = x87_quiet_bits(value);
                let (first, second) = if index == 0 {
                    (column, point)
                } else {
                    (point, column)
                };
                e.mem.set_u32(first + 4 * row as u32, quiet);
                e.mem
                    .set_u32(second + 4 * row as u32, x87_quiet_bits(sums[row]));
            }
            let (target, argument) = if index == 0 {
                (point, column)
            } else {
                (column, point)
            };
            let dot = e.call(DOT, &args![target, argument]).f64();
            e.mem
                .set_u32(light_matrix + 0x30 + 4 * index, x87_store32(-dot));
        }
        copy_words(e, other, renderer + 0x9c0, 16);
        e.call(MATRIX_MULTIPLY, &args![product, light_matrix, other]);
        e.call(MATRIX_INVERSE, &args![inverse, 0u32, product]);
        e.call(MATRIX_TRANSPOSE, &args![product, inverse]);
        for index in 0..6u32 {
            let source = light + 0x144 + 0x10 * index;
            let words = [0u32, 4, 8].map(|offset| e.mem.u32(source + offset));
            for (component, word) in words.iter().enumerate() {
                e.mem
                    .set_u32(point + 4 * component as u32, x87_quiet_bits(*word));
            }
            let distance = x87_load32(e.mem.u32(source + 0xc));
            e.mem.set_u32(point + 0xc, x87_store32(-distance));
            e.call(PLANE_CALL_00EE6DCE, &args![point, point]);
            e.call(PLANE_CALL_00EE6DC8, &args![column, point, product]);
            for component in 0..4u32 {
                let word = x87_quiet_bits(e.mem.u32(column + 4 * component));
                e.mem.set_u32(planes + 0x10 * index + 4 * component, word);
            }
        }
        let device_holder = e.global::<u32>(RENDERER_011F4748) + 0x288;
        for index in 0..6u32 {
            let device = e.mem.u32(device_holder);
            e.vcall(device, 0xdc, &args![index, planes + 0x10 * index]);
        }
        e.call(STORE_PLANES_00B71AD0, &args![light, planes]);
    });
}

/// The light's stored planes set on the device (`00bbef90` when the light or the accumulator
/// refuses the computed ones): the six planes at +0x1a8.
fn stored_clip_planes(e: &mut Engine, light: u32) {
    let device_holder = e.global::<u32>(RENDERER_011F4748) + 0x288;
    for index in 0..6u32 {
        let device = e.mem.u32(device_holder);
        e.vcall(device, 0xdc, &args![index, light + 0x1a8 + 0x10 * index]);
    }
}

// Translated from 00bbef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`; the vtable `010ba0f8`, slot +0x7c):
/// the lighting 3.0 shader's geometry set-up, `thiscall` with one stack word (the geometry's
/// set-up record: +0 a flags object, +0x8 the material, +0xc the property, +0x10 another flags
/// object; a 16-byte aligned frame). With the scene object `O` of `011f91e0`: for the passes
/// 0x1d3 to 0x1d6 with flags bit 0, `SetZWriteEnable(0, 1)`; the light matrix (`00c4c2d0`) and
/// `00e6fbb0` on the shader's +0x18 with `O` + 0x68. For the passes 0x1e0 and 0x1e1:
/// `00bbdcd0`, `00bbde20` (with `O`) and the light position (`00bbb890`). Otherwise
/// (`00bbdda0` first for the passes 0xe and 0xf): the lights (`00bbc050`), the view matrix
/// (`00bbe6b0`), the world matrix (`00bbba00`), the refraction constant (the time / 3600 * 2 pi
/// times the property's integer +0xe4), the eye position (`00bbbcf0`), the light position
/// (`00bbb890`), the emittance (`00bbbda0`), the material (`00bbe9c0`), the colour
/// (`00bbecd0`), the glow (`00bbbdf0`), the parallax (+0xe8, +0xec), the alpha (`00bbbe30`)
/// and the fog (`00bbbe80`); for the passes 0x1c4 to 0x1c7 the shadow clip planes of the first light (computed, or the light's stored
/// ones when its byte +0x208 or the accumulator's +0x164 / +0x165 says so; none when the
/// accumulator's bytes are set). Then the constant maps of the render state's +0x34 and +0x48
/// (when the shader has the maps +0x30 / +0x34 and the state its +0x44 / +0x5c) get their slot
/// +0x78 with eleven words, and the object at +0x1c its slot +0x24 with the record's +0x10
/// flags object when it has bit 0.
pub fn fn_00bbef90(e: &mut Engine, this: Ptr, record: Ptr) {
    let base = this.addr();
    let record = record.addr();
    let holder = e.global::<u32>(SCENE_HOLDER_011F91E0);
    let scene = e.mem.u32(holder);
    let pass = e.global::<u32>(PASS_011F91E4);
    let mut fallback = e.mem.u32(scene + 0x20);
    let transform = scene + 0x68;
    if fallback == 0 {
        fallback = DEFAULT_011F4288;
    }
    let camera = e.mem.u32(scene + 0xb8);
    let camera_word = e.mem.u32(camera + 0x34);
    let scene_word = e.mem.u32(scene + 0xbc);
    let property = e.mem.u32(record + 0xc);
    let material = e.mem.u32(record + 8);
    if pass.wrapping_sub(0x1d3) <= 3 {
        let flags = e.mem.u32(record);
        if e.mem.u8(flags + 0x18) & 1 != 0 {
            e.call(SET_Z_WRITE_ENABLE, &args![0u32, 1u32]);
        }
    }
    e.with_stack(0x40, |e, matrix| {
        let matrix = matrix.addr();
        e.call(LIGHT_MATRIX_00C4C2D0, &args![transform, scene_word, matrix]);
        let target = e.mem.u32(base + 0x18);
        e.call(RENDERER_CALL_00E6FBB0, &args![target, transform, 0u32]);
        let matrix_words: [u32; 16] =
            core::array::from_fn(|index| e.mem.u32(matrix + 4 * index as u32));
        if pass == 0x1e0 || pass == 0x1e1 {
            let state = e.global::<u32>(RENDER_STATE_0126F74C);
            e.call(SET_UP_00BBDCD0, &args![base, property, state]);
            let state = e.global::<u32>(RENDER_STATE_0126F74C);
            e.call(SET_UP_00BBDE20, &args![base, scene, property, state]);
            fn_00bbb890(e, this, matrix_words, Ptr::new(transform));
            return;
        }
        if pass == 0xe || pass == 0xf {
            let state = e.global::<u32>(RENDER_STATE_0126F74C);
            e.call(SET_UP_00BBDDA0, &args![base, property, state]);
        }
        fn_00bbc050(
            e,
            this,
            Ptr::new(property),
            Ptr::new(holder),
            pass,
            Ptr::new(material),
            matrix,
            scene_word,
            Ptr::new(transform),
        );
        fn_00bbe6b0(
            e,
            this,
            Ptr::new(holder),
            pass,
            scene_word,
            Ptr::new(transform),
        );
        fn_00bbba00(e, this, scene_word, Ptr::new(transform));
        let enable = e.global::<u32>(REFRACTION_ENABLE_012010AC);
        if e.mem.u8(enable + 8) != 0 {
            let time = x87_load32(e.global::<u32>(TIME_011F940C));
            let hours = x87_div(
                time,
                f64::from_bits(e.global::<u64>(SECONDS_PER_HOUR_01012640)),
            );
            let angle = x87_mul(hours, f64::from_bits(e.global::<u64>(TWO_PI_0104EEA0)));
            let count = e.mem.u32(property + 0xe4) as i32;
            let value = x87_store32(x87_mul(angle, f64::from(count)));
            e.set_global::<u32>(REFRACTION_01201350, value);
        }
        fn_00bbbcf0(e, this, Ptr::new(fallback));
        let matrix_words: [u32; 16] =
            core::array::from_fn(|index| e.mem.u32(matrix + 4 * index as u32));
        fn_00bbb890(e, this, matrix_words, Ptr::new(transform));
        fn_00bbbda0(e, this, Ptr::new(property));
        fn_00bbe9c0(
            e,
            this,
            fallback,
            Ptr::new(property),
            Ptr::new(record),
            pass,
        );
        fn_00bbecd0(e, this, Ptr::new(material), Ptr::new(property));
        fn_00bbbdf0(e, this, Ptr::new(property));
        let enable = e.global::<u32>(PARALLAX_ENABLE_012010DC);
        if e.mem.u8(enable + 8) != 0 {
            for index in 0..2u32 {
                let word = x87_quiet_bits(e.mem.u32(property + 0xe8 + 4 * index));
                e.set_global::<u32>(PARALLAX_01201BA0 + 4 * index, word);
            }
        }
        fn_00bbbe30(e, Ptr::new(property));
        fn_00bbbe80(e, this);
        if pass.wrapping_sub(0x1c4) <= 3 {
            let holder = e.global::<u32>(SCENE_HOLDER_011F91E0);
            let lights = e.mem.u32(holder + 0xc);
            let light = e.mem.u32(lights);
            let refused = e.mem.u8(light + 0x208) != 0 || {
                let accumulator = e.call(GET_ACCUMULATOR, &args![]).u32();
                e.mem.u8(accumulator + 0x164) != 0 || {
                    let accumulator = e.call(GET_ACCUMULATOR, &args![]).u32();
                    e.mem.u8(accumulator + 0x165) != 0
                }
            };
            if !refused {
                shadow_clip_planes(e, base, light);
            } else {
                let accumulator = e.call(GET_ACCUMULATOR, &args![]).u32();
                if e.mem.u8(accumulator + 0x164) == 0 {
                    let accumulator = e.call(GET_ACCUMULATOR, &args![]).u32();
                    if e.mem.u8(accumulator + 0x165) == 0 {
                        stored_clip_planes(e, light);
                    }
                }
            }
        }
    });
    let words = |shader: u32| {
        args![
            shader,
            scene,
            scene_word,
            0u32,
            camera_word,
            record,
            0u32,
            transform,
            fallback,
            0u32,
            1u32
        ]
    };
    let state = e.global::<u32>(RENDER_STATE_0126F74C);
    if e.mem.u32(base + 0x30) != 0 {
        let shader = e.mem.u32(state + 0x44);
        if shader != 0 {
            let map = e.mem.u32(state + 0x34);
            e.vcall(map, 0x78, &words(shader));
        }
    }
    let state = e.global::<u32>(RENDER_STATE_0126F74C);
    if e.mem.u32(base + 0x34) != 0 {
        let shader = e.mem.u32(state + 0x5c);
        if shader != 0 {
            let map = e.mem.u32(state + 0x48);
            e.vcall(map, 0x78, &words(shader));
        }
    }
    let other = e.mem.u32(record + 0x10);
    if other != 0 && e.mem.u8(other + 0x18) & 1 != 0 {
        let target = e.mem.u32(base + 0x1c);
        e.vcall(target, 0x24, &args![other]);
    }
}

/// One loader frame's source blocks: block `index` at `first` + 0x4c * `index` gets its explicit
/// words (the HLSL file, the macro name / value pairs, the null name that ends them), the rest of
/// the 0x4c bytes cleared by `memset` (in block order, as the exe calls it; a block written in
/// full gets no `memset`).
fn write_sources(e: &mut Engine, first: u32, sources: &[&[u32]]) {
    for (index, words) in sources.iter().enumerate() {
        let block = first + 0x4c * index as u32;
        for (slot, word) in words.iter().enumerate() {
            e.mem.set_u32(block + 4 * slot as u32, *word);
        }
        let used = 4 * words.len() as u32;
        if used < 0x4c {
            e.call(MEMSET, &args![block + used, 0u32, 0x4c - used]);
        }
    }
}

/// The vertex loader's callees and strings: the vertex profile `00b4f3e0` (unnamed, unit
/// `bsshadermanager.cpp`; no arguments), `NiPointer::operator=` as a function (`00a5f860`,
/// `thiscall` on the pointer, `RET 4`), the shader model 3 setting (`011f94a4`) with
/// `"vs_3_0"` (`010a93dc`), the name format `"SM3%03i.vso"` (`010b9f78`) and the shaders'
/// `NiPointer`s from `01200dd8`.
const VERTEX_PROFILE_00B4F3E0: u32 = 0x00b4_f3e0;
const NI_POINTER_ASSIGN_00A5F860: u32 = 0x00a5_f860;
const SHADER_MODEL_3_011F94A4: u32 = 0x011f_94a4;
const PROFILE_VS_3_0_010A93DC: u32 = 0x010a_93dc;
const VERTEX_NAME_FORMAT_010B9F78: u32 = 0x010b_9f78;
const VERTEX_SHADERS_01200DD8: u32 = 0x0120_0dd8;

/// The 57 source blocks of `Lighting30Shader::LoadVertexShaders` (read from its stack set-up):
/// the files `"lighting\3x\SM3Lighting.v.hlsl"` (`010b9ff4`), `"lighting\2x\v\EnvMap.v.hlsl"`
/// (`010aeae0`), `"lighting\2x\v\WaterFog.v.hlsl"` (`010ae93c`),
/// `"lighting\3x\SM3SimpleShadow.v.hlsl"` (`010b9fc0`), `"lighting\3x\SM3DepthMap.v.hlsl"`
/// (`010b9fa0`), `"lighting\2x\v\renderNormals.v.hlsl"` (`010aea2c`),
/// `"lighting\2x\v\renderDepth.v.hlsl"` (`010ae9f8`), `"lighting\2x\v\renderVelocity.v.hlsl"`
/// (`010ae9d4`), `"lighting\2x\v\localMap.v.hlsl"` (`010ae9ac`),
/// `"lighting\1x\v\texEffect.v.hlsl"` (`010ae98c`) and `"lighting\3x\SM3ZOnly.v.hlsl"`
/// (`010b9f84`); the macros `"SKIN"` (`010aec10`), `"VC"` (`010aed64`), `"PROJSHADOW"`
/// (`010b9fe8`), `"SM3"` (`010b9fe4`), `"ENVMAP"` (`010aead8`), `"EYE"` (`010aead4`),
/// `"TEXCOORD"` (`010ae930`), `"FIRE"` (`010aea24`), `"CLEAR"` (`010aea1c`), with the value `""`
/// (`01011584`) or a null one.
const VERTEX_SOURCES: [&[u32]; 57] = [
    &[0x010b_9ff4, 0],
    &[0x010b_9ff4, 0],
    &[0x010b_9ff4, 0x010a_ec10, 0x0101_1584, 0],
    &[0x010b_9ff4, 0x010a_ed64, 0x0101_1584, 0],
    &[0x010b_9ff4, 0x010a_ed64, 0x0101_1584, 0],
    &[
        0x010b_9ff4,
        0x010a_ec10,
        0x0101_1584,
        0x010a_ed64,
        0x0101_1584,
        0,
    ],
    &[0x010b_9ff4, 0x010b_9fe8, 0x0101_1584, 0],
    &[0x010b_9ff4, 0x010b_9fe8, 0x0101_1584, 0],
    &[
        0x010b_9ff4,
        0x010a_ec10,
        0x0101_1584,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_9ff4,
        0x010a_ed64,
        0x0101_1584,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_9ff4,
        0x010a_ed64,
        0x0101_1584,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_9ff4,
        0x010a_ec10,
        0x0101_1584,
        0x010a_ed64,
        0x0101_1584,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[0x010a_eae0, 0x010b_9fe4, 0x0101_1584, 0x010a_ead8, 0, 0],
    &[
        0x010a_eae0,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_ead8,
        0,
        0x010a_ed64,
        0x0101_1584,
        0,
    ],
    &[
        0x010a_eae0,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_ead8,
        0,
        0x010a_ec10,
        0x0101_1584,
        0,
    ],
    &[
        0x010a_eae0,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_ead8,
        0,
        0x010a_ec10,
        0x0101_1584,
        0x010a_ed64,
        0x0101_1584,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    &[
        0x010a_eae0,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_ead8,
        0,
        0x010a_ead4,
        0,
        0,
    ],
    &[0x010a_e93c, 0x010b_9fe4, 0x0101_1584, 0],
    &[
        0x010a_e93c,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_ec10,
        0x0101_1584,
        0,
    ],
    &[
        0x010a_e93c,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_e930,
        0x0101_1584,
        0,
    ],
    &[
        0x010a_e93c,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_ec10,
        0x0101_1584,
        0x010a_e930,
        0x0101_1584,
        0,
    ],
    &[
        0x010a_e93c,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_ed64,
        0x0101_1584,
        0,
    ],
    &[
        0x010a_e93c,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_ec10,
        0x0101_1584,
        0x010a_ed64,
        0x0101_1584,
        0,
    ],
    &[
        0x010a_e93c,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_e930,
        0x0101_1584,
        0x010a_ed64,
        0x0101_1584,
        0,
    ],
    &[
        0x010a_e93c,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_ec10,
        0x0101_1584,
        0x010a_e930,
        0x0101_1584,
        0x010a_ed64,
        0x0101_1584,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    &[0x010b_9fc0, 0],
    &[0x010b_9fc0, 0x010a_ed64, 0, 0],
    &[0x010b_9fc0, 0x010a_ec10, 0x0101_1584, 0],
    &[0x010b_9fc0, 0x010a_ec10, 0x0101_1584, 0x010a_ed64, 0, 0],
    &[0x010b_9fa0, 0],
    &[0x010b_9fa0, 0x010a_ed64, 0x0101_1584, 0],
    &[0x010b_9fa0, 0x010a_ec10, 0x0101_1584, 0],
    &[
        0x010b_9fa0,
        0x010a_ed64,
        0x0101_1584,
        0x010a_ec10,
        0x0101_1584,
        0,
    ],
    &[0x010a_ea2c, 0x010b_9fe4, 0, 0],
    &[0x010a_ea2c, 0x010b_9fe4, 0, 0x010a_ec10, 0x0101_1584, 0],
    &[0x010a_ea2c, 0x010b_9fe4, 0, 0x010a_ea24, 0x0101_1584, 0],
    &[0x010a_ea2c, 0x010b_9fe4, 0, 0x010a_ea1c, 0x0101_1584, 0],
    &[
        0x010a_ea2c,
        0x010b_9fe4,
        0,
        0x010a_ea1c,
        0x0101_1584,
        0x010a_ec10,
        0x0101_1584,
        0,
    ],
    &[0x010a_e9f8, 0x010b_9fe4, 0, 0],
    &[0x010a_e9f8, 0x010b_9fe4, 0, 0x010a_ed64, 0, 0],
    &[0x010a_e9f8, 0x010b_9fe4, 0, 0x010a_ec10, 0x0101_1584, 0],
    &[
        0x010a_e9f8,
        0x010b_9fe4,
        0,
        0x010a_ec10,
        0x0101_1584,
        0x010a_ed64,
        0,
        0,
    ],
    &[0x010a_e9d4, 0x010b_9fe4, 0, 0],
    &[0x010a_e9d4, 0x010b_9fe4, 0, 0x010a_ec10, 0x0101_1584, 0],
    &[0x010a_e9ac, 0x010b_9fe4, 0, 0],
    &[0x010a_e9ac, 0x010b_9fe4, 0, 0x010a_ec10, 0x0101_1584, 0],
    &[0x010a_e9ac, 0x010b_9fe4, 0, 0x010a_ea1c, 0x0101_1584, 0],
    &[0x010a_e98c, 0],
    &[0x010a_e98c, 0x010a_ec10, 0x0101_1584, 0],
    &[0x010b_9f84, 0],
    &[0x010b_9f84, 0x010a_ec10, 0x0101_1584, 0],
    &[0x010b_9f84, 0x010a_ed64, 0x0101_1584, 0],
    &[
        0x010b_9f84,
        0x010a_ed64,
        0x0101_1584,
        0x010a_ec10,
        0x0101_1584,
        0,
    ],
    &[0x010b_9f84, 0],
    &[0x010b_9f84, 0x010a_ec10, 0x0101_1584, 0],
    &[0x010b_9f84, 0],
    &[0x010b_9f84, 0x010a_ec10, 0x0101_1584, 0],
];

/// The 50 source blocks of `Lighting30Shader::LoadPixelShaders` (read from its stack set-up; the
/// macro values of the blocks 29 to 31 that point at the shadow map size string are set after
/// them).
const PIXEL_SOURCES: [&[u32]; 50] = [
    &[0x010b_a0d4, 0x010b_a0c8, 0x0106_3ee8, 0],
    &[0x010b_a0d4, 0x010b_a0c8, 0x0106_3ef4, 0x010a_eb8c, 0, 0],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x0101_c61c,
        0x0106_3f48,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3f00,
        0x0101_c61c,
        0x0106_3f48,
        0x010a_eb8c,
        0,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0bc,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0b0,
        0x010b_a0ac,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0bc,
        0x0101_1584,
        0x010a_eb8c,
        0,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0b0,
        0x010b_a0ac,
        0x010a_eb8c,
        0,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010a_ed54,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010a_ed54,
        0x0101_1584,
        0x010a_eb8c,
        0,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0a4,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0a4,
        0x0101_1584,
        0x010a_eb8c,
        0,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010a_eb8c,
        0,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x0101_c61c,
        0x0106_3f48,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3f00,
        0x0101_c61c,
        0x0106_3f48,
        0x010a_eb8c,
        0,
        0x010b_9fe8,
        0x0101_1584,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0bc,
        0x0101_1584,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0b0,
        0x010b_a0ac,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0bc,
        0x0101_1584,
        0x010a_eb8c,
        0,
        0x010b_9fe8,
        0x0101_1584,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0b0,
        0x010b_a0ac,
        0x010a_eb8c,
        0,
        0x010b_9fe8,
        0x0101_1584,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010a_ed54,
        0x0101_1584,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010a_ed54,
        0x0101_1584,
        0x010a_eb8c,
        0,
        0x010b_9fe8,
        0x0101_1584,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0a4,
        0x0101_1584,
        0x010b_9fe8,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3ef4,
        0x010b_a0a4,
        0x0101_1584,
        0x010a_eb8c,
        0,
        0x010b_9fe8,
        0x0101_1584,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    &[0x010a_f054, 0x010b_9fe4, 0x0101_1584, 0],
    &[0x010a_f054, 0x010b_9fe4, 0x0101_1584, 0x010a_f04c, 0, 0],
    &[0x010a_f054, 0x010b_9fe4, 0x0101_1584, 0x010a_ead4, 0, 0],
    &[0x010a_ef10, 0x010b_9fe4, 0x0101_1584, 0],
    &[
        0x010a_ef10,
        0x010b_9fe4,
        0x0101_1584,
        0x010a_ef00,
        0x0101_1584,
        0,
    ],
    &[
        0x010b_a080,
        0x010b_a074,
        0x0106_3f24,
        0x010a_ee24,
        0x010b_a070,
        0x010a_efe0,
        0x0106_3f30,
        0,
    ],
    &[0x010b_a050, 0x010a_efe0, 0x0106_3f30, 0],
    &[0x010b_a050, 0x010a_efe0, 0x0106_3f30, 0],
    &[0x010a_efbc, 0x010b_9fe4, 0, 0],
    &[0x010a_efbc, 0x010b_9fe4, 0, 0],
    &[0x010a_efbc, 0x010b_9fe4, 0, 0x010a_ea24, 0x0101_1584, 0],
    &[0x010a_efbc, 0x010b_9fe4, 0, 0x010a_ea1c, 0x0101_1584, 0],
    &[0x010a_efbc, 0x010b_9fe4, 0, 0x010a_ea1c, 0x0101_1584, 0],
    &[
        0x010b_a0d4,
        0x010b_a0c8,
        0x0106_3f48,
        0x010a_eea4,
        0x0101_1584,
        0,
    ],
    &[0x010a_ef98, 0x010b_9fe4, 0, 0],
    &[0x010a_ef74, 0x010b_9fe4, 0, 0],
    &[0x010a_ef54, 0x010b_9fe4, 0, 0],
    &[0x010a_ef54, 0x010b_9fe4, 0, 0x010a_ea1c, 0, 0],
    &[0x010a_ef34, 0x010a_ef30, 0x0101_1584, 0],
    &[0x010a_ef34, 0x010a_ef30, 0x0101_1584, 0],
    &[0x010b_a034, 0],
    &[0x010b_a034, 0],
    &[0x010b_a034, 0],
    &[0x010b_a034, 0],
    &[0x010b_a034, 0x010a_f130, 0x0101_1584, 0],
    &[0x010b_a034, 0x010a_f130, 0x0101_1584, 0],
];

// Translated from 00bb96f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Lighting30Shader::LoadVertexShaders` (engine map), `thiscall`, no stack words. Writes the 57
/// source blocks on its stack (`VERTEX_SOURCES`), asks `00b4f3e0` for the vertex profile, then
/// for each block: `MakePath`, the name `"SM3%03i.vso"` with the index, and
/// `CompileVertexShaderHLSL` into the `NiPointer` at `01200dd8` + 4 * index. The blocks 1, 4, 7
/// and 10 compile with `"vs_3_0"` when the setting `011f94a4` is set and assign inline; the
/// others assign through `00a5f860`. The stack probe `__alloca_probe` (`00ec8660`) is the
/// [`Engine::with_stack`] frame.
// Translation incomplete: 00bb96f0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn lighting30_shader_load_vertex_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x1318, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x18, base);
        write_sources(e, frame + 0x20, &VERTEX_SOURCES);
        let profile = e.call(VERTEX_PROFILE_00B4F3E0, &args![]).u32();
        e.mem.set_u32(frame + 0x1c, profile);
        let path = frame + 0x110c;
        let name = frame + 0x1210;
        for index in 0..57u32 {
            let block = frame + 0x20 + 0x4c * index;
            let file = e.mem.u32(block);
            e.call(MAKE_PATH, &args![file, path]);
            e.call(
                FORMAT_00406D00,
                &args![name, 0x104u32, VERTEX_NAME_FORMAT_010B9F78, index],
            );
            let slot = VERTEX_SHADERS_01200DD8 + 4 * index;
            if matches!(index, 1 | 4 | 7 | 10) {
                let profile = if e.global::<u8>(SHADER_MODEL_3_011F94A4) != 0 {
                    PROFILE_VS_3_0_010A93DC
                } else {
                    e.mem.u32(frame + 0x1c)
                };
                let shader = e
                    .call(
                        COMPILE_VERTEX_SHADER,
                        &args![base, path, block + 4, profile, name],
                    )
                    .u32();
                ni_pointer_assign(e, slot, shader);
            } else {
                let profile = e.mem.u32(frame + 0x1c);
                let shader = e
                    .call(
                        COMPILE_VERTEX_SHADER,
                        &args![base, path, block + 4, profile, name],
                    )
                    .u32();
                e.call(NI_POINTER_ASSIGN_00A5F860, &args![slot, shader]);
            }
        }
    });
}

/// The pixel loader's callees and strings: the pixel profile `00b4f380` (unnamed; `cdecl`, one
/// word), the renderer mode `004dc060` (unnamed, unit `renderer.cpp`; `cdecl`, one word), CRT
/// `sprintf` (`00ec623a`) with `"%i"` (`01020774`) and the shadow map size setting (`011f91b0`),
/// the string `"20"` (`010ba030`), `"ps_3_0"` (`010a93b4`), the name formats `"SM3%03i.pso"`
/// (`010ba024`) and `"SM3LL%03i.pso"` (`010ba014`), and the shaders' `NiPointer`s from
/// `01200c48` and `01200d10`.
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
const RENDERER_MODE_004DC060: u32 = 0x004d_c060;
const SPRINTF: u32 = 0x00ec_623a;
const FORMAT_INT_01020774: u32 = 0x0102_0774;
const SHADOW_SIZE_011F91B0: u32 = 0x011f_91b0;
const TWENTY_010BA030: u32 = 0x010b_a030;
const PROFILE_PS_3_0_010A93B4: u32 = 0x010a_93b4;
const PIXEL_NAME_FORMAT_010BA024: u32 = 0x010b_a024;
const LOW_LIGHT_NAME_FORMAT_010BA014: u32 = 0x010b_a014;
const PIXEL_SHADERS_01200C48: u32 = 0x0120_0c48;
const LOW_LIGHT_SHADERS_01200D10: u32 = 0x0120_0d10;

/// The pixel blocks 5, 7, 0x11 and 0x13: they take `"ps_3_0"` with the setting `011f94a4`, else
/// the `"20"` value for their second macro.
fn shader_model_3_block(index: u32) -> bool {
    matches!(index, 5 | 7 | 0x11 | 0x13)
}

// Translated from 00bba5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Lighting30Shader::LoadPixelShaders` (engine map), `thiscall`, no stack words. Writes the 50
/// source blocks on its stack (`PIXEL_SOURCES`), asks `00b4f380` (with 0) for the pixel profile,
/// builds the macro value strings `"20"`, `"2"`, `"4"` and the shadow map size (`sprintf` of
/// `011f91b0` with `"%i"`, pointed to by three blocks' macros), then for each block: `MakePath`,
/// the name `"SM3%03i.pso"` and, by the renderer mode (`004dc060` with 0) and the index, the
/// first macro value `"4"` (blocks below 0x18 in modes 5 to 7), the second `"2"` (blocks 2, 3,
/// 0xe and 0xf in mode 7) or `"20"` (`shader_model_3_block` without the setting), `"ps_3_0"`
/// for those blocks with the setting `011f94a4`; `CompilePixelShaderHLSL` into the `NiPointer`
/// at `01200c48` + 4 * index (assigned inline in modes 5 to 7 except for the other blocks of
/// modes 5 and 6, else through `00a5f860`). The blocks below 0x18 also compile a low-light
/// variant (`"SM3LL%03i.pso"`, the first macro value `"2"`) into `01200d10` + 4 * index. The
/// stack probe is the [`Engine::with_stack`] frame.
// Translation incomplete: 00bba5d0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn lighting30_shader_load_pixel_shaders(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x1110, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x14, base);
        write_sources(e, frame + 0x2c, &PIXEL_SOURCES);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![0u32]).u32();
        let twenty_chars = e.global::<u16>(TWENTY_010BA030);
        e.mem.set_u16(frame + 0x24, 0x34);
        let size = e.global::<u32>(SHADOW_SIZE_011F91B0);
        e.mem.set_u16(frame + 0x1c, twenty_chars);
        e.mem.set_u16(frame + 0x20, 0x32);
        let end = e.global::<u8>(TWENTY_010BA030 + 2);
        e.mem.set_u32(frame + 0x28, profile);
        e.mem.set_u8(frame + 0x22, 0);
        e.mem.set_u8(frame + 0x26, 0);
        e.mem.set_u8(frame + 0x1e, end);
        e.call(SPRINTF, &args![frame + 0x18, FORMAT_INT_01020774, size]);
        for offset in [0x8e0u32, 0x91c, 0x968] {
            e.mem.set_u32(frame + offset, frame + 0x18);
        }
        let path = frame + 0x1008;
        let name = frame + 0xf04;
        let twenty = frame + 0x1c;
        let two = frame + 0x20;
        let four = frame + 0x24;
        for index in 0..0x32u32 {
            let macros = frame + 0x30 + 0x4c * index;
            let profile = e.mem.u32(frame + 0x28);
            let file = e.mem.u32(macros - 4);
            e.call(MAKE_PATH, &args![file, path]);
            e.call(
                FORMAT_00406D00,
                &args![name, 0x104u32, PIXEL_NAME_FORMAT_010BA024, index],
            );
            let mode = e.call(RENDERER_MODE_004DC060, &args![0u32]).u32();
            let slot = PIXEL_SHADERS_01200C48 + 4 * index;
            let flagged = e.global::<u8>(SHADER_MODEL_3_011F94A4) != 0;
            let (chosen, inline) = if mode == 7 {
                if index < 0x18 {
                    e.mem.set_u32(macros + 4, four);
                }
                if matches!(index, 2 | 3 | 0xe | 0xf) {
                    e.mem.set_u32(macros + 0xc, two);
                }
                (profile, true)
            } else if mode == 5 || mode == 6 {
                if index < 0x18 {
                    e.mem.set_u32(macros + 4, four);
                }
                if shader_model_3_block(index) {
                    if flagged {
                        (PROFILE_PS_3_0_010A93B4, true)
                    } else {
                        e.mem.set_u32(macros + 0xc, twenty);
                        (profile, true)
                    }
                } else {
                    (profile, false)
                }
            } else if shader_model_3_block(index) {
                if flagged {
                    (PROFILE_PS_3_0_010A93B4, false)
                } else {
                    e.mem.set_u32(macros + 0xc, twenty);
                    (profile, false)
                }
            } else {
                (profile, false)
            };
            let shader = e
                .call(
                    COMPILE_PIXEL_SHADER,
                    &args![base, path, macros, chosen, name],
                )
                .u32();
            if inline {
                ni_pointer_assign(e, slot, shader);
            } else {
                e.call(NI_POINTER_ASSIGN_00A5F860, &args![slot, shader]);
            }
            if index < 0x18 {
                e.call(
                    FORMAT_00406D00,
                    &args![name, 0x104u32, LOW_LIGHT_NAME_FORMAT_010BA014, index],
                );
                e.mem.set_u32(macros + 4, two);
                let slot = LOW_LIGHT_SHADERS_01200D10 + 4 * index;
                let profile = e.mem.u32(frame + 0x28);
                if shader_model_3_block(index) {
                    let chosen = if e.global::<u8>(SHADER_MODEL_3_011F94A4) != 0 {
                        PROFILE_PS_3_0_010A93B4
                    } else {
                        e.mem.set_u32(macros + 0xc, twenty);
                        profile
                    };
                    let shader = e
                        .call(
                            COMPILE_PIXEL_SHADER,
                            &args![base, path, macros, chosen, name],
                        )
                        .u32();
                    ni_pointer_assign(e, slot, shader);
                } else {
                    let shader = e
                        .call(
                            COMPILE_PIXEL_SHADER,
                            &args![base, path, macros, profile, name],
                        )
                        .u32();
                    e.call(NI_POINTER_ASSIGN_00A5F860, &args![slot, shader]);
                }
            }
        }
    });
}

/// The byte that lets `Lighting30Shader::CreateConstantMaps` make the shared vertex map once
/// (`01201f28`).
const VERTEX_MAP_MADE_01201F28: u32 = 0x0120_1f28;

/// The vertex map's entries in the exe's order: the eight words of a predefined entry (vtable
/// slot +0x68) or the ten of an entry (slot +0x8), and the shared `NiPointer` that takes the
/// map's slot +0x70 answer for the entry's name (through `00a5f860`). Names (read in Ghidra):
/// `"WorldViewProjTranspose"` (`010a1e50`), `"SkinWorldViewProjTranspose"` (`010a1d20`),
/// `"WorldViewTranspose"` (`010a1ee0`), `"SkinWorldViewTranspose"` (`010a1db8`),
/// `"BoneMatrix3"` (`010a1cac`), `"PrevBoneMatrix3"` (`010af580`), `"ObjToCubeMap"` (`010ba32c`),
/// `"EyePos"` (`010a18a4`), `"WorldEyePos"` (`010ba320`), `"Fog"` (`01013cf4`), `"DepthData"`
/// (`010af5b0`), `"Refraction"` (`01031334`), `"WorldBoundCenter"` (`010ba30c`),
/// `"ShadowProjectionData"` (`010ba2f4`), `"PrevWorldViewPT"` (`010af5a0`), `"PrevWorldViewT"`
/// (`010af590`) and `"SkinToCubeMap"` (`010ba2e4`).
const VERTEX_PREDEFINED: [([u32; 8], u32); 4] = [
    (
        [0x010a_1e50, 0x2000_0009, 0, 0x0101_1584, 0, 0, 0, 0],
        0x0120_1080,
    ),
    (
        [0x010a_1d20, 0x2000_0009, 0x14, 0x0101_1584, 0, 0, 0, 0],
        0x0120_1084,
    ),
    (
        [0x010a_1ee0, 0x2000_0009, 4, 0x0101_1584, 0, 0, 0, 0],
        0x0120_1088,
    ),
    (
        [0x010a_1db8, 0x2000_0009, 0x18, 0x0101_1584, 0, 0, 0, 0],
        0x0120_108c,
    ),
];
const VERTEX_ENTRIES: [([u32; 10], u32); 13] = [
    (
        [
            0x010a_1cac,
            0x2000_0009,
            0x12_0000,
            0x34,
            0x36,
            0,
            0,
            0,
            0,
            0,
        ],
        0x0120_1090,
    ),
    (
        [
            0x010a_f580,
            0x1000_0009,
            0,
            0x6c,
            0x36,
            0x0101_1584,
            0x360,
            4,
            0x011f_a580,
            0,
        ],
        0x0120_1094,
    ),
    (
        [
            0x010b_a32c,
            0x1000_0009,
            0,
            8,
            4,
            0x0101_1584,
            0x40,
            4,
            0x0120_1170,
            0,
        ],
        0x0120_1098,
    ),
    (
        [
            0x010a_18a4,
            0x1000_0007,
            0,
            0x28,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1370,
            0,
        ],
        0x0120_10a4,
    ),
    (
        [
            0x010b_a320,
            0x1000_0007,
            0,
            0x2e,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_13d0,
            0,
        ],
        0x0120_10c0,
    ),
    (
        [
            0x0101_3cf4,
            0x1000_000c,
            0,
            0x2c,
            2,
            0x0101_1584,
            0x20,
            4,
            0x0120_13b0,
            0,
        ],
        0x0120_10bc,
    ),
    (
        [
            0x010a_f5b0,
            0x1000_0007,
            0,
            0x29,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1380,
            0,
        ],
        0x0120_10a8,
    ),
    (
        [
            0x0103_1334,
            0x1000_0007,
            0,
            0x26,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1350,
            0,
        ],
        0x0120_10ac,
    ),
    (
        [
            0x010b_a30c,
            0x1000_0007,
            0,
            0x27,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1360,
            0,
        ],
        0x0120_10b0,
    ),
    (
        [
            0x010b_a2f4,
            0x1000_0009,
            0,
            0x20,
            6,
            0x0101_1584,
            0x60,
            4,
            0x0120_12f0,
            0,
        ],
        0x0120_10b4,
    ),
    (
        [
            0x010a_f5a0,
            0x1000_0009,
            0,
            0xc,
            4,
            0x0101_1584,
            0x40,
            4,
            0x011f_ed08,
            0,
        ],
        0x0120_109c,
    ),
    (
        [
            0x010a_f590,
            0x1000_0009,
            0,
            0x10,
            4,
            0x0101_1584,
            0x40,
            4,
            0x011f_ed48,
            0,
        ],
        0x0120_10a0,
    ),
    (
        [
            0x010b_a2e4,
            0x1000_0009,
            0,
            0x1c,
            4,
            0x0101_1584,
            0x40,
            4,
            0x0120_12b0,
            0,
        ],
        0x0120_10b8,
    ),
];

/// The pixel map's entries (slot +0x8) and the flag object `NiPointer` that takes the slot
/// +0x70 answer (assigned inline): `"ambientcolor"` (`010ba2d4`), `"eyepos"` (`010aff58`),
/// `"hairtint"` (`010ba2c8`), `"matalpha"` (`010ba2bc`), `"matemit"` (`010ba2b4`),
/// `"refractpower"` (`010ba2a4`), `"parallaxdata"` (`010ba294`), `"waterfog"` (`010ba288`),
/// `"envmaptoggles"` (`010ba278`) and `"lightdata"` (`010ba26c`).
const PIXEL_ENTRIES: [([u32; 10], u32); 10] = [
    (
        [
            0x010b_a2d4,
            0x1000_0007,
            0,
            0,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1b30,
            0,
        ],
        0x0120_10c4,
    ),
    (
        [
            0x010a_ff58,
            0x1000_0007,
            0,
            1,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1b40,
            0,
        ],
        0x0120_10c8,
    ),
    (
        [
            0x010b_a2c8,
            0x1000_0007,
            0,
            2,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1b50,
            0,
        ],
        0x0120_10cc,
    ),
    (
        [
            0x010b_a2bc,
            0x1000_0007,
            0,
            3,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1b60,
            0,
        ],
        0x0120_10d0,
    ),
    (
        [
            0x010b_a2b4,
            0x1000_0007,
            0,
            4,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1b70,
            0,
        ],
        0x0120_10d4,
    ),
    (
        [
            0x010b_a2a4,
            0x1000_0007,
            0,
            6,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1b90,
            0,
        ],
        0x0120_10d8,
    ),
    (
        [
            0x010b_a294,
            0x1000_0007,
            0,
            7,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1ba0,
            0,
        ],
        0x0120_10dc,
    ),
    (
        [
            0x010b_a288,
            0x1000_000c,
            0,
            9,
            2,
            0x0101_1584,
            0x20,
            4,
            0x0120_1bc0,
            0,
        ],
        0x0120_10e4,
    ),
    (
        [
            0x010b_a278,
            0x1000_0007,
            0,
            0xb,
            1,
            0x0101_1584,
            0x10,
            4,
            0x0120_1be0,
            0,
        ],
        0x0120_10e0,
    ),
    (
        [
            0x010b_a26c,
            0x1000_0009,
            0,
            0x14,
            0x29,
            0x0101_1584,
            0x290,
            4,
            0x0120_1c70,
            0,
        ],
        0x0120_10e8,
    ),
];

/// The entries of the maps at +0x70 (two predefined, then seven: `"BoneMatrix3"`,
/// `"EyePosition"` (`010af5d8`), `"U Offset"` (`010af498`), `"V Offset"` (`010af48c`),
/// `"InvBoundDiameter"` (`010ba258`), `"FogParam"` (`010af474`), `"FogColor"` (`010a16ec`)) and
/// +0x6c (`"Fill Color"` (`010af468`), `"Rim Color"` (`010af45c`), `"Vars"` (`010ba250`)).
const SECOND_VERTEX_PREDEFINED: [[u32; 8]; 2] = [
    [0x010a_1e50, 0x2000_0009, 0, 0x0101_1584, 0, 0, 0, 0],
    [0x010a_1d20, 0x2000_0009, 4, 0x0101_1584, 0, 0, 0, 0],
];
const SECOND_VERTEX_ENTRIES: [[u32; 10]; 7] = [
    [
        0x010a_1cac,
        0x2000_0009,
        0x12_0000,
        0xe,
        0x36,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        0x010a_f5d8,
        0x1000_0007,
        0,
        8,
        1,
        0x0101_1584,
        0x10,
        4,
        0x0120_1370,
        0,
    ],
    [
        0x010a_f498,
        0x1000_0004,
        0,
        9,
        1,
        0x0101_1584,
        4,
        4,
        0x0120_0be8,
        0,
    ],
    [
        0x010a_f48c,
        0x1000_0004,
        0,
        0xa,
        1,
        0x0101_1584,
        4,
        4,
        0x0120_0bec,
        0,
    ],
    [
        0x010b_a258,
        0x1000_0004,
        0,
        0xb,
        1,
        0x0101_1584,
        4,
        4,
        0x0120_0bf0,
        0,
    ],
    [
        0x010a_f474,
        0x1000_0007,
        0,
        0xc,
        1,
        0x0101_1584,
        0x10,
        4,
        0x0120_0c38,
        0,
    ],
    [
        0x010a_16ec,
        0x1000_0007,
        0,
        0xd,
        1,
        0x0101_1584,
        0x10,
        4,
        0x0120_0c28,
        0,
    ],
];
const SECOND_PIXEL_ENTRIES: [[u32; 10]; 3] = [
    [
        0x010a_f468,
        0x1000_0007,
        0,
        0,
        1,
        0x0101_1584,
        0x10,
        4,
        0x0120_0bf8,
        0,
    ],
    [
        0x010a_f45c,
        0x1000_0007,
        0,
        1,
        1,
        0x0101_1584,
        0x10,
        4,
        0x0120_0c08,
        0,
    ],
    [
        0x010b_a250,
        0x1000_0007,
        0,
        2,
        1,
        0x0101_1584,
        0x10,
        4,
        0x0120_0c18,
        0,
    ],
];

/// The map at `slot`'s slot +0x70 answer for the entry `name`.
fn lookup_entry(e: &mut Engine, slot: u32, name: u32) -> u32 {
    let map = e.mem.u32(slot);
    e.vcall(map, 0x70, &args![name]).u32()
}

/// A new constant map (0x44 bytes from `00aa13e0`, the constructor with the word at
/// `renderer_word` and the program type; a null block gives a null map) assigned to `slot`
/// through `00a5f860`.
fn make_map_through_assign(e: &mut Engine, slot: u32, renderer_word: u32, program: u32) {
    let block = e.call(CONSTANT_MAP_NI_ALLOCATE, &args![0x44u32]).u32();
    let map = if block != 0 {
        let renderer = e.mem.u32(renderer_word);
        e.call(CONSTANT_MAP_CONSTRUCTOR, &args![block, renderer, program])
            .u32()
    } else {
        0
    };
    e.call(NI_POINTER_ASSIGN_00A5F860, &args![slot, map]);
}

/// The Ni allocation and the map constructor (`00aa13e0`, `00b7e330`).
const CONSTANT_MAP_NI_ALLOCATE: u32 = 0x00aa_13e0;
const CONSTANT_MAP_CONSTRUCTOR: u32 = 0x00b7_e330;

// Translated from 00bbf5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Lighting30Shader::CreateConstantMaps` (engine map), `thiscall`, no stack words. Once (the
/// byte `01201f28`, while the `NiPointer` at +0x34 is null): a vertex map (program type 0, the
/// word at +0x18 as the renderer) through `00a5f860` into +0x34 with `VERTEX_PREDEFINED` and
/// `VERTEX_ENTRIES`, each entry's handle (slot +0x70) kept in its shared `NiPointer`, then a new
/// empty vertex map in its place. With +0x30 null: a pixel map (program type 2, assigned inline)
/// with `PIXEL_ENTRIES` (handles into the flag objects `012010c4` to `012010e8`), then a new
/// empty one. Then +0x78 takes +0x34 and +0x74 takes +0x30; with +0x70 null a vertex map with
/// two predefined entries and `SECOND_VERTEX_ENTRIES`; with +0x6c null a pixel map with
/// `SECOND_PIXEL_ENTRIES`.
// Translation incomplete: 00bbf5a0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5 for the allocations) is not translated.
pub fn lighting30_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let renderer_word = base + 0x18;
    let vertex_slot = base + 0x34;
    if e.mem.u32(vertex_slot) == 0 && e.global::<u8>(VERTEX_MAP_MADE_01201F28) == 0 {
        e.set_global::<u8>(VERTEX_MAP_MADE_01201F28, 1);
        make_map_through_assign(e, vertex_slot, renderer_word, 0);
        for (words, handle) in VERTEX_PREDEFINED {
            add_predefined_entry(e, vertex_slot, &words);
            let entry = lookup_entry(e, vertex_slot, words[0]);
            e.call(NI_POINTER_ASSIGN_00A5F860, &args![handle, entry]);
        }
        for (words, handle) in VERTEX_ENTRIES {
            add_entry(e, vertex_slot, &words);
            let entry = lookup_entry(e, vertex_slot, words[0]);
            e.call(NI_POINTER_ASSIGN_00A5F860, &args![handle, entry]);
        }
        make_map_through_assign(e, vertex_slot, renderer_word, 0);
    }
    let pixel_slot = base + 0x30;
    if e.mem.u32(pixel_slot) == 0 {
        make_constant_map(e, pixel_slot, renderer_word, 2);
        for (words, handle) in PIXEL_ENTRIES {
            add_entry(e, pixel_slot, &words);
            let entry = lookup_entry(e, pixel_slot, words[0]);
            ni_pointer_assign(e, handle, entry);
        }
        make_constant_map(e, pixel_slot, renderer_word, 2);
    }
    let vertex = e.mem.u32(vertex_slot);
    ni_pointer_assign(e, base + 0x78, vertex);
    let pixel = e.mem.u32(pixel_slot);
    ni_pointer_assign(e, base + 0x74, pixel);
    let second_vertex = base + 0x70;
    if e.mem.u32(second_vertex) == 0 {
        make_constant_map(e, second_vertex, renderer_word, 0);
        for words in SECOND_VERTEX_PREDEFINED {
            add_predefined_entry(e, second_vertex, &words);
        }
        for words in SECOND_VERTEX_ENTRIES {
            add_entry(e, second_vertex, &words);
        }
    }
    let second_pixel = base + 0x6c;
    if e.mem.u32(second_pixel) == 0 {
        make_constant_map(e, second_pixel, renderer_word, 2);
        for words in SECOND_PIXEL_ENTRIES {
            add_entry(e, second_pixel, &words);
        }
    }
}

// Translated from 00bbb7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall`, no stack words: gives
/// each of the 0x70 passes at `01200ec0` to `01201080` to `this`'s vtable slot +0x128.
pub fn fn_00bbb7f0(e: &mut Engine, this: Ptr) {
    let mut at = PASSES_01200EC0;
    while (at as i32) < 0x0120_1080 {
        let pass = e.global::<u32>(at);
        e.vcall(this.addr(), 0x128, &args![pass]);
        at += 4;
    }
}

/// The pass picker `00c0bd40` (unnamed; `thiscall`, two words) and `00e7eed0` (unnamed;
/// `thiscall` on the pass's +0x30, no stack words).
const PICK_PASS_00C0BD40: u32 = 0x00c0_bd40;
const STATES_00E7EED0: u32 = 0x00e7_eed0;

// Translated from 00bbb820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 81 of the vtable `010ba0f8`, `thiscall`, two
/// words: the pass `00c0bd40` picks for them (on `this`) becomes current (`0126f74c`), then
/// `00e7eed0` on its +0x30; what that answers in EAX, EDX and ST0 is this function's result.
pub fn fn_00bbb820(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ret {
    let pass = e
        .call(PICK_PASS_00C0BD40, &args![this, first, second])
        .u32();
    e.set_global(RENDER_STATE_0126F74C, pass);
    let states = e.mem.u32(pass + 0x30);
    e.call(STATES_00E7EED0, &args![states])
}

/// The uniform-form entry of [`fn_00bbb820`]: `this` and the two words; the last call's
/// registers pass through.
fn fn_00bbb820_entry(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00bbb820(e, Ptr::new(a[0]), a[1], a[2])
}

// Translated from 00bbc610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall` (`this` not read), two
/// words: clears the texture word (+8) of each stage of `pass` (stage count +0x18, array +0x24)
/// from `first` on.
pub fn fn_00bbc610(e: &mut Engine, _this: Ptr, pass: Ptr, first: u32) {
    let pass = pass.addr();
    let mut index = first;
    while index < e.mem.u32(pass + 0x18) {
        let stage = e.mem.u32(e.mem.u32(pass + 0x24).wrapping_add(4 * index));
        if stage != 0 {
            e.mem.set_u32(stage + 8, 0);
        }
        index += 1;
    }
}

/// The skinning object (`01201090`), the renderer (`011f9508`) and its `00e6fe30` (unnamed;
/// `thiscall`, five words), and the name `"BoneMatrix3"` (`010a1cac`).
const SKIN_01201090: u32 = 0x0120_1090;
const RENDERER_CALL_00E6FE30: u32 = 0x00e6_fe30;
const NAME_BONE_MATRIX3_010A1CAC: u32 = 0x010a_1cac;

// Translated from 00bbd000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall`, eight words of which
/// the second (a geometry: +0x20, +0x28), third (skin data: +4, the u16 +0x20) and seventh are
/// read. With skin data, a null +0x38 and either the skinning object `01201090` with its byte
/// +8 set or +0x34 equal to +0x70: the renderer's `00e6fe30(geometry, seventh, 0, 3, 1)`; then
/// the current pass's (`0126f74c`) vertex shader (+0x5c) gets its slot +0x78 with an entry,
/// the geometry's +0x28, the u16, the geometry's +0x20 and the skin data's +4. The entry is
/// the skinning object, or with +0x34 equal to +0x70 what the map at +0x70 answers for
/// `"BoneMatrix3"` (its slot +0x70, `NiD3DShaderConstantMap::GetEntry`'s slot, one word: the
/// other four words were pushed before and stay for slot +0x78). Answers 0.
pub fn fn_00bbd000(
    e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    geometry: Ptr,
    skin: Ptr,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    seventh: u32,
    _unused_7: u32,
) -> u32 {
    let base = this.addr();
    if skin.is_null() || e.mem.u32(base + 0x38) != 0 {
        return 0;
    }
    let object = e.global::<u32>(SKIN_01201090);
    let skinning = object != 0 && e.mem.u8(object + 8) != 0;
    if !skinning && e.mem.u32(base + 0x34) != e.mem.u32(base + 0x70) {
        return 0;
    }
    let renderer = e.global::<u32>(RENDERER_011F9508);
    e.call(
        RENDERER_CALL_00E6FE30,
        &args![renderer, geometry, seventh, 0u32, 3u32, 1u32],
    );
    let map = e.mem.u32(base + 0x70);
    let geometry = geometry.addr();
    let word_28 = e.mem.u32(geometry + 0x28);
    let word_20 = e.mem.u32(geometry + 0x20);
    let bones = u32::from(e.mem.u16(skin.addr() + 0x20));
    let skin_word = e.mem.u32(skin.addr() + 4);
    let pass = e.global::<u32>(RENDER_STATE_0126F74C);
    let shader = e.mem.u32(pass + 0x5c);
    let entry = if e.mem.u32(base + 0x34) == map {
        e.vcall(map, 0x70, &args![NAME_BONE_MATRIX3_010A1CAC]).u32()
    } else {
        e.global::<u32>(SKIN_01201090)
    };
    e.vcall(
        shader,
        0x78,
        &args![entry, word_28, bones, word_20, skin_word],
    );
    0
}

/// `BSShaderManager::GetShader` (engine map, `00b55560`; `cdecl`, the shader index), the
/// `InterlockedIncrement`/`InterlockedDecrement` import slots, the masks per pass index
/// (`01200a28`, `01200868`), the flag bytes they set (`01200854` to `01200864`, `01200848` to
/// `01200851`), the setting byte `011f941c` and the render-state helpers.
const GET_SHADER_00B55560: u32 = 0x00b5_5560;
const MASKS_01200A28: u32 = 0x0120_0a28;
const MASKS_01200868: u32 = 0x0120_0868;
const FLAGS_01200854: u32 = 0x0120_0854;
const FLAGS_01200848: u32 = 0x0120_0848;
const SETTING_011F941C: u32 = 0x011f_941c;

/// The `NiPointer` at `object + to` set from `object + from` when they differ (old one
/// released: count +4, slot +4 at 0; new one held).
fn copy_shader_pointer(e: &mut Engine, object: u32, to: u32, from: u32) {
    let old = e.mem.u32(object + to);
    if old == e.mem.u32(object + from) {
        return;
    }
    if old != 0 {
        let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 4]).u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    let new = e.mem.u32(object + from);
    e.mem.set_u32(object + to, new);
    if new != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![new + 4]);
    }
}

// Translated from 00bbd880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `cdecl`, one word (the pass kind).
/// With shader 0x1d (`GetShader`) and kind 0x1e0 or 0x1e1, its constant maps +0x30 and +0x34
/// select +0x6c and +0x70 (`00c663d0`) and nothing more. Otherwise its `NiPointer`s +0x30 and
/// +0x34 take +0x74 and +0x78, and the kind picks a pass index (10..15: 1..6, 0x2da: 0x68,
/// 0x2db: 0x69, 0x2e0/0x2e1: 0x6a/0x6b, 0x2e6/0x2e7: 0x6c/0x6d, 0x2ec/0x2ed: 0x6e/0x6f, any
/// other: kind - 0x178; the compiler's jump tables as `match` arms). For the 17 shaders at
/// `01201080` the byte +8 (when set) and the bytes `01200854 + i` become bit `i + 1` of the
/// index's mask at `01200a28`; for the 10 objects at `012010c4` and the bytes `01200848 + j`
/// bit `j + 1` of the mask at `01200868`. Last the index's pass (`01200ec0`) gets render
/// state 0xa8: 3 for kinds 0x1d3..0x1d6, 0xf for 0x1cc..0x1df, 7 for 0x179..0x1e1, else 0 (and
/// 0 for kinds 10..15 or with bit 0 of `011f941c` set).
pub fn fn_00bbd880(e: &mut Engine, kind: u32) {
    let shader = e.call(GET_SHADER_00B55560, &args![0x1du32]).u32();
    if shader != 0 && (kind == 0x1e0 || kind == 0x1e1) {
        e.call(SELECT_MAP_00C663D0, &args![shader + 0x30, shader + 0x6c]);
        e.call(SELECT_MAP_00C663D0, &args![shader + 0x34, shader + 0x70]);
        return;
    }
    copy_shader_pointer(e, shader, 0x30, 0x74);
    copy_shader_pointer(e, shader, 0x34, 0x78);
    let index = match kind {
        10 => 1,
        11 => 2,
        12 => 3,
        13 => 4,
        14 => 5,
        15 => 6,
        0x2da => 0x68,
        0x2db => 0x69,
        0x2e0 => 0x6a,
        0x2e1 => 0x6b,
        0x2e6 => 0x6c,
        0x2e7 => 0x6d,
        0x2ec => 0x6e,
        0x2ed => 0x6f,
        _ => kind.wrapping_sub(0x178),
    };
    let mut bit = 1u32;
    for i in 0..0x11u32 {
        let object = e.global::<u32>(SHADERS_01201080 + 4 * i);
        if object != 0 {
            let mask = e.global::<u32>(MASKS_01200A28.wrapping_add(index.wrapping_mul(4)));
            e.mem
                .set_u8(object + 8, u8::from(mask & bit.rotate_left(1) != 0));
        }
        bit = bit.rotate_left(1);
        let mask = e.global::<u32>(MASKS_01200A28.wrapping_add(index.wrapping_mul(4)));
        e.set_global(FLAGS_01200854 + i, u8::from(mask & bit != 0));
    }
    for j in 1..=10u32 {
        let object = e.global::<u32>(FLAGS_012010C4 + 4 * (j - 1));
        if object != 0 {
            let mask = e.global::<u32>(MASKS_01200868.wrapping_add(index.wrapping_mul(4)));
            e.mem.set_u8(object + 8, (mask >> j) as u8 & 1);
        }
        let mask = e.global::<u32>(MASKS_01200868.wrapping_add(index.wrapping_mul(4)));
        e.set_global(FLAGS_01200848 + (j - 1), (mask >> j) as u8 & 1);
    }
    let value = if !(10..=15).contains(&kind) && e.global::<u8>(SETTING_011F941C) & 1 == 0 {
        if kind.wrapping_sub(0x1d3) <= 3 {
            3
        } else if kind.wrapping_sub(0x1cc) <= 0x13 {
            0xf
        } else if kind.wrapping_sub(0x179) <= 0x68 {
            7
        } else {
            0
        }
    } else {
        0
    };
    let pass = e.global::<u32>(PASSES_01200EC0.wrapping_add(index.wrapping_mul(4)));
    pass_render_state(e, pass, 0xa8, value);
}

/// The stage function `00e7de90` (unnamed; `thiscall` on a stage, one word), the index map
/// `00ba5060` (unit `run_00ba3e30`, `cdecl`, one word), the render-state setters `00b97ff0`
/// (three words) and `00b97f20` (three words) (unit `bsrenderstate_xenon.cpp`, `cdecl`) and
/// the default textures `011f9510`, `011f9518` and `011f952c`.
const STAGE_SET_00E7DE90: u32 = 0x00e7_de90;
const INDEX_MAP_00BA5060: u32 = 0x00ba_5060;
const RENDER_STATE_00B97FF0: u32 = 0x00b9_7ff0;
const RENDER_STATE_00B97F20: u32 = 0x00b9_7f20;
const TEXTURE_011F9510: u32 = 0x011f_9510;
const TEXTURE_011F9518: u32 = 0x011f_9518;
const TEXTURE_011F952C: u32 = 0x011f_952c;

// Translated from 00bbdcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall`, two words (a geometry,
/// a pass). With the geometry's object +0xdc (O): stage 0 takes the texture of the geometry's
/// `+0xb0` (when null `011f9518` with bit 7 of the geometry's +0x20, else `011f9510`), stage 1
/// O's +8 (`011f952c` when null), each with `00e7de90(3)`; stages 2 on are cleared; then
/// `00b97ff0` with the index maps (`00ba5060`) of O's +0x5c and +0x60 and 0, and `this`'s
/// slot +0x108.
pub fn fn_00bbdcd0(e: &mut Engine, this: Ptr, geometry: Ptr, pass: Ptr) {
    let geometry = geometry.addr();
    let object = e.mem.u32(geometry + 0xdc);
    if object == 0 {
        return;
    }
    let pass = pass.addr();
    let stage = e.mem.u32(e.mem.u32(pass + 0x24));
    let mut texture = e.mem.u32(e.mem.u32(geometry + 0xb0));
    if texture == 0 {
        texture = if e.mem.u32(geometry + 0x20) & 0x80 != 0 {
            e.global::<u32>(TEXTURE_011F9518)
        } else {
            e.global::<u32>(TEXTURE_011F9510)
        };
    }
    e.mem.set_u32(stage + 8, texture);
    e.call(STAGE_SET_00E7DE90, &args![stage, 3u32]);
    let own = e.mem.u32(object + 8);
    let stage = e.mem.u32(e.mem.u32(pass + 0x24) + 4);
    let texture = if own != 0 {
        own
    } else {
        e.global::<u32>(TEXTURE_011F952C)
    };
    e.mem.set_u32(stage + 8, texture);
    e.call(STAGE_SET_00E7DE90, &args![stage, 3u32]);
    fn_00bbc610(e, Ptr::NULL, Ptr::new(pass), 2);
    let second = e.mem.u32(object + 0x60);
    let second = e.call(INDEX_MAP_00BA5060, &args![second]).u32();
    let first = e.mem.u32(object + 0x5c);
    let first = e.call(INDEX_MAP_00BA5060, &args![first]).u32();
    e.call(RENDER_STATE_00B97FF0, &args![first, second, 0u32]);
    e.vcall(this.addr(), 0x108, &args![]);
}

// Translated from 00bbdda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`), `thiscall`, two words (a geometry,
/// a pass). With the geometry's object +0xdc (O): stage 0 takes O's +0x6c (`011f952c` when
/// null) with `00e7de90(3)`, stages 1 on are cleared, `this`'s slot +0x108 runs and
/// `00b97f20(4, O's +0x70, 0)`.
pub fn fn_00bbdda0(e: &mut Engine, this: Ptr, geometry: Ptr, pass: Ptr) {
    let object = e.mem.u32(geometry.addr() + 0xdc);
    if object == 0 {
        return;
    }
    let pass = pass.addr();
    let own = e.mem.u32(object + 0x6c);
    let stage = e.mem.u32(e.mem.u32(pass + 0x24));
    let texture = if own != 0 {
        own
    } else {
        e.global::<u32>(TEXTURE_011F952C)
    };
    e.mem.set_u32(stage + 8, texture);
    e.call(STAGE_SET_00E7DE90, &args![stage, 3u32]);
    fn_00bbc610(e, Ptr::NULL, Ptr::new(pass), 1);
    e.vcall(this.addr(), 0x108, &args![]);
    let value = e.mem.u32(object + 0x70);
    e.call(RENDER_STATE_00B97F20, &args![4u32, value, 0u32]);
}

/// What `00bbde20` writes: the light constants `01200be8` to `01200c1c` and the fog constants
/// `01200c28` to `01200c44`; `BSShaderManager::GetFogProperty` (engine map, `00b55520`,
/// `cdecl`, the fog kind byte `011f91c4`).
const LIGHT_01200BE8: u32 = 0x0120_0be8;
const LIGHT_01200BF8: u32 = 0x0120_0bf8;
const LIGHT_01200C18: u32 = 0x0120_0c18;
const FOG_01200C28: u32 = 0x0120_0c28;
const GET_FOG_PROPERTY_00B55520: u32 = 0x00b5_5520;
const FOG_KIND_011F91C4: u32 = 0x011f_91c4;

// Translated from 00bbde20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable `010ba0f8`'s slot at `010ba15c`), `thiscall`, three words
/// (a flag word, a geometry, one not read). With a geometry: its object +0xdc, when set, gives
/// the eight words +0xc to +0x28 to `01200bf8`, the `float`s +0x4c, +0x50 to `01200be8` and
/// +0x54 to `01200c18`, 1.0 to `01200c1c`; the fog constants `01200c28` to `01200c44` are
/// cleared; with the flag word and the fog property (kind `011f91c4`) whose end (+0x30) or
/// start (+0x2c) is not 0 (`FUCOM`; NaN counts as not 0) they become (colour +0x20 to +0x28, 0)
/// and (end, end - start, power +0x60, 0).
pub fn fn_00bbde20(e: &mut Engine, _this: Ptr, flag: u32, geometry: Ptr, _unused_2: u32) {
    if geometry.is_null() {
        return;
    }
    let object = e.mem.u32(geometry.addr() + 0xdc);
    if object != 0 {
        for index in 0..8u32 {
            let word = e.mem.u32(object + 0xc + 4 * index);
            e.set_global(LIGHT_01200BF8 + 4 * index, word);
        }
        for (offset, target) in [
            (0x4cu32, LIGHT_01200BE8),
            (0x50, LIGHT_01200BE8 + 4),
            (0x54, LIGHT_01200C18),
        ] {
            let word = x87_quiet_bits(e.mem.u32(object + offset));
            e.set_global(target, word);
        }
        e.set_global(LIGHT_01200C18 + 4, x87_store32(1.0));
    }
    for index in 0..8u32 {
        e.set_global(FOG_01200C28 + 4 * index, x87_store32(0.0));
    }
    if flag == 0 {
        return;
    }
    let kind = u32::from(e.global::<u8>(FOG_KIND_011F91C4));
    let fog = e.call(GET_FOG_PROPERTY_00B55520, &args![kind]).u32();
    if fog == 0 {
        return;
    }
    let start = x87_load32(e.mem.u32(fog + 0x2c));
    let end = x87_load32(e.mem.u32(fog + 0x30));
    let power = x87_quiet_bits(e.mem.u32(fog + 0x60));
    if end == 0.0 && start == 0.0 {
        return;
    }
    let colour = [0x20u32, 0x24, 0x28].map(|offset| e.mem.u32(fog + offset));
    let words = [
        x87_store32(end),
        x87_store32(x87_sub(end, start)),
        power,
        x87_store32(0.0),
    ];
    for (index, word) in words.iter().enumerate() {
        e.set_global(FOG_01200C28 + 0x10 + 4 * index as u32, *word);
    }
    for (index, word) in colour.iter().enumerate() {
        e.set_global(FOG_01200C28 + 4 * index as u32, x87_quiet_bits(*word));
    }
    e.set_global(FOG_01200C28 + 0xc, x87_store32(0.0));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_b7f0, fn_00bbb7f0(Ptr)),
        (0x00bb_b820, fn_00bbb820_entry as AbiFn),
        entry!(0x00bb_c610, fn_00bbc610(Ptr, Ptr, u32)),
        entry!(
            0x00bb_d000,
            fn_00bbd000(Ptr, u32, Ptr, Ptr, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00bb_d880, fn_00bbd880(u32)),
        entry!(0x00bb_dcd0, fn_00bbdcd0(Ptr, Ptr, Ptr)),
        entry!(0x00bb_dda0, fn_00bbdda0(Ptr, Ptr, Ptr)),
        entry!(0x00bb_de20, fn_00bbde20(Ptr, u32, Ptr, u32)),
        entry!(0x00bb_c640, fn_00bbc640(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00bb_d0d0, fn_00bbd0d0() -> Ptr),
        entry!(0x00bb_b790, fn_00bbb790(Ptr, u8)),
        entry!(0x00bb_b840, fn_00bbb840()),
        entry!(0x00bb_bda0, fn_00bbbda0(Ptr, Ptr)),
        entry!(0x00bb_bdf0, fn_00bbbdf0(Ptr, Ptr)),
        entry!(0x00bb_be30, fn_00bbbe30(Ptr)),
        entry!(0x00bb_ef60, fn_00bbef60(Ptr, u32) -> Ptr),
        entry!(0x00bb_c9b0, fn_00bbc9b0(Ptr) -> u32),
        entry!(0x00bb_d820, fn_00bbd820(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00bb_c5c0, fn_00bbc5c0(Ptr, Ptr) -> u32),
        entry!(0x00bb_e020, fn_00bbe020(Ptr, u32)),
        entry!(0x00bb_e120, fn_00bbe120(Ptr, u32)),
        entry!(0x00c0_cf10, fn_00c0cf10(Ptr, u32, u32)),
        entry!(0x00bb_e650, fn_00bbe650(Ptr, Ptr)),
        entry!(0x00bb_eef0, fn_00bbeef0(Ptr, Ptr)),
        (0x00bb_b890, fn_00bbb890_entry as AbiFn),
        entry!(0x00bb_bcf0, fn_00bbbcf0(Ptr, Ptr)),
        entry!(0x00bb_ba00, fn_00bbba00(Ptr, u32, Ptr)),
        entry!(0x00bb_be80, fn_00bbbe80(Ptr)),
        entry!(0x00bb_ce70, fn_00bbce70(Ptr) -> bool),
        entry!(0x00bb_c9c0, fn_00bbc9c0(Ptr)),
        entry!(0x00bb_e6b0, fn_00bbe6b0(Ptr, Ptr, u32, u32, Ptr)),
        entry!(0x00bb_e9c0, fn_00bbe9c0(Ptr, u32, Ptr, Ptr, u32)),
        entry!(0x00bb_ecd0, fn_00bbecd0(Ptr, Ptr, Ptr)),
        entry!(
            0x00bb_c050,
            fn_00bbc050(Ptr, Ptr, Ptr, u32, Ptr, u32, u32, Ptr)
        ),
        entry!(
            0x00bb_c0a4,
            fn_00bbc0a4(Ptr, u32, Ptr, u32, Ptr, u32, u32, Ptr)
        ),
        entry!(0x00bb_ef90, fn_00bbef90(Ptr, Ptr)),
        entry!(0x00bb_96f0, lighting30_shader_load_vertex_shaders(Ptr)),
        entry!(0x00bb_a5d0, lighting30_shader_load_pixel_shaders(Ptr)),
        entry!(0x00bb_f5a0, lighting30_shader_create_constant_maps(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    fn entries(declaration: u32, list: &[[u32; 5]]) -> Vec<(u32, Vec<u32>)> {
        list.iter()
            .map(|entry| {
                let mut words = vec![declaration];
                words.extend_from_slice(entry);
                (declaration_slot(0x8c), words)
            })
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

    #[test]
    fn fn_00bbd0d0_makes_eight_declarations_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BBC640, &[]);
        counting(&mut e);
        let (shader, log) = run(&mut e, 0x00bb_d0d0);
        assert_eq!(shader, SHADER);
        let d = |n: u32| DECLARATION_0 + 0x100 * n;
        let mut want: Vec<(u32, Vec<u32>)> = [6, 6, 8, 8, 6, 6, 8, 8]
            .iter()
            .map(|&n| (DECLARATION_CREATE, vec![RENDERER, n, 1]))
            .collect();
        for (first, second, list) in [
            (0u32, 1u32, &FIVE_ENTRIES[..]),
            (4, 5, &SIX_ENTRIES[..]),
            (2, 3, &SEVEN_ENTRIES[..]),
            (6, 7, &EIGHT_ENTRIES[..]),
        ] {
            want.extend(entries(d(first), list));
            want.extend(entries(d(second), list));
            want.push((declaration_slot(0xb4), vec![d(first)]));
            want.push((declaration_slot(0xb4), vec![d(second)]));
        }
        want.push((NI_ALLOCATE, vec![0xa4]));
        want.push((
            SHADER_CONSTRUCT_00BBC640,
            vec![SHADER, d(0), d(2), d(4), d(6)],
        ));
        want.push((shader_slot(0x4c), vec![SHADER]));
        for n in [1, 5, 3, 7] {
            want.push((IMPORT_INTERLOCKED_INCREMENT, vec![d(n) + 4]));
        }
        for slot in [0x148, 0x11c, 0x154] {
            want.push((shader_slot(slot), vec![SHADER]));
        }
        want.push((shader_slot(0xc0), vec![SHADER, d(0)]));
        assert_eq!(log, want);
        assert_eq!(e.mem.u32(SHADER + 0x80), d(1));
        assert_eq!(e.mem.u32(SHADER + 0x88), d(5));
        assert_eq!(e.mem.u32(SHADER + 0x90), d(3));
        assert_eq!(e.mem.u32(SHADER + 0x98), d(7));
    }

    #[test]
    fn assign_member_releases_the_old_object_and_keeps_the_same_one() {
        let mut e = Engine::new();
        counting(&mut e);
        e.put_vtable(0x0f90_0000, &[0, 0x0f90_1004]);
        e.register(0x0f90_1004, |_, _| Ret::default());
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, 0x0f90_0000);
        e.mem.set_u32(old + 4, 1);
        let new = e.mem.alloc(8);
        let member = e.mem.alloc(4);
        e.mem.set_u32(member, old);
        e.call_log = Some(vec![]);
        assign_member(&mut e, member, new);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (IMPORT_INTERLOCKED_DECREMENT, vec![old + 4]),
                (0x0f90_1004, vec![old]),
                (IMPORT_INTERLOCKED_INCREMENT, vec![new + 4]),
            ]
        );
        assert_eq!(e.mem.u32(member), new);
        e.call_log = Some(vec![]);
        assign_member(&mut e, member, new);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        assert_eq!(e.mem.u32(new + 4), 1);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(model: i32) -> (Engine, u32, [u32; 4]) {
        let mut e = rig();
        for page in [0x0120_0000u32, 0x011f_9000, 0x011a_e000] {
            e.map(page, 0x1000);
        }
        e.mem.write(0x0120_0848, &[0xee; 0x20]);
        e.set_global::<u32>(WORD_SETTING_011AE60C, 9);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0xa4);
        e.mem.write(this, &[0xcc; 0xa4]);
        e.mem.set_u32(this + 0x28, 0);
        let declarations = [
            object(&mut e, 0),
            object(&mut e, 0),
            object(&mut e, 0),
            object(&mut e, 0),
        ];
        let words = [
            this,
            declarations[0],
            declarations[1],
            declarations[2],
            declarations[3],
        ];
        assert_eq!(e.call(0x00bb_c640, &words).u32(), this);
        (e, this, declarations)
    }

    #[test]
    fn fn_00bbc640_builds_the_shader() {
        let (e, this, d) = construct(3);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), LIGHTING30_VTABLE_010BA0F8);
        assert_eq!(e.mem.u32(this + 0x68), 0x11);
        for (offset, declaration) in [
            (0x28u32, d[0]),
            (0x7c, d[0]),
            (0x80, d[0]),
            (0x8c, d[1]),
            (0x90, d[1]),
            (0x84, d[2]),
            (0x88, d[2]),
            (0x94, d[3]),
            (0x98, d[3]),
        ] {
            assert_eq!(e.mem.u32(this + offset), declaration, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(d[0] + 4), 3);
        assert_eq!(e.mem.u32(d[1] + 4), 2);
        for offset in [0x6cu32, 0x70, 0x74, 0x78, 0x9c] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0xa0), 1);
        assert_eq!(e.mem.bytes(0x0120_0848, 0xa), vec![0; 0xa]);
        assert_eq!(e.mem.bytes(0x0120_0854, 0x11), vec![0; 0x11]);
        assert_eq!(e.global::<u16>(0x0120_0852), 0xeeee);
        assert_eq!(e.global::<u32>(WORD_SETTING_011AE60C), 5);
    }

    #[test]
    fn fn_00bbc640_keeps_the_word_from_model_7() {
        let (e, _, _) = construct(7);
        assert_eq!(e.global::<u32>(WORD_SETTING_011AE60C), 9);
    }
}

#[cfg(test)]
mod b056_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    pub(super) const SLOTS: u32 = 0x0e9f_0000;
    pub(super) const VTABLE: u32 = 0x0e9e_0000;

    /// [`rig`] with the constant pages mapped, a test vtable of recorded no-op slots, the flag
    /// objects at `012010a4` to `012010e8` (all set) and no-op doubles for the callees.
    pub(super) fn engine() -> Engine {
        let mut e = rig();
        for page in [
            0x0120_0000u32,
            0x0120_1000,
            0x0120_2000,
            0x011f_9000,
            0x011f_a000,
            0x011f_4000,
            0x011f_d000,
            0x011f_f000,
            0x011a_9000,
            0x011a_e000,
            0x0101_2000,
            0x0101_7000,
            0x0101_e000,
            0x0126_f000,
        ] {
            e.map(page, 0x1000);
        }
        let slots: Vec<u32> = (0..0x60u32).map(|i| SLOTS + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for slot in slots {
            e.register(slot, |_, _| Ret::default());
        }
        for offset in (0x0120_10a4u32..=0x0120_10e8).step_by(4) {
            let flag = e.mem.alloc(0x70);
            e.mem.set_u32(flag, VTABLE);
            e.mem.set_u32(flag + 4, 1);
            e.mem.set_u8(flag + 8, 1);
            e.set_global::<u32>(offset, flag);
        }
        for target in [
            NI_FREE_SIZED_00AA1460,
            SHADER_CALL_00BE1FF0,
            MAKE_RENDER_STATE_00C0ABD0,
            SELECT_MAP_00C663D0,
            STATE_CALL_00B97E80,
            STATE_CALL_00B98070,
            SET_Z_WRITE_ENABLE,
            VEC3_TRANSFORM_COORD,
            SHADER_CALL_00BE0870,
            BS_SHADER_DESTRUCT,
            MEMCPY,
            MATRIX_MULTIPLY,
            MATRIX_TRANSPOSE,
            GET_D3D_FROM_NI,
            ALPHA_TEST_004A2020,
            FIRST_ACTIVE_LIGHT,
            VECTOR_LENGTH_00B719E0,
            TRANSFORM_00439EF0,
            CALL_00633B40,
            LENGTH_00457990,
            CLAMP_0068B050,
            CLAMP_00B719C0,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e
    }

    pub(super) fn flag(e: &Engine, global: u32) -> u8 {
        e.mem.u8(e.global::<u32>(global) + 8)
    }

    pub(super) fn set_flag(e: &mut Engine, global: u32, value: u8) {
        let object = e.global::<u32>(global);
        e.mem.set_u8(object + 8, value);
    }

    pub(super) fn words(e: &Engine, addr: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.global::<u32>(addr + 4 * i)).collect()
    }

    pub(super) fn f(value: f32) -> u32 {
        value.to_bits()
    }

    #[test]
    fn fn_00bbb790_copies_the_ambient_colour_and_keeps_the_flag() {
        let mut e = engine();
        let color = e.mem.alloc(0x10);
        for index in 0..4u32 {
            e.mem.set_u32(color + 4 * index, 0x40 + index);
        }
        e.call(0x00bb_b790, &args![color, 0u32]);
        assert_eq!(words(&e, AMBIENT_01201B30, 4), vec![0x40, 0x41, 0x42, 0x43]);
        assert_eq!(flag(&e, AMBIENT_ENABLE_012010C4), 0);
        e.call(0x00bb_b790, &args![color, 0x101u32]);
        assert_eq!(flag(&e, AMBIENT_ENABLE_012010C4), 1);
        assert_eq!(e.global::<u8>(AMBIENT_SET_01200BF4), 1);
        // The byte set earlier keeps the flag for a clear call, then the byte is cleared.
        e.call(0x00bb_b790, &args![color, 0u32]);
        assert_eq!(flag(&e, AMBIENT_ENABLE_012010C4), 1);
        assert_eq!(e.global::<u8>(AMBIENT_SET_01200BF4), 0);
    }

    #[test]
    fn fn_00bbb840_and_00bbe020_copy_the_fog_colour() {
        let mut e = engine();
        for index in 0..4u32 {
            e.set_global::<u32>(FOG_COLOR_011A9BD0 + 4 * index, 0x50 + index);
        }
        set_flag(&mut e, AMBIENT_ENABLE_012010C4, 0);
        e.call(0x00bb_b840, &[]);
        assert_eq!(words(&e, AMBIENT_01201B30, 4), vec![0x50, 0x51, 0x52, 0x53]);
        assert_eq!(flag(&e, AMBIENT_ENABLE_012010C4), 1);
        assert_eq!(e.global::<u8>(AMBIENT_FOG_01200BF5), 1);
        e.register(MAKE_RENDER_STATE_00C0ABD0, |_, a| {
            (a[1] + 0x1000).into_ret()
        });
        let this = e.mem.alloc(0xa4);
        let first = object(&mut e, 1);
        let second = object(&mut e, 1);
        e.mem.set_u32(this + 0x6c, first);
        e.mem.set_u32(this + 0x70, second);
        e.call(0x00bb_e020, &args![this, 0x1e0u32]);
        assert_eq!(e.global::<u32>(RENDER_STATE_0126F74C), 0x11e0);
        assert_eq!(e.mem.u32(this + 0x30), first);
        assert_eq!(e.mem.u32(this + 0x34), second);
        assert_eq!(e.mem.u32(first + 4), 2);
        e.call(0x00bb_e020, &args![this, 7u32]);
        assert_eq!(
            calls(&e, SELECT_MAP_00C663D0),
            vec![
                vec![this + 0x30, this + 0x74],
                vec![this + 0x34, this + 0x78]
            ]
        );
    }

    #[test]
    fn constant_setters_copy_with_their_flags() {
        let mut e = engine();
        let property = with_vtable(&mut e, 0x100);
        for index in 0..4u32 {
            e.mem.set_u32(property + 0x84 + 4 * index, 0x60 + index);
        }
        e.mem.set_f32(property + 0xe0, 0.75);
        e.mem.set_f32(property + 0x30, 0.5);
        e.set_global::<f32>(GLOW_SCALE_010177E8, 0.9);
        e.call(0x00bb_bda0, &args![0u32, property]);
        assert_eq!(words(&e, EMIT_01201B50, 4), vec![0x60, 0x61, 0x62, 0x63]);
        e.call(0x00bb_bdf0, &args![0u32, property]);
        assert_eq!(words(&e, GLOW_01201B90, 2), vec![f(0.75), f(1.0)]);
        e.mem.set_u8(property + 0x24, 0x10);
        e.call(0x00bb_bdf0, &args![0u32, property]);
        assert_eq!(e.global::<f32>(GLOW_01201B90 + 4), 0.9);
        e.register(SLOTS + 0xf4, |_, a| {
            u32::from(a[1] == 5 && a[2] == 0).into_ret()
        });
        e.call(0x00bb_be30, &args![property]);
        assert_eq!(words(&e, ALPHA_01201BE8, 2), vec![f(0.5), f(1.0)]);
        // Flags clear: nothing is written.
        for global in [
            EMIT_ENABLE_012010CC,
            GLOW_ENABLE_012010D8,
            ALPHA_ENABLE_012010E0,
        ] {
            set_flag(&mut e, global, 0);
        }
        e.mem.set_u32(property + 0x84, 0x99);
        e.mem.set_f32(property + 0x30, 0.25);
        e.call(0x00bb_bda0, &args![0u32, property]);
        e.call(0x00bb_be30, &args![property]);
        assert_eq!(e.global::<u32>(EMIT_01201B50), 0x60);
        assert_eq!(e.global::<f32>(ALPHA_01201BE8), 0.5);
    }

    pub(super) fn with_vtable(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, VTABLE);
        object
    }

    #[test]
    fn small_answers_and_the_scalar_deleting_destructor() {
        let mut e = engine();
        let this = with_vtable(&mut e, 0xa4);
        for offset in (0x7cu32..=0x94).step_by(4) {
            e.mem.set_u32(this + offset, offset);
        }
        assert_eq!(e.call(0x00bb_c9b0, &args![this]).u32(), 0x0120_1f20);
        let geometry = e.mem.alloc(0xbc);
        let holder = e.mem.alloc(0x2c);
        e.mem.set_u32(geometry + 0xb8, holder);
        let property = e.mem.alloc(0x28);
        let mut seen = vec![];
        for (skin, flags, bit) in [
            (0u32, 0u8, 0u8),
            (1, 0, 0),
            (0, 0x20, 0),
            (0, 0, 2),
            (1, 0, 2),
        ] {
            e.mem.set_u32(holder + 0x28, skin);
            e.mem.set_u8(property + 0x24, flags);
            e.mem.set_u8(property + 0x20, bit);
            seen.push(e.call(0x00bb_d820, &args![this, geometry, property]).u32());
        }
        assert_eq!(seen, vec![0x7c, 0x84, 0x84, 0x8c, 0x94]);
        e.set_global::<u32>(PASS_011F91E4, 0x1d0);
        assert_eq!(e.call(0x00bb_c5c0, &args![this, 0u32]).u32(), 0x1d0);
        assert!(calls(&e, SHADER_CALL_00BE1FF0).is_empty());
        e.set_global::<u32>(PASS_011F91E4, 0x1c4);
        e.register(SHADER_CALL_00BE1FF0, |_, _| 0x31u32.into_ret());
        assert_eq!(e.call(0x00bb_c5c0, &args![this, 0x77u32]).u32(), 0x31);
        assert_eq!(calls(&e, SHADER_CALL_00BE1FF0), vec![vec![this, 0x77]]);
    }

    #[test]
    fn fn_00bbe650_runs_its_slots_except_for_some_passes() {
        let mut e = engine();
        let this = with_vtable(&mut e, 0xa4);
        let record = e.mem.alloc(0x14);
        e.mem.set_u32(record + 0xc, 0x0e33_0000);
        e.set_global::<u32>(PASS_011F91E4, 0xe);
        e.call(0x00bb_e650, &args![this, record]);
        assert!(calls(&e, SLOTS + 0x13c).is_empty());
        e.set_global::<u32>(PASS_011F91E4, 0x1d0);
        e.call(0x00bb_e650, &args![this, record]);
        assert_eq!(
            calls(&e, SLOTS + 0x13c),
            vec![vec![this, 0x1d0, 0x0e33_0000]]
        );
        assert_eq!(
            calls(&e, SLOTS + 0x140),
            vec![vec![this, 0x1d0, 0x0e33_0000]]
        );
        assert_eq!(calls(&e, SLOTS + 0x10c), vec![vec![this]]);
    }

    #[test]
    fn fn_00bbeef0_undoes_the_states() {
        let mut e = engine();
        let record = e.mem.alloc(0x14);
        let flags = e.mem.alloc(0x1c);
        e.mem.set_u8(flags + 0x18, 1);
        e.mem.set_u32(record, flags);
        e.mem.set_u32(record + 0x10, flags);
        e.set_global::<u32>(COUNTER_011FF9DC, 5);
        e.set_global::<u32>(PASS_011F91E4, 0x1d4);
        e.call(0x00bb_eef0, &args![0u32, record]);
        assert!(calls(&e, STATE_CALL_00B97E80).is_empty());
        assert_eq!(calls(&e, STATE_CALL_00B98070), vec![vec![0, 0]]);
        assert_eq!(e.global::<u32>(COUNTER_011FF9DC), 4);
        assert_eq!(calls(&e, SET_Z_WRITE_ENABLE), vec![vec![1, 0]]);
        e.set_global::<u32>(PASS_011F91E4, 0x1e1);
        e.call(0x00bb_eef0, &args![0u32, record]);
        assert_eq!(calls(&e, STATE_CALL_00B97E80), vec![vec![3, 0]]);
        assert_eq!(e.global::<u32>(COUNTER_011FF9DC), 4);
    }

    pub(super) fn camera(e: &mut Engine, position: [f32; 3]) {
        let camera = e.mem.alloc(0x1f0);
        for (index, value) in position.iter().enumerate() {
            e.mem.set_f32(camera + 0x1e4 + 4 * index as u32, *value);
        }
        e.set_global::<u32>(CAMERA_011F91C8, camera);
    }

    #[test]
    fn fn_00bbb890_transforms_the_light_position() {
        let mut e = engine();
        camera(&mut e, [1.0, 2.0, 3.0]);
        for (index, value) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.set_global::<f32>(LIGHT_DIRECTION_011FA2A0 + 4 * index as u32, *value);
        }
        // The transform double doubles the vector into the output.
        e.register(VEC3_TRANSFORM_COORD, |e, a| {
            for index in 0..3 {
                let value = e.mem.f32(a[1] + 4 * index);
                e.mem.set_f32(a[0] + 4 * index, 2.0 * value);
            }
            Ret::default()
        });
        let holder = e.mem.alloc(0x34);
        e.mem.set_f32(holder + 0x30, 7.0);
        let mut words = vec![0u32];
        words.extend((0..16u32).map(|i| 0x1000 + i));
        words.push(holder);
        e.call(0x00bb_b890, &words);
        let transform = calls(&e, VEC3_TRANSFORM_COORD);
        assert_eq!(transform.len(), 1);
        assert_eq!(e.mem.u32(transform[0][2] + 0x3c), 0x100f);
        assert_eq!(
            words_at(&e, LIGHT_POSITION_01201370),
            vec![f(22.0), f(44.0), f(66.0), f(1.0)]
        );
        assert_eq!(
            words_at(&e, LIGHT_POSITION_01201B40),
            vec![f(22.0), f(44.0), f(66.0), f(7.0)]
        );
        // Both flags clear: nothing.
        set_flag(&mut e, LIGHT_ENABLE_012010A4, 0);
        set_flag(&mut e, LIGHT_ENABLE_012010C8, 0);
        e.call(0x00bb_b890, &words);
        assert_eq!(calls(&e, VEC3_TRANSFORM_COORD).len(), 1);
    }

    fn words_at(e: &Engine, addr: u32) -> Vec<u32> {
        words(e, addr, 4)
    }

    #[test]
    fn fn_00bbbcf0_and_00bbbe80_compute_from_the_camera() {
        let mut e = engine();
        camera(&mut e, [1.0, 2.0, 3.0]);
        for (index, value) in [0.5f32, 0.25, 0.125].iter().enumerate() {
            e.set_global::<f32>(ORIGIN_011F474C + 4 * index as u32, *value);
        }
        let position = e.mem.alloc(0xc);
        for (index, value) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * index as u32, *value);
        }
        e.call(0x00bb_bcf0, &args![0u32, position]);
        assert_eq!(
            words(&e, EYE_01201360, 3),
            vec![f(8.5), f(17.75), f(26.875)]
        );
        for (index, value) in [1.0f32, 0.0, 2.0].iter().enumerate() {
            e.set_global::<f32>(FOG_PLANE_011F9604 + 4 * index as u32, *value);
            e.set_global::<f32>(LIGHT_DIRECTION_011FA2A0 + 4 * index as u32, 4.0);
        }
        e.set_global::<f32>(FOG_OFFSET_011F9610, 3.0);
        e.set_global::<f32>(FOG_FAR_011F9618, 100.0);
        e.set_global::<f32>(FOG_NEAR_011F9614, 40.0);
        e.call(0x00bb_be80, &args![0u32]);
        // (1 * (1 + 4) + 0 + 2 * (3 + 4)) - 3 = 16.
        assert_eq!(
            words(&e, FOG_01201BC0, 8),
            vec![
                f(1.0),
                0,
                f(2.0),
                f(16.0),
                f(100.0),
                f(60.0),
                f(1.0),
                f(1.0)
            ]
        );
        e.set_global::<f32>(FOG_FAR_011F9618, 0.0);
        e.set_global::<f32>(FOG_NEAR_011F9614, 0.0);
        e.set_global::<f32>(MINUS_ONE_01012054, -1.0);
        e.call(0x00bb_be80, &args![0u32]);
        assert_eq!(words(&e, FOG_01201BC0 + 0x10, 4), vec![0, 0, f(-1.0), 0]);
    }

    #[test]
    fn fn_00bbba00_builds_the_world_matrix() {
        let mut e = engine();
        camera(&mut e, [1.0, 2.0, 3.0]);
        let transform = e.mem.alloc(0x34);
        for index in 0..13u32 {
            e.mem.set_f32(transform + 4 * index, index as f32);
        }
        for index in 0..8u32 {
            e.set_global::<u32>(WORLD_EXTRA_011FD874 + 4 * index, 0x70 + index);
        }
        e.call(0x00bb_ba00, &args![0u32, 0u32, transform]);
        let matrix = words(&e, WORLD_012012F0, 16);
        let want: Vec<u32> = [
            0.0f32, 1.0, 2.0, 10.0, 3.0, 4.0, 5.0, 12.0, 6.0, 7.0, 8.0, 14.0, 0.0, 0.0, 0.0, 12.0,
        ]
        .iter()
        .map(|v| f(*v))
        .collect();
        assert_eq!(matrix, want);
        assert_eq!(
            words(&e, WORLD_012012F0 + 0x40, 8),
            (0x70..0x78).collect::<Vec<u32>>()
        );
        for (index, value) in [0.5f32, 0.25, 0.125].iter().enumerate() {
            e.set_global::<f32>(ORIGIN_011F474C + 4 * index as u32, *value);
        }
        e.call(0x00bb_ba00, &args![0u32, 1u32, transform]);
        let matrix = words(&e, WORLD_012012F0, 16);
        assert_eq!(matrix[0], f(1.0));
        assert_eq!(matrix[3], f(1.5));
        assert_eq!(matrix[7], f(2.25));
        assert_eq!(matrix[11], f(3.125));
        assert_eq!(matrix[15], f(1.0));
        assert_eq!(matrix[12], 0);
    }

    #[test]
    fn fn_00bbce70_releases_and_resets_the_shared_constants() {
        let mut e = engine();
        let shader = object(&mut e, 1);
        e.set_global::<u32>(SHADERS_01200DD8 + 8, shader);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.set_global::<u32>(PASSES_01200EC0 + 4, pass);
        for index in 0..4u32 {
            e.set_global::<u32>(FOG_COLOR_011A9BD0 + 4 * index, 0x80 + index);
        }
        let this = e.mem.alloc(0xa4);
        assert!(e.call(0x00bb_ce70, &args![this]).bool());
        assert_eq!(calls(&e, SHADER_CALL_00BE0870), vec![vec![this]]);
        assert_eq!(calls(&e, REFERENCE_DELETE)[0], vec![shader]);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(e.global::<u32>(FLAGS_012010C4), 0);
        // The colour repeated over the blocks.
        assert_eq!(
            words(&e, CONSTANTS_012010F0 + 0xa30, 4),
            vec![0x80, 0x81, 0x82, 0x83]
        );
        assert_eq!(
            words(&e, AMBIENT_01201B30 + 0x3e0, 4),
            vec![0x80, 0x81, 0x82, 0x83]
        );
    }

    #[test]
    fn fn_00bbc9c0_destroys_the_members_and_keeps_the_memory() {
        let mut e = engine();
        let this = e.mem.alloc(0xa4);
        let declaration = object(&mut e, 2);
        e.mem.set_u32(this + 0x28, declaration);
        let cleared = object(&mut e, 1);
        e.mem.set_u32(this + 0x80, cleared);
        e.mem.set_u32(this + 0x9c, 5);
        e.call(0x00bb_c9c0, &args![this]);
        assert_eq!(e.mem.u32(this), LIGHTING30_VTABLE_010BA0F8);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(declaration + 4), 1);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u32(this + 0x9c), 0);
        assert!(calls(&e, REFERENCE_DELETE).contains(&vec![cleared]));
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
        assert!(calls(&e, NI_FREE_SIZED_00AA1460).is_empty());
    }

    #[test]
    fn fn_00bbef60_destroys_the_shader() {
        let mut e = engine();
        let this = e.mem.alloc(0xa4);
        let declaration = object(&mut e, 2);
        e.mem.set_u32(this + 0x28, declaration);
        let member = object(&mut e, 2);
        e.mem.set_u32(this + 0x6c, member);
        let cleared = object(&mut e, 1);
        e.mem.set_u32(this + 0x80, cleared);
        e.mem.set_u32(this + 0x9c, 5);
        assert_eq!(e.call(0x00bb_ef60, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), LIGHTING30_VTABLE_010BA0F8);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(declaration + 4), 1);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u32(this + 0x9c), 0);
        // +0x6c is only released by its member destructor.
        assert_eq!(e.mem.u32(this + 0x6c), member);
        assert_eq!(e.mem.u32(member + 4), 1);
        assert!(calls(&e, REFERENCE_DELETE).contains(&vec![cleared]));
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0xa4]]);
    }

    #[test]
    fn fn_00bbe6b0_copies_or_builds_the_view_matrix() {
        let mut e = engine();
        let enable = e.mem.alloc(0xc);
        e.mem.set_u8(enable + 8, 1);
        e.set_global::<u32>(VIEW_ENABLE_01201098, enable);
        let this = e.mem.alloc(0xa4);
        let renderer = e.mem.alloc(0x980);
        e.mem.set_u32(this + 0x18, renderer);
        for index in 0..16u32 {
            e.mem
                .set_u32(renderer + 0x940 + 4 * index, 0x7f80_0001 + index);
        }
        let context = e.mem.alloc(0x10);
        e.call(0x00bb_e6b0, &args![this, context, 0u32, 0u32, 0u32]);
        let transposed = calls(&e, MATRIX_TRANSPOSE);
        assert_eq!(transposed.len(), 1);
        // The copy goes through the x87: the signalling NaN is quieted.
        assert_eq!(e.global::<u32>(VIEW_01201170), 0x7fc0_0001);
        assert!(calls(&e, MEMCPY).is_empty());
        // A light list and the context byte: the identity times the light's matrix.
        let lights = e.mem.alloc(8);
        let first = e.mem.alloc(0x100);
        let second = e.mem.alloc(0x100);
        e.mem.set_u32(lights, first);
        e.mem.set_u32(lights + 4, second);
        e.mem.set_u32(context + 0xc, lights);
        e.mem.set_u8(context + 9, 1);
        e.set_global::<f32>(ORIGIN_011F474C, 2.0);
        e.register(MATRIX_MULTIPLY, |e, a| {
            for index in 0..16 {
                let word = e.mem.u32(a[1] + 4 * index);
                e.mem.set_u32(a[0] + 4 * index, word);
            }
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.call(0x00bb_e6b0, &args![this, context, 0x1c4u32, 1u32, 0u32]);
        let multiply = calls(&e, MATRIX_MULTIPLY);
        assert_eq!(multiply[0][2], first + 0x10);
        assert!(calls(&e, GET_D3D_FROM_NI).is_empty());
        let copies = calls(&e, MEMCPY);
        assert_eq!(copies[0][0], VIEW_01201170);
        assert_eq!(e.global::<f32>(VIEW_01201170 + 0x30), 2.0);
        assert_eq!(copies[1][0], PROJECTION_012012B0);
        assert_eq!(e.global::<f32>(PROJECTION_012012B0), 1.0);
        assert_eq!(e.global::<f32>(PROJECTION_012012B0 + 0x30), 0.0);
        // Another pass uses the second light; without the flag word, the transform's D3D form.
        e.call(
            0x00bb_e6b0,
            &args![this, context, 0x1d0u32, 0u32, 0x0e40_0000u32],
        );
        assert_eq!(calls(&e, MATRIX_MULTIPLY)[1][2], second + 0x10);
        assert_eq!(calls(&e, GET_D3D_FROM_NI)[0][1], 0x0e40_0000);
        assert_eq!(calls(&e, MEMCPY).len(), 3);
    }

    #[test]
    fn fn_00bbe9c0_sets_the_material_constants() {
        let mut e = engine();
        let property = with_vtable(&mut e, 0x100);
        e.mem.set_f32(property + 0x28, 0.5);
        let record = e.mem.alloc(0x14);
        let flags = e.mem.alloc(0x1c);
        e.mem.set_u32(record, flags);
        // Field 2, bit 9 with the byte 51.
        e.mem.set_u16(flags + 0x18, (2 << 5) | (1 << 9));
        e.mem.set_u8(flags + 0x1a, 51);
        e.set_global::<f64>(TWO_FIFTY_FIVE_0101E568, 255.0);
        e.call(0x00bb_e9c0, &args![0u32, 0u32, property, record, 0u32]);
        assert_eq!(
            words(&e, MATERIAL_01201B60, 4),
            vec![f(0.5), f(0.2), 0, f(1.0)]
        );
        assert_eq!(e.global::<u8>(AMBIENT_FOG_01200BF5), 1);
        assert_eq!(flag(&e, ALPHA_TEST_ENABLE_012010D0), 1);
        // Mode 8.
        e.set_global::<u32>(MODE_011F91B8, 8);
        e.mem.set_u32(property + 0x24, 0x2_0000);
        e.call(0x00bb_e9c0, &args![0u32, 0u32, property, record, 0u32]);
        assert_eq!(words(&e, MATERIAL_01201B60, 2), vec![0, 0]);
        e.mem.set_u32(record, 0);
        e.call(0x00bb_e9c0, &args![0u32, 0u32, property, record, 0u32]);
        assert_eq!(e.global::<f32>(MATERIAL_01201B60 + 4), 1.0);
        // The light path: the alpha scaled by slot +0x110, the light test answering 0.
        e.set_global::<u32>(MODE_011F91B8, 0);
        e.mem.set_u32(record, flags);
        e.mem.set_u16(flags + 0x18, 0);
        e.mem.set_u32(property + 0x24, 0);
        e.register(SLOTS + 0x110, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        e.call(0x00bb_e9c0, &args![0u32, 0u32, property, record, 0x1bdu32]);
        assert_eq!(words(&e, MATERIAL_01201B60, 4), vec![f(1.0), 0, f(1.0), 0]);
        assert_eq!(e.global::<u8>(AMBIENT_FOG_01200BF5), 0);
        assert_eq!(flag(&e, ALPHA_TEST_ENABLE_012010D0), 1);
        e.call(0x00bb_e9c0, &args![0u32, 0u32, property, record, 0x1bdu32]);
        assert_eq!(flag(&e, ALPHA_TEST_ENABLE_012010D0), 0);
    }

    #[test]
    fn fn_00bbe9c0_light_falloff() {
        let mut e = engine();
        let property = with_vtable(&mut e, 0x100);
        e.mem.set_f32(property + 0x28, 0.5);
        e.mem.set_u32(property + 0x24, 0x8000);
        e.register(SLOTS + 0x110, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        let record = e.mem.alloc(0x14);
        let flags = e.mem.alloc(0x1c);
        e.mem.set_u32(record, flags);
        let camera_object = e.mem.alloc(0xf0);
        let node = e.mem.alloc(0xfc);
        let target = e.mem.alloc(0xe0);
        e.mem.set_u32(camera_object + 0xe0, node);
        e.mem.set_u32(node + 0xf8, target);
        e.set_global::<u32>(CAMERA_011F91C8, camera_object);
        let light = e.mem.alloc(0xfc);
        let data = e.mem.alloc(0xe4);
        e.mem.set_u32(light + 0xf8, data);
        e.mem.set_f32(data + 0xe0, 10.0);
        e.register_double(FIRST_ACTIVE_LIGHT, move |_, _| light.into_ret());
        // The camera vector's length 0.25, the distance 5 (ratio 0.5, clamped as it is),
        // the other vector's length 0.75: max(0.25, 0.75 * 0.5) = 0.375, times the alpha 0.5.
        e.register_double(VECTOR_LENGTH_00B719E0, {
            let mut count = 0;
            move |_, _| {
                count += 1;
                Ret {
                    st0: if count == 1 { 0.25 } else { 0.75 },
                    ..Ret::default()
                }
            }
        });
        e.register(LENGTH_00457990, |_, _| Ret {
            st0: 5.0,
            ..Ret::default()
        });
        e.register(CLAMP_0068B050, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[2])),
            ..Ret::default()
        });
        e.register(CLAMP_00B719C0, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[2]) | (u64::from(a[3]) << 32)),
            ..Ret::default()
        });
        e.call(
            0x00bb_e9c0,
            &args![0u32, 0x0e66_0000u32, property, record, 0x1c0u32],
        );
        assert_eq!(e.global::<f32>(MATERIAL_01201B60), 0.1875);
        assert_eq!(calls(&e, TRANSFORM_00439EF0)[0][2], 0x0e66_0000);
        assert_eq!(calls(&e, ALPHA_TEST_004A2020), vec![vec![property, 0x11]]);
    }

    #[test]
    fn fn_00bbecd0_adds_the_ambient_or_sets_the_diffuse() {
        let mut e = engine();
        let material = e.mem.alloc(0x44);
        for (index, value) in [0.5f32, 0.75, 0.25].iter().enumerate() {
            e.mem.set_f32(material + 0x28 + 4 * index as u32, *value);
        }
        e.mem.set_f32(material + 0x40, 2.0);
        let property = e.mem.alloc(0xb8);
        let list = e.mem.alloc(4);
        e.mem.set_u32(property + 0xb4, list);
        for (index, value) in [0.5f32, 0.5, 0.5].iter().enumerate() {
            e.set_global::<f32>(AMBIENT_01201B30 + 4 * index as u32, *value);
        }
        e.set_global::<u32>(AMBIENT_01201B30 + 0xc, 0x77);
        set_flag(&mut e, AMBIENT_ENABLE_012010C4, 0);
        e.call(0x00bb_ecd0, &args![0u32, material, property]);
        assert_eq!(
            words(&e, AMBIENT_01201B30, 4),
            vec![f(1.0), f(1.0), f(0.75), 0x77]
        );
        assert_eq!(flag(&e, AMBIENT_ENABLE_012010C4), 1);
        // With a light: the diffuse, clamped.
        e.mem.set_u32(list, 1);
        e.mem.set_f32(material + 0x2c, 1.5);
        e.call(0x00bb_ecd0, &args![0u32, material, property]);
        assert_eq!(
            words(&e, DIFFUSE_01201B70, 4),
            vec![f(0.5), f(1.0), f(0.25), 0]
        );
        // The scale setting: scaled by 2 * the light scale 0.5, not clamped.
        e.set_global::<u8>(ALWAYS_SCALE_011F941E, 1);
        let chain = [
            e.mem.alloc(0xc),
            e.mem.alloc(8),
            e.mem.alloc(0x70),
            e.mem.alloc(0x10),
        ];
        e.mem.set_u32(chain[0] + 8, chain[1]);
        e.mem.set_u32(chain[1] + 4, chain[2]);
        e.mem.set_u32(chain[2] + 0x6c, chain[3]);
        e.mem.set_f32(chain[3] + 0xc, 0.5);
        e.set_global::<u32>(SCALE_HOLDER_011F91AC, chain[0]);
        e.call(0x00bb_ecd0, &args![0u32, material, property]);
        assert_eq!(
            words(&e, DIFFUSE_01201B70, 3),
            vec![f(0.5), f(1.5), f(0.25)]
        );
        // The "no colour" colour without lights: nothing.
        e.set_global::<u8>(ALWAYS_SCALE_011F941E, 0);
        e.mem.set_u32(list, 0);
        for (index, value) in [0.5f32, 1.5, 0.25].iter().enumerate() {
            e.set_global::<f32>(NO_COLOR_011F4998 + 4 * index as u32, *value);
        }
        e.set_global::<u32>(AMBIENT_01201B30, 0x55);
        e.call(0x00bb_ecd0, &args![0u32, material, property]);
        assert_eq!(e.global::<u32>(AMBIENT_01201B30), 0x55);
    }
}

#[cfg(test)]
mod b056_set_up_tests {
    use super::b056_tests::{camera, engine, f, flag, with_vtable, words, SLOTS};
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// [`engine`] with no-op doubles for the light set-up's callees and `_ftol2` truncating.
    fn light_engine() -> Engine {
        let mut e = engine();
        for target in [
            SHADER_MANAGER_CALL_00B4F5D0,
            LIGHT_SET_00B70DE0,
            LIGHT_SET_00B71130,
            ACTIVE_LIGHTS,
            VEC3_TRANSFORM_NORMAL,
            VEC3_NORMALIZE,
            UNITIZE_004A0C10,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            (value as i32 as u32).into_ret()
        });
        e
    }

    /// A scene context of `kind` with `count` lights (0x0e60_0000 + index).
    fn context(e: &mut Engine, kind: u16, count: u8) -> u32 {
        let context = e.mem.alloc(0x10);
        e.mem.set_u16(context + 4, kind);
        e.mem.set_u8(context + 9, count);
        let lights = e.mem.alloc(0x40);
        for index in 0..16u32 {
            e.mem.set_u32(lights + 4 * index, 0x0e60_0000 + index);
        }
        e.mem.set_u32(context + 0xc, lights);
        context
    }

    #[test]
    fn fn_00bbc050_copies_the_accumulator_colour_in_mode_15() {
        let mut e = light_engine();
        let source = e.mem.alloc(0x164);
        for index in 0..4u32 {
            e.mem.set_u32(source + 0x154 + 4 * index, 0x90 + index);
        }
        e.register_double(SHADER_MANAGER_CALL_00B4F5D0, move |_, _| source.into_ret());
        e.set_global::<u32>(MODE_011F91B8, 0xf);
        let property = e.mem.alloc(0x70);
        e.call(
            0x00bb_c050,
            &args![0u32, property, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(words(&e, AMBIENT_01201B30, 4), vec![0x90, 0x91, 0x92, 0x93]);
        assert!(calls(&e, LIGHT_SET_00B70DE0).is_empty());
    }

    #[test]
    fn fn_00bbc050_passes_the_lights_and_the_scale() {
        let mut e = light_engine();
        camera(&mut e, [0.0, 0.0, 0.0]);
        let property = e.mem.alloc(0x70);
        e.mem.set_f32(property + 0x6c, 0.75);
        let context = context(&mut e, 0, 3);
        let material = e.mem.alloc(0x3c);
        e.mem.set_f32(material + 0x38, 9.0);
        e.set_global::<i32>(LIGHT_LIMIT_011AE60C, 2);
        e.call(
            0x00bb_c050,
            &args![0u32, property, context, 0x10u32, material, 0u32, 1u32, 0u32],
        );
        let scale = f(0.75);
        assert_eq!(
            calls(&e, LIGHT_SET_00B70DE0),
            vec![vec![0, 0x0e60_0000, scale]]
        );
        assert_eq!(
            calls(&e, LIGHT_SET_00B71130),
            vec![vec![1, 0x0e60_0001, scale], vec![2, 0x0e60_0002, scale]]
        );
        // Below pass 0x1bd: min(3, 2) - 1 = 1.
        assert_eq!(
            words(&e, LIGHT_DATA_01201C70, 4),
            vec![f(1.0), f(1.0), f(9.0), f(1.0)]
        );
        let data = e.global::<u32>(LIGHT_DATA_ENABLE_012010E8);
        assert_eq!(e.mem.u32(data + 0x20), 5);
    }

    #[test]
    fn fn_00bbc0a4_transforms_the_light_entries() {
        let mut e = light_engine();
        camera(&mut e, [1.0, 2.0, 3.0]);
        let property = e.mem.alloc(0x70);
        let context = context(&mut e, 0x1c5, 0);
        let material = e.mem.alloc(0x3c);
        let transform = e.mem.alloc(0x34);
        e.mem.set_f32(transform + 0x30, 2.0);
        // The normal transform doubles the vector; normalising copies it.
        e.register(VEC3_TRANSFORM_NORMAL, |e, a| {
            for index in 0..3 {
                let value = e.mem.f32(a[1] + 4 * index);
                e.mem.set_f32(a[0] + 4 * index, 2.0 * value);
            }
            Ret::default()
        });
        e.register(VEC3_NORMALIZE, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(VEC3_TRANSFORM_COORD, |e, a| {
            for index in 0..3 {
                let value = e.mem.f32(a[1] + 4 * index);
                e.mem.set_f32(a[0] + 4 * index, value + 100.0);
            }
            Ret::default()
        });
        // One directional and one point entry (pass 0x1d0: the else branch gives (0, 0)):
        // set the counts by hand through the pass below 0x1bd path with the limit.
        e.set_global::<i32>(LIGHT_LIMIT_011AE60C, 8);
        for (index, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
            .iter()
            .enumerate()
        {
            e.set_global::<f32>(LIGHT_ENTRIES_01201C90 + 4 * index as u32, *value);
        }
        e.call(
            0x00bb_c0a4,
            &args![
                property,
                0u32,
                context,
                0x1c4u32,
                material,
                0x0e70_0000u32,
                0u32,
                transform
            ],
        );
        // Kind 0x1c5: only light 0 through `00b71130`.
        assert_eq!(calls(&e, LIGHT_SET_00B71130), vec![vec![0, 0x0e60_0000, 0]]);
        // Pass 0x1c4: (0, 1) - no directional entries, one point entry scaled by 2.
        assert_eq!(words(&e, LIGHT_DATA_01201C70, 2), vec![0, f(1.0)]);
        let data = e.global::<u32>(LIGHT_DATA_ENABLE_012010E8);
        assert_eq!(e.mem.u32(data + 0x20), 3);
        assert!(calls(&e, VEC3_TRANSFORM_NORMAL).is_empty());
        assert_eq!(calls(&e, VEC3_TRANSFORM_COORD)[0][2], 0x0e70_0000);
        // (1 + 1 + 100) * 2, (2 + 2 + 100) * 2, (3 + 3 + 100) * 2, the fourth word kept.
        assert_eq!(
            words(&e, LIGHT_ENTRIES_01201C90, 4),
            vec![f(204.0), f(208.0), f(212.0), f(4.0)]
        );
        // Pass 0x1c6 with the flag word clear: the fourth word divided by the scale.
        e.call(
            0x00bb_c0a4,
            &args![
                property,
                0u32,
                context,
                0x1c6u32,
                material,
                0x0e70_0000u32,
                0u32,
                transform
            ],
        );
        assert_eq!(e.global::<f32>(LIGHT_ENTRIES_01201C90 + 0xc), 2.0);
        assert_eq!(e.global::<f32>(LIGHT_ENTRIES_01201C90), 305.0);
        // Pass 0x1c7 with the flag word set: stored as transformed.
        e.call(
            0x00bb_c0a4,
            &args![
                property,
                0u32,
                context,
                0x1c7u32,
                material,
                0x0e70_0000u32,
                1u32,
                transform
            ],
        );
        assert_eq!(e.global::<f32>(LIGHT_ENTRIES_01201C90 + 0xc), 2.0);
        assert_eq!(e.global::<f32>(LIGHT_ENTRIES_01201C90), 406.0);
        // A directional entry: count 1 from pass 0x10 with one light (min(1, 8) - 1 = 0 ->
        // (1.0, 0.0)): the negated direction transformed, normalised and unitised.
        let context = context_with_one(&mut e);
        e.call(
            0x00bb_c0a4,
            &args![
                property,
                0u32,
                context,
                0x10u32,
                material,
                0x0e70_0000u32,
                1u32,
                transform
            ],
        );
        assert_eq!(calls(&e, VEC3_TRANSFORM_NORMAL).len(), 1);
        assert_eq!(e.global::<f32>(LIGHT_ENTRIES_01201C90), -812.0);
        assert_eq!(calls(&e, UNITIZE_004A0C10).len(), 1);
    }

    fn context_with_one(e: &mut Engine) -> u32 {
        context(e, 0, 1)
    }

    #[test]
    fn fn_00bbc0a4_pass_0x1c3_and_the_colour_settings() {
        let mut e = light_engine();
        e.map(0x0101_6000, 0x1000);
        let property = e.mem.alloc(0x70);
        e.mem.set_u32(property + 0x20, 0x400);
        let context = context(&mut e, 0, 0);
        let material = e.mem.alloc(0x3c);
        let transform = e.mem.alloc(0x34);
        e.set_global::<u8>(ALWAYS_SCALE_011F941E, 1);
        e.set_global::<f32>(LIGHT_SCALE_011F9184, 2.0);
        for index in 0..4u32 {
            e.set_global::<f32>(LIGHT_COLOR_011FA0D0 + 4 * index, index as f32);
        }
        e.set_global::<u32>(MODE_011F91B8, 0xa);
        e.set_global::<f32>(HALF_01016248, 0.5);
        e.call(
            0x00bb_c0a4,
            &args![property, 0u32, context, 0x1c3u32, material, 0u32, 0u32, transform],
        );
        assert_eq!(
            words(&e, LIGHT_COLOR_011FA0D0, 4),
            vec![0, f(2.0), f(4.0), f(6.0)]
        );
        assert_eq!(
            words(&e, LIGHT_DATA_01201C70 + 0x10, 4),
            vec![f(0.5), f(0.5), f(0.5), f(1.0)]
        );
        let data = e.global::<u32>(LIGHT_DATA_ENABLE_012010E8);
        assert_eq!(e.mem.u32(data + 0x20), 2);
        // The fog lights setting replaces the colour; other modes copy the ambient constant.
        e.set_global::<u8>(FOG_LIGHTS_011F91A7, 1);
        for index in 0..4u32 {
            e.set_global::<u32>(FOG_COLOR_011A9BD0 + 4 * index, 0xa0 + index);
            e.set_global::<u32>(AMBIENT_01201B30 + 4 * index, 0xb0 + index);
        }
        e.set_global::<u32>(MODE_011F91B8, 0);
        e.call(
            0x00bb_c0a4,
            &args![property, 0u32, context, 0x1c3u32, material, 0u32, 0u32, transform],
        );
        assert_eq!(
            words(&e, LIGHT_COLOR_011FA0D0, 4),
            vec![0xa0, 0xa1, 0xa2, 0xa3]
        );
        assert_eq!(
            words(&e, LIGHT_DATA_01201C70 + 0x10, 4),
            vec![0xb0, 0xb1, 0xb2, 0xb3]
        );
        assert_eq!(
            calls(&e, ACTIVE_LIGHTS),
            vec![vec![property], vec![property]]
        );
    }

    /// [`light_engine`] with the scene holder, render state and the set-up's other callees.
    fn set_up_engine() -> (Engine, u32, u32) {
        let mut e = light_engine();
        for target in [
            LIGHT_MATRIX_00C4C2D0,
            RENDERER_CALL_00E6FBB0,
            SET_UP_00BBDDA0,
            SET_UP_00BBDCD0,
            SET_UP_00BBDE20,
            CAMERA_POSITION_00B71B40,
            DOT,
            MATRIX_INVERSE,
            PLANE_CALL_00EE6DCE,
            PLANE_CALL_00EE6DC8,
            STORE_PLANES_00B71AD0,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        let accumulator = e.mem.alloc(0x168);
        e.register_double(GET_ACCUMULATOR, move |_, _| accumulator.into_ret());
        let scene = e.mem.alloc(0xc0);
        let camera_holder = e.mem.alloc(0x38);
        e.mem.set_u32(scene + 0xb8, camera_holder);
        let holder = context(&mut e, 0, 0);
        e.mem.set_u32(holder, scene);
        e.set_global::<u32>(SCENE_HOLDER_011F91E0, holder);
        camera(&mut e, [0.0, 0.0, 0.0]);
        e.map(0x0104_e000, 0x1000);
        let state = e.mem.alloc(0x60);
        e.set_global::<u32>(RENDER_STATE_0126F74C, state);
        let enable = e.mem.alloc(0xc);
        e.mem.set_u8(enable + 8, 1);
        e.set_global::<u32>(VIEW_ENABLE_01201098, enable);
        (e, scene, state)
    }

    fn record(e: &mut Engine) -> (u32, u32) {
        let record = e.mem.alloc(0x14);
        let property = with_vtable(e, 0x100);
        let material = e.mem.alloc(0x44);
        e.mem.set_u32(record + 0xc, property);
        e.mem.set_u32(record + 8, material);
        let list = e.mem.alloc(4);
        e.mem.set_u32(property + 0xb4, list);
        let flags = e.mem.alloc(0x1c);
        e.mem.set_u32(record, flags);
        (record, property)
    }

    #[test]
    fn fn_00bbef90_passes_0x1e0_take_the_short_path() {
        let (mut e, scene, state) = set_up_engine();
        let this = with_vtable(&mut e, 0xa4);
        e.mem.set_u32(this + 0x18, 0x0e18_0000);
        let (record, property) = record(&mut e);
        e.set_global::<u32>(PASS_011F91E4, 0x1e0);
        e.call(0x00bb_ef90, &args![this, record]);
        assert_eq!(
            calls(&e, RENDERER_CALL_00E6FBB0),
            vec![vec![0x0e18_0000, scene + 0x68, 0]]
        );
        assert_eq!(
            calls(&e, SET_UP_00BBDCD0),
            vec![vec![this, property, state]]
        );
        assert_eq!(
            calls(&e, SET_UP_00BBDE20),
            vec![vec![this, scene, property, state]]
        );
        assert_eq!(calls(&e, LIGHT_MATRIX_00C4C2D0)[0][..2], [scene + 0x68, 0]);
        assert!(calls(&e, LIGHT_SET_00B70DE0).is_empty());
        assert!(calls(&e, SET_UP_00BBDDA0).is_empty());
    }

    #[test]
    fn fn_00bbef90_runs_the_set_ups_and_the_maps() {
        let (mut e, scene, state) = set_up_engine();
        let this = with_vtable(&mut e, 0xa4);
        let map = with_vtable(&mut e, 8);
        e.mem.set_u32(this + 0x30, 1);
        e.mem.set_u32(state + 0x34, map);
        let shader = 0x0e22_0000;
        e.mem.set_u32(state + 0x44, shader);
        let (record, property) = record(&mut e);
        e.mem.set_u32(property + 0xe4, 3);
        e.mem.set_f32(property + 0xe8, 0.25);
        e.set_global::<f32>(TIME_011F940C, 1800.0);
        e.set_global::<f64>(SECONDS_PER_HOUR_01012640, 3600.0);
        e.set_global::<f64>(TWO_PI_0104EEA0, 4.0);
        let other = e.mem.alloc(0x1c);
        e.mem.set_u8(other + 0x18, 1);
        e.mem.set_u32(record + 0x10, other);
        let target = with_vtable(&mut e, 8);
        e.mem.set_u32(this + 0x1c, target);
        e.set_global::<u32>(PASS_011F91E4, 0xe);
        e.call(0x00bb_ef90, &args![this, record]);
        assert_eq!(
            calls(&e, SET_UP_00BBDDA0),
            vec![vec![this, property, state]]
        );
        // 1800 / 3600 * 4 * 3 = 6.
        assert_eq!(e.global::<f32>(REFRACTION_01201350), 6.0);
        assert_eq!(e.global::<f32>(PARALLAX_01201BA0), 0.25);
        assert_eq!(
            calls(&e, SLOTS + 0x78),
            vec![vec![
                map,
                shader,
                scene,
                0,
                0,
                0,
                record,
                0,
                scene + 0x68,
                DEFAULT_011F4288,
                0,
                1
            ]]
        );
        assert_eq!(calls(&e, SLOTS + 0x24), vec![vec![target, other]]);
        assert!(calls(&e, STORE_PLANES_00B71AD0).is_empty());
        assert_eq!(flag(&e, AMBIENT_ENABLE_012010C4), 1);
    }

    #[test]
    fn fn_00bbef90_shadow_passes_set_the_clip_planes() {
        let (mut e, _, _) = set_up_engine();
        let this = with_vtable(&mut e, 0xa4);
        let renderer = e.mem.alloc(0xa00);
        e.mem.set_u32(this + 0x18, renderer);
        let (record, _) = record(&mut e);
        let holder = e.global::<u32>(SCENE_HOLDER_011F91E0);
        let lights = e.mem.alloc(4);
        let light = e.mem.alloc(0x210);
        e.mem.set_u32(lights, light);
        e.mem.set_u32(holder + 0xc, lights);
        let device = with_vtable(&mut e, 8);
        let renderer_holder = e.mem.alloc(0x28c);
        e.mem.set_u32(renderer_holder + 0x288, device);
        e.set_global::<u32>(RENDERER_011F4748, renderer_holder);
        e.set_global::<u32>(PASS_011F91E4, 0x1c4);
        e.register(DOT, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        e.call(0x00bb_ef90, &args![this, record]);
        let set = calls(&e, SLOTS + 0xdc);
        assert_eq!(set.len(), 6);
        assert_eq!(set[5][..2], [device, 5]);
        assert_eq!(calls(&e, DOT).len(), 3);
        assert_eq!(calls(&e, STORE_PLANES_00B71AD0)[0][0], light);
        assert_eq!(calls(&e, PLANE_CALL_00EE6DCE).len(), 6);
        // A refusing light: its stored planes at +0x1a8.
        e.mem.set_u8(light + 0x208, 1);
        e.call(0x00bb_ef90, &args![this, record]);
        let set = calls(&e, SLOTS + 0xdc);
        assert_eq!(set.len(), 12);
        assert_eq!(set[6][2], light + 0x1a8);
        assert_eq!(calls(&e, STORE_PLANES_00B71AD0).len(), 1);
    }

    /// The loaders' rig: the loader doubles, the profiles answering fixed strings, the pass
    /// pointer assignment recorded and done.
    fn loader_engine() -> Engine {
        let mut e = loader_rig();
        for page in [0x0120_0000u32, 0x011f_9000] {
            e.map(page, 0x1000);
        }
        e.register(VERTEX_PROFILE_00B4F3E0, |_, _| 0x0e01_0000u32.into_ret());
        e.register(PIXEL_PROFILE_00B4F380, |_, _| 0x0e02_0000u32.into_ret());
        e.register(NI_POINTER_ASSIGN_00A5F860, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn vertex_loader_compiles_57_shaders() {
        let mut e = loader_engine();
        e.set_global::<u8>(SHADER_MODEL_3_011F94A4, 1);
        let this = e.mem.alloc(0xa4);
        e.call(0x00bb_96f0, &args![this]);
        assert_eq!(calls(&e, MEMSET).len(), 55);
        let formats = calls(&e, FORMAT_00406D00);
        assert_eq!(formats.len(), 57);
        assert_eq!(formats[56][2..], [VERTEX_NAME_FORMAT_010B9F78, 56]);
        let assigned = calls(&e, NI_POINTER_ASSIGN_00A5F860);
        assert_eq!(assigned.len(), 53);
        for index in 0..57u32 {
            let shader = e.global::<u32>(VERTEX_SHADERS_01200DD8 + 4 * index);
            let seen = compiled(&e, shader);
            assert_eq!(seen.file, VERTEX_SOURCES[index as usize][0]);
            let want = if matches!(index, 1 | 4 | 7 | 10) {
                PROFILE_VS_3_0_010A93DC
            } else {
                0x0e01_0000
            };
            assert_eq!(seen.profile, want, "{index}");
        }
        let skinned = compiled(&e, e.global::<u32>(VERTEX_SHADERS_01200DD8 + 0xc));
        assert_eq!(skinned.macros[..3], [0x010a_ed64, 0x0101_1584, 0]);
    }

    /// The pixel loader's compile double: answers a new object recording the profile, the
    /// format and the first bytes of the first two macro values.
    fn pixel_engine(mode: u32) -> Engine {
        let mut e = loader_engine();
        e.map(0x010b_a000, 0x1000);
        e.mem.write(TWENTY_010BA030, b"20\0");
        e.register_double(RENDERER_MODE_004DC060, move |_, _| mode.into_ret());
        e.register(SPRINTF, |e, a| {
            e.mem.write(a[0], b"512\0");
            Ret::default()
        });
        e.register(COMPILE_PIXEL_SHADER, |e, a| {
            let shader = object(e, 0);
            e.mem.set_u32(shader + 0x10, a[3]);
            let name = e.mem.u32(a[4]);
            e.mem.set_u32(shader + 0xc, name);
            for (index, offset) in [4u32, 0xc].iter().enumerate() {
                let value = e.mem.u32(a[2] + offset);
                let byte = if value >= 0x0200_0000 {
                    e.mem.u8(value)
                } else {
                    0
                };
                e.mem.set_u8(shader + 0x18 + index as u32, byte);
            }
            shader.into_ret()
        });
        e
    }

    fn seen(e: &Engine, slot: u32) -> (u32, u32, u8, u8) {
        let shader = e.global::<u32>(slot);
        (
            e.mem.u32(shader + 0x10),
            e.mem.u32(shader + 0xc),
            e.mem.u8(shader + 0x18),
            e.mem.u8(shader + 0x19),
        )
    }

    #[test]
    fn pixel_loader_mode_7() {
        let mut e = pixel_engine(7);
        let this = e.mem.alloc(0xa4);
        e.call(0x00bb_a5d0, &args![this]);
        assert_eq!(calls(&e, MEMSET).len(), 45);
        assert_eq!(calls(&e, COMPILE_PIXEL_SHADER).len(), 50 + 0x18);
        assert_eq!(calls(&e, SPRINTF)[0][1..], [FORMAT_INT_01020774, 0]);
        let profile = 0x0e02_0000;
        // Block 2: first value "4", second "2"; its low-light variant: first "2".
        assert_eq!(
            seen(&e, PIXEL_SHADERS_01200C48 + 8),
            (profile, PIXEL_NAME_FORMAT_010BA024, b'4', b'2')
        );
        assert_eq!(
            seen(&e, LOW_LIGHT_SHADERS_01200D10 + 8),
            (profile, LOW_LIGHT_NAME_FORMAT_010BA014, b'2', b'2')
        );
        // Block 30's first value is the shadow map size string.
        assert_eq!(seen(&e, PIXEL_SHADERS_01200C48 + 4 * 30).2, b'5');
        // Mode 7 assigns inline; the low-light block 5 through `00a5f860` would need the
        // setting clear: it is clear here, so block 5's low-light one is inline with "20".
        let assigned = calls(&e, NI_POINTER_ASSIGN_00A5F860);
        assert_eq!(assigned.len(), 0x18 - 4);
        assert_eq!(seen(&e, LOW_LIGHT_SHADERS_01200D10 + 4 * 5).3, b'2');
    }

    #[test]
    fn pixel_loader_other_modes_and_the_setting() {
        let mut e = pixel_engine(0);
        e.set_global::<u8>(SHADER_MODEL_3_011F94A4, 1);
        let this = e.mem.alloc(0xa4);
        e.call(0x00bb_a5d0, &args![this]);
        assert_eq!(
            seen(&e, PIXEL_SHADERS_01200C48 + 4 * 5).0,
            PROFILE_PS_3_0_010A93B4
        );
        assert_eq!(seen(&e, PIXEL_SHADERS_01200C48 + 4 * 6).0, 0x0e02_0000);
        assert_eq!(
            seen(&e, LOW_LIGHT_SHADERS_01200D10 + 4 * 0x13).0,
            PROFILE_PS_3_0_010A93B4
        );
        // Mode 0: every main shader through `00a5f860`, the low-light ones except the four.
        assert_eq!(calls(&e, NI_POINTER_ASSIGN_00A5F860).len(), 50 + 0x18 - 4);
        let mut e = pixel_engine(5);
        let this = e.mem.alloc(0xa4);
        e.call(0x00bb_a5d0, &args![this]);
        // Mode 5 without the setting: block 7 takes "20" and is assigned inline.
        assert_eq!(seen(&e, PIXEL_SHADERS_01200C48 + 4 * 7).3, b'2');
        assert_eq!(calls(&e, NI_POINTER_ASSIGN_00A5F860).len(), 46 + 0x18 - 4);
    }

    #[test]
    fn create_constant_maps_makes_the_shared_maps_once() {
        let mut e = crate::units::bsshader::particleshader::constant_map_rig::map_rig();
        for page in [0x0120_0000u32, 0x0120_1000] {
            e.map(page, 0x1000);
        }
        e.register(NI_POINTER_ASSIGN_00A5F860, |e, a| {
            ni_pointer_assign(e, a[0], a[1]);
            a[0].into_ret()
        });
        // Each lookup answers a new entry object (count 0) noting the name at +8.
        e.register(
            crate::units::bsshader::particleshader::constant_map_rig::MAP_LOOKUP,
            |e, a| {
                let entry = object(e, 0);
                e.mem.set_u32(entry + 8, a[1]);
                entry.into_ret()
            },
        );
        let this = e.mem.alloc(0xa4);
        e.mem.set_u32(this + 0x18, 0x0e50_0000);
        e.call(0x00bb_f5a0, &args![this]);
        assert_eq!(e.global::<u8>(VERTEX_MAP_MADE_01201F28), 1);
        // The handles: the entry objects the lookups answered for each name.
        let name = |e: &Engine, slot: u32| e.mem.u32(e.global::<u32>(slot) + 8);
        assert_eq!(name(&e, 0x0120_1080), 0x010a_1e50);
        assert_eq!(name(&e, 0x0120_10b8), 0x010b_a2e4);
        assert_eq!(name(&e, 0x0120_10c8), 0x010a_ff58);
        assert_eq!(name(&e, 0x0120_10e8), 0x010b_a26c);
        assert_eq!(e.mem.u32(e.global::<u32>(0x0120_10c8) + 4), 1);
        use crate::units::bsshader::particleshader::constant_map_rig::{entries, predefined};
        use crate::units::bsshader::particleshader::constant_maps::CONSTANT_MAP_CONSTRUCT;
        // Two vertex maps, two pixel maps, then the +0x70 and +0x6c maps.
        assert_eq!(calls(&e, CONSTANT_MAP_CONSTRUCT).len(), 6);
        let vertex = e.mem.u32(this + 0x34);
        assert_eq!(e.mem.u32(this + 0x78), vertex);
        assert_eq!(e.mem.u32(this + 0x74), e.mem.u32(this + 0x30));
        let second = e.mem.u32(this + 0x70);
        assert_eq!(predefined(&e, second).len(), 2);
        assert_eq!(entries(&e, second).len(), 7);
        let third = e.mem.u32(this + 0x6c);
        assert_eq!(entries(&e, third)[2][0], 0x010b_a250);
        // Made already: only the empty slots are filled again.
        e.call(0x00bb_f5a0, &args![this]);
        assert_eq!(calls(&e, CONSTANT_MAP_CONSTRUCT).len(), 6);
    }
}

#[cfg(test)]
mod n070_tests {
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

    /// Slot doubles: `SLOTS + offset`.
    const SLOTS: u32 = 0x0ed0_0000;

    fn object(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x140);
        for offset in (0..0x140u32).step_by(4) {
            e.mem.set_u32(vtable + offset, SLOTS + offset);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [0x0120_0000u32, 0x0120_1000, 0x011f_9000, 0x0126_f000] {
            e.map(page, 0x1000);
        }
        for offset in (0..0x140u32).step_by(4) {
            e.register(SLOTS + offset, |_, _| Ret::default());
        }
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
        e.call_log = Some(vec![]);
        e
    }

    /// A pass with `count` stages.
    fn pass(e: &mut Engine, count: u32) -> (u32, Vec<u32>) {
        let stages: Vec<u32> = (0..count)
            .map(|_| {
                let stage = e.mem.alloc(0x10);
                e.mem.set_u32(stage + 8, 0x99);
                stage
            })
            .collect();
        let array = e.mem.alloc(4 * count);
        for (i, stage) in stages.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *stage);
        }
        let pass = e.mem.alloc(0x40);
        e.mem.set_u32(pass + 0x18, count);
        e.mem.set_u32(pass + 0x24, array);
        (pass, stages)
    }

    #[test]
    fn fn_00bbe120_pass_index_follows_the_jump_tables() {
        for (pass, index) in [
            (10u32, 1u32),
            (15, 6),
            (9, 9u32.wrapping_sub(0x178)),
            (0x1d3, 0x59),
            (0x1d4, 0x5a),
            (0x1da, 0x60),
            (0x1db, 0x63),
            (0x2da, 0x68),
            (0x2db, 0x69),
            (0x2dc, 0x164),
            (0x2e0, 0x68),
            (0x2e1, 0x69),
            (0x2ec, 0x68),
            (0x2ed, 0x69),
            (0x2ee, 0x176),
            (0x180, 8),
        ] {
            assert_eq!(pass_index_00bbe120(pass), index, "pass {pass:#x}");
        }
    }

    #[test]
    fn fn_00c0cf10_runs_the_pass_constant_setters() {
        let mut e = rig();
        let setters = [
            CONSTANTS_00C0BAE0,
            CONSTANTS_00C0BB20,
            CONSTANTS_00C0BB50,
            CONSTANTS_00C0BBA0,
            CONSTANTS_00C0BBD0,
            CONSTANTS_00C0BC00,
            CONSTANTS_00C0BC90,
            CONSTANTS_00C0BCF0,
            SET_UP_00BBDDA0,
            SET_UP_00BBDCD0,
        ];
        for setter in setters {
            e.register(setter, |_, _| Ret::default());
        }
        let this = object(&mut e, 0x80);
        let state = e.mem.alloc(0x30);
        let entries = e.mem.alloc(0x20);
        e.mem.set_u32(state + 0x24, entries);
        let entry: Vec<u32> = (0..3).map(|_| e.mem.alloc(0xc)).collect();
        for (i, item) in entry.iter().enumerate() {
            e.mem.set_u32(entries + 0x10 + 4 * i as u32, *item);
        }
        let property = e.mem.alloc(0xc0);
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 0x77);
        e.mem.set_u32(property + 0xb4, word);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first + 4, 0x55);
        e.mem.set_u32(property + 0xac, first);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second + 4, 0x66);
        e.mem.set_u32(property + 0xb0, second);
        e.set_global::<u32>(RENDER_STATE_0126F74C, state);
        e.set_global::<u32>(STATE_WORD_SOURCE_011F9530, 0x99);
        let entry_words =
            |e: &Engine| -> Vec<u32> { entry.iter().map(|item| e.mem.u32(item + 8)).collect() };
        let run = |e: &mut Engine, pass: u32| -> Vec<(u32, Vec<u32>)> {
            let start = e.call_log.as_ref().unwrap().len();
            e.call(0x00c0_cf10, &args![this, pass, property]);
            e.call_log.as_ref().unwrap()[start..]
                .iter()
                .filter(|(a, _)| setters.contains(a))
                .cloned()
                .collect()
        };
        let (s, p) = (state, property);
        // Passes below 0xe, 0x10 to 0x180 and above 0x1e1: nothing.
        for pass in [0xdu32, 0x10, 0x180, 0x1e2, 0x188, 0x18e] {
            assert!(run(&mut e, pass).is_empty(), "{pass:#x}");
        }
        assert_eq!(entry_words(&e), vec![0, 0, 0]);
        assert_eq!(run(&mut e, 0xe), vec![(SET_UP_00BBDDA0, vec![this, p, s])]);
        assert_eq!(run(&mut e, 0xf), vec![(SET_UP_00BBDDA0, vec![this, p, s])]);
        assert!(run(&mut e, 0x181).is_empty());
        assert_eq!(entry_words(&e), vec![0x99, 0x77, 0]);
        let bae0 = (CONSTANTS_00C0BAE0, vec![this, s, p, 4]);
        let bb20 = (CONSTANTS_00C0BB20, vec![this, s, p]);
        let bb50 = (CONSTANTS_00C0BB50, vec![this, s, p, 1]);
        let bbd0 = (CONSTANTS_00C0BBD0, vec![this, s]);
        assert_eq!(run(&mut e, 0x182), vec![bae0.clone()]);
        assert_eq!(run(&mut e, 0x185), vec![bb50.clone()]);
        assert_eq!(run(&mut e, 0x190), vec![bb20.clone()]);
        assert_eq!(run(&mut e, 0x19d), vec![bbd0.clone()]);
        assert_eq!(run(&mut e, 0x19f), vec![bb20.clone(), bbd0.clone()]);
        assert_eq!(run(&mut e, 0x1ae), vec![bb20, bbd0.clone()]);
        assert_eq!(run(&mut e, 0x1a0), vec![bbd0.clone(), bae0]);
        assert_eq!(run(&mut e, 0x1a3), vec![bbd0.clone(), bb50]);
        assert_eq!(
            run(&mut e, 0x1bd),
            vec![
                (CONSTANTS_00C0BC00, vec![this, s, p, 0x1bd]),
                (CONSTANTS_00C0BC90, vec![this, s, p]),
                (CONSTANTS_00C0BCF0, vec![this, s, p, 0]),
            ]
        );
        assert_eq!(
            run(&mut e, 0x1cc),
            vec![(CONSTANTS_00C0BBA0, vec![this, s, p])]
        );
        assert_eq!(
            run(&mut e, 0x1e0),
            vec![(SET_UP_00BBDCD0, vec![this, p, s])]
        );
        // The entry cases: +0x18 from +0xb4, +0x10/+0x14 from +0xac/+0xb0.
        e.mem.set_u32(word, 0x78);
        assert!(run(&mut e, 0x186).is_empty());
        assert_eq!(entry_words(&e), vec![0x99, 0x77, 0x78]);
        assert!(run(&mut e, 0x184).is_empty());
        assert_eq!(entry_words(&e), vec![0x55, 0x66, 0x78]);
        e.mem.set_u32(word, 0x79);
        assert_eq!(run(&mut e, 0x1a4), vec![bbd0.clone()]);
        assert_eq!(entry_words(&e), vec![0x55, 0x66, 0x79]);
        e.mem.set_u32(first + 4, 0x56);
        assert_eq!(run(&mut e, 0x1a2), vec![bbd0]);
        assert_eq!(entry_words(&e), vec![0x56, 0x66, 0x79]);
    }

    #[test]
    fn fn_00bbe120_sets_flags_fog_and_eye_constants() {
        let mut e = rig();
        for page in [0x011f_a000u32, 0x011f_4000, 0x011a_9000, 0x010a_d000] {
            e.map(page, 0x1000);
        }
        let this = object(&mut e, 0x40);
        let first = object(&mut e, 8);
        let second = object(&mut e, 8);
        e.mem.set_u32(this + 0x34, first);
        e.mem.set_u32(this + 0x30, second);
        // 0x1e0 and 0x1e1: nothing.
        e.call(0x00bb_e120, &args![this, 0x1e0u32]);
        e.call(0x00bb_e120, &args![this, 0x1e1u32]);
        assert!(calls(&e, SLOTS + 0x24).is_empty());
        // Pass 0x1d5: index 0x5b.
        let index = 0x5bu32;
        e.set_global(MASKS_01200A28 + 4 * index, 0b1000u32);
        e.set_global(MASKS_01200868 + 4 * index, 0b100_0000_0000u32);
        let flagged = e.mem.alloc(0xc);
        e.set_global(SHADERS_01201080 + 8, flagged);
        let last = e.mem.alloc(0xc);
        e.mem.set_u8(last + 8, 7);
        e.set_global(FLAGS_012010C4 + 4 * 9, last);
        let fog_enable = e.mem.alloc(0xc);
        let eye_enable = e.mem.alloc(0xc);
        e.set_global(FOG_ENABLE_012010BC, fog_enable);
        e.set_global(EYE_ENABLE_012010C0, eye_enable);
        e.call(0x00bb_e120, &args![this, 0x1d5u32]);
        assert_eq!(calls(&e, SLOTS + 0x24), vec![vec![first], vec![second]]);
        assert_eq!(e.mem.u8(flagged + 8), 1);
        assert_eq!(e.global::<u8>(FLAGS_01200854 + 2), 1);
        assert_eq!(e.global::<u8>(FLAGS_01200854 + 1), 0);
        assert_eq!(e.mem.u8(last + 8), 1);
        assert_eq!(e.global::<u8>(FLAGS_01200848 + 9), 1);
        assert_eq!(e.global::<u8>(FLAGS_01200848), 0);
        // Neither enable byte set: the constants stay.
        assert_eq!(e.global::<u32>(FOG_CONSTANT_012013B0), 0);
        assert!(calls(&e, GET_FOG_PROPERTY_00B55520).is_empty());
        // Fog with start and end 0, and the eye.
        let fog = e.mem.alloc(0x70);
        e.set_global(0x0120_0f00, fog);
        e.register(GET_FOG_PROPERTY_00B55520, |e, _| {
            e.mem.u32(0x0120_0f00).into_ret()
        });
        e.set_global(FOG_KIND_011F91C4, 2u8);
        e.mem.set_f32(fog + 0x60, 3.5);
        e.set_global(FAR_010AD6E0, 500000.0f32);
        for i in 0..4u32 {
            e.set_global(DEFAULT_COLOUR_011A9BD0 + 4 * i, 0x100 + i);
        }
        for (i, (eye, origin)) in [(10.0f32, 1.0f32), (20.0, 2.0), (30.5, 0.5)]
            .iter()
            .enumerate()
        {
            e.set_global(EYE_011FA2A0 + 4 * i as u32, *eye);
            e.set_global(ORIGIN_011F474C + 4 * i as u32, *origin);
        }
        // The enable objects are shaders 15 and 16 of `01201080`: bits 16 and 17 set their bytes.
        e.set_global(MASKS_01200A28 + 4 * index, 0b11_0000_0000_0000_1000u32);
        e.call(0x00bb_e120, &args![this, 0x1d5u32]);
        assert_eq!(calls(&e, GET_FOG_PROPERTY_00B55520), vec![vec![2]]);
        let fog_words: Vec<f32> = (0..4u32)
            .map(|i| e.global::<f32>(FOG_CONSTANT_012013B0 + 4 * i))
            .collect();
        assert_eq!(fog_words, vec![500000.0, 0.0, 3.5, 0.0]);
        let colour: Vec<u32> = (0..4u32)
            .map(|i| e.global::<u32>(FOG_COLOUR_012013C0 + 4 * i))
            .collect();
        assert_eq!(colour, vec![0x100, 0x101, 0x102, 0x103]);
        let eye: Vec<f32> = (0..4u32)
            .map(|i| e.global::<f32>(EYE_CONSTANT_012013D0 + 4 * i))
            .collect();
        assert_eq!(eye, vec![9.0, 18.0, 30.0, 1.0]);
        // A start of NaN counts as not 0: (end, end - start, power, 0) and (colour, 1.0), the
        // signalling NaN colour word quieted.
        e.mem.set_f32(fog + 0x30, 100.0);
        e.mem.set_f32(fog + 0x2c, 40.0);
        e.mem.set_f32(fog + 0x20, 0.25);
        e.mem.set_u32(fog + 0x24, 0x7f80_0001);
        e.mem.set_f32(fog + 0x28, 0.75);
        e.call(0x00bb_e120, &args![this, 0x1d5u32]);
        let fog_words: Vec<f32> = (0..4u32)
            .map(|i| e.global::<f32>(FOG_CONSTANT_012013B0 + 4 * i))
            .collect();
        assert_eq!(fog_words, vec![100.0, 60.0, 3.5, 0.0]);
        assert_eq!(e.global::<f32>(FOG_COLOUR_012013C0), 0.25);
        assert_eq!(e.global::<u32>(FOG_COLOUR_012013C0 + 4), 0x7fc0_0001);
        assert_eq!(e.global::<f32>(FOG_COLOUR_012013C0 + 8), 0.75);
        assert_eq!(e.global::<f32>(FOG_COLOUR_012013C0 + 0xc), 1.0);
        e.mem.set_f32(fog + 0x30, 0.0);
        e.mem.set_u32(fog + 0x2c, 0xffc0_0000);
        e.call(0x00bb_e120, &args![this, 0x1d5u32]);
        assert_eq!(e.global::<f32>(FOG_CONSTANT_012013B0), 0.0);
        assert_eq!(e.global::<u32>(FOG_CONSTANT_012013B0 + 4), 0xffc0_0000);
    }

    #[test]
    fn fn_00bbb7f0_gives_the_passes_to_slot_0x128() {
        let mut e = rig();
        for i in 0..0x70u32 {
            e.set_global(PASSES_01200EC0 + 4 * i, 0x100 + i);
        }
        let this = object(&mut e, 4);
        e.call(0x00bb_b7f0, &args![this]);
        let given: Vec<u32> = calls(&e, SLOTS + 0x128).iter().map(|a| a[1]).collect();
        assert_eq!(given, (0x100..0x170).collect::<Vec<u32>>());
    }

    #[test]
    fn fn_00bbb820_makes_the_picked_pass_current() {
        let mut e = rig();
        let pass = e.mem.alloc(0x40);
        e.mem.set_u32(pass + 0x30, 0x3030);
        e.set_global(0x0120_0f00, pass);
        e.register(PICK_PASS_00C0BD40, |e, _| e.mem.u32(0x0120_0f00).into_ret());
        e.register(STATES_00E7EED0, |_, _| 7u32.into_ret());
        assert_eq!(e.call(0x00bb_b820, &args![0x10u32, 1u32, 2u32]).u32(), 7);
        assert_eq!(calls(&e, PICK_PASS_00C0BD40), vec![vec![0x10, 1, 2]]);
        assert_eq!(e.global::<u32>(RENDER_STATE_0126F74C), pass);
        assert_eq!(calls(&e, STATES_00E7EED0), vec![vec![0x3030]]);
    }

    #[test]
    fn fn_00bbc610_clears_stages_from_the_first() {
        let mut e = rig();
        let (pass, stages) = pass(&mut e, 4);
        let array = e.mem.u32(pass + 0x24);
        e.mem.set_u32(array + 8, 0);
        e.call(0x00bb_c610, &args![0u32, pass, 1u32]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x99);
        assert_eq!(e.mem.u32(stages[1] + 8), 0);
        assert_eq!(e.mem.u32(stages[3] + 8), 0);
        assert_eq!(e.mem.u32(stages[2] + 8), 0x99);
    }

    #[test]
    fn fn_00bbd000_sets_the_bone_constants() {
        let mut e = rig();
        e.register(RENDERER_CALL_00E6FE30, |_, _| Ret::default());
        e.register(SLOTS + 0x70, |_, _| 0x7070u32.into_ret());
        e.set_global(RENDERER_011F9508, 0x9508u32);
        let shader = object(&mut e, 4);
        let pass = e.mem.alloc(0x60);
        e.mem.set_u32(pass + 0x5c, shader);
        e.set_global(RENDER_STATE_0126F74C, pass);
        let map = object(&mut e, 4);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x34, map);
        e.mem.set_u32(this + 0x70, map);
        let geometry = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0x20, 0x20);
        e.mem.set_u32(geometry + 0x28, 0x28);
        let skin = e.mem.alloc(0x24);
        e.mem.set_u32(skin + 4, 0x44);
        e.mem.set_u16(skin + 0x20, 3);
        let words = |geometry: u32, skin: u32| {
            args![this, 0u32, geometry, skin, 0u32, 0u32, 0u32, 0x77u32, 0u32]
        };
        assert_eq!(e.call(0x00bb_d000, &words(geometry, skin)).u32(), 0);
        assert_eq!(
            calls(&e, RENDERER_CALL_00E6FE30),
            vec![vec![0x9508, geometry, 0x77, 0, 3, 1]]
        );
        assert_eq!(
            calls(&e, SLOTS + 0x70),
            vec![vec![map, NAME_BONE_MATRIX3_010A1CAC]]
        );
        assert_eq!(
            calls(&e, SLOTS + 0x78),
            vec![vec![shader, 0x7070, 0x28, 3, 0x20, 0x44]]
        );
        // A set skinning object with its byte: the object is the entry.
        let skinning = e.mem.alloc(0xc);
        e.mem.set_u8(skinning + 8, 1);
        e.set_global(SKIN_01201090, skinning);
        e.mem.set_u32(this + 0x34, 0);
        e.call(0x00bb_d000, &words(geometry, skin));
        assert_eq!(calls(&e, SLOTS + 0x78)[1][1], skinning);
        // Neither: nothing.
        e.mem.set_u8(skinning + 8, 0);
        e.call(0x00bb_d000, &words(geometry, skin));
        assert_eq!(calls(&e, SLOTS + 0x78).len(), 2);
        // No skin data or +0x38 set: nothing.
        e.mem.set_u8(skinning + 8, 1);
        e.call(0x00bb_d000, &words(geometry, 0));
        e.mem.set_u32(this + 0x38, 1);
        e.call(0x00bb_d000, &words(geometry, skin));
        assert_eq!(calls(&e, SLOTS + 0x78).len(), 2);
    }

    #[test]
    fn fn_00bbd880_selects_maps_for_kinds_0x1e0_and_0x1e1() {
        let mut e = rig();
        let shader = e.mem.alloc(0x80);
        e.set_global(0x0120_0f00, shader);
        e.register(GET_SHADER_00B55560, |e, _| {
            e.mem.u32(0x0120_0f00).into_ret()
        });
        e.register(SELECT_MAP_00C663D0, |_, _| Ret::default());
        e.call(0x00bb_d880, &args![0x1e1u32]);
        assert_eq!(
            calls(&e, SELECT_MAP_00C663D0),
            vec![
                vec![shader + 0x30, shader + 0x6c],
                vec![shader + 0x34, shader + 0x70]
            ]
        );
        assert_eq!(calls(&e, GET_SHADER_00B55560), vec![vec![0x1d]]);
    }

    #[test]
    fn fn_00bbd880_sets_flags_and_the_render_state() {
        let mut e = rig();
        e.register(NEW_RENDER_STATES_00E7F5D0, |e, _| {
            e.mem.alloc(0x10).into_ret()
        });
        e.register(SET_RENDER_STATE_00E7F430, |_, _| Ret::default());
        let shader = e.mem.alloc(0x80);
        let held = object(&mut e, 8);
        e.mem.set_u32(held + 4, 1);
        e.mem.set_u32(shader + 0x30, held);
        let new = object(&mut e, 8);
        e.mem.set_u32(shader + 0x74, new);
        e.set_global(0x0120_0f00, shader);
        e.register(GET_SHADER_00B55560, |e, _| {
            e.mem.u32(0x0120_0f00).into_ret()
        });
        // Kind 0x1d4: index 0x5c, render state 3.
        let index = 0x1d4u32 - 0x178;
        e.set_global(MASKS_01200A28 + 4 * index, 0b100u32);
        e.set_global(MASKS_01200868 + 4 * index, 0b10u32);
        let flagged = e.mem.alloc(0xc);
        e.set_global(SHADERS_01201080 + 4, flagged);
        let other = e.mem.alloc(0xc);
        e.mem.set_u8(other + 8, 9);
        e.set_global(FLAGS_012010C4, other);
        let pass = e.mem.alloc(0x40);
        e.set_global(PASSES_01200EC0 + 4 * index, pass);
        e.call(0x00bb_d880, &args![0x1d4u32]);
        assert_eq!(e.mem.u32(shader + 0x30), new);
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![held]]);
        assert_eq!(e.mem.u32(new + 4), 1);
        // Bit 2 belongs to shader 1.
        assert_eq!(e.mem.u8(flagged + 8), 1);
        assert_eq!(e.global::<u8>(FLAGS_01200854 + 1), 1);
        assert_eq!(e.global::<u8>(FLAGS_01200854), 0);
        assert_eq!(e.mem.u8(other + 8), 1);
        assert_eq!(e.global::<u8>(FLAGS_01200848), 1);
        let block = e.mem.u32(pass + 0x30);
        assert_eq!(
            calls(&e, SET_RENDER_STATE_00E7F430),
            vec![vec![block, 0xa8, 3, 1]]
        );
        // Kind 12: index 3, state 0; kind 0x1d0: 0xf; kind 0x180: 7; kind 0x2ec: index 0x6e, 0.
        for (kind, index, state) in [
            (12u32, 3u32, 0u32),
            (0x1d0, 0x58, 0xf),
            (0x180, 8, 7),
            (0x2ec, 0x6e, 0),
        ] {
            let pass = e.mem.alloc(0x40);
            e.set_global(PASSES_01200EC0 + 4 * index, pass);
            e.call(0x00bb_d880, &args![kind]);
            let block = e.mem.u32(pass + 0x30);
            assert_eq!(
                calls(&e, SET_RENDER_STATE_00E7F430).last().unwrap(),
                &vec![block, 0xa8, state, 1]
            );
        }
        // The setting bit forces 0.
        e.set_global(SETTING_011F941C, 1u8);
        let pass = e.mem.alloc(0x40);
        e.set_global(PASSES_01200EC0 + 4 * 8, pass);
        e.call(0x00bb_d880, &args![0x180u32]);
        assert_eq!(calls(&e, SET_RENDER_STATE_00E7F430).last().unwrap()[2], 0);
    }

    fn textured(e: &mut Engine) -> (u32, u32) {
        let object = e.mem.alloc(0x74);
        let textures = e.mem.alloc(4);
        let geometry = e.mem.alloc(0xe0);
        e.mem.set_u32(geometry + 0xb0, textures);
        e.mem.set_u32(geometry + 0xdc, object);
        (geometry, object)
    }

    #[test]
    fn fn_00bbdcd0_sets_the_two_textures() {
        let mut e = rig();
        e.register(STAGE_SET_00E7DE90, |_, _| Ret::default());
        e.register(RENDER_STATE_00B97FF0, |_, _| Ret::default());
        e.register(INDEX_MAP_00BA5060, |_, a| (a[0] + 100).into_ret());
        e.set_global(TEXTURE_011F9510, 0x9510u32);
        e.set_global(TEXTURE_011F9518, 0x9518u32);
        e.set_global(TEXTURE_011F952C, 0x952cu32);
        let this = object(&mut e, 4);
        let (geometry, object) = textured(&mut e);
        e.mem.set_u32(object + 0x5c, 1);
        e.mem.set_u32(object + 0x60, 2);
        let (pass, stages) = pass(&mut e, 3);
        e.call(0x00bb_dcd0, &args![this, geometry, pass]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x9510);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x952c);
        assert_eq!(e.mem.u32(stages[2] + 8), 0);
        assert_eq!(calls(&e, RENDER_STATE_00B97FF0), vec![vec![101, 102, 0]]);
        assert_eq!(calls(&e, SLOTS + 0x108), vec![vec![this]]);
        // Bit 7 of +0x20 picks the other default; own textures win.
        e.mem.set_u32(geometry + 0x20, 0x80);
        e.call(0x00bb_dcd0, &args![this, geometry, pass]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x9518);
        e.mem.set_u32(e.mem.u32(geometry + 0xb0), 0x1111);
        e.mem.set_u32(object + 8, 0x2222);
        e.call(0x00bb_dcd0, &args![this, geometry, pass]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x1111);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x2222);
        // Without the object: nothing.
        e.mem.set_u32(geometry + 0xdc, 0);
        e.call(0x00bb_dcd0, &args![this, geometry, pass]);
        assert_eq!(calls(&e, SLOTS + 0x108).len(), 3);
    }

    #[test]
    fn fn_00bbdda0_sets_the_texture_and_state() {
        let mut e = rig();
        e.register(STAGE_SET_00E7DE90, |_, _| Ret::default());
        e.register(RENDER_STATE_00B97F20, |_, _| Ret::default());
        e.set_global(TEXTURE_011F952C, 0x952cu32);
        let this = object(&mut e, 4);
        let (geometry, object) = textured(&mut e);
        e.mem.set_u32(object + 0x70, 0x70);
        let (pass, stages) = pass(&mut e, 2);
        e.call(0x00bb_dda0, &args![this, geometry, pass]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x952c);
        assert_eq!(e.mem.u32(stages[1] + 8), 0);
        assert_eq!(calls(&e, RENDER_STATE_00B97F20), vec![vec![4, 0x70, 0]]);
        e.mem.set_u32(object + 0x6c, 0x6c6c);
        e.call(0x00bb_dda0, &args![this, geometry, pass]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0x6c6c);
    }

    #[test]
    fn fn_00bbde20_sets_light_and_fog_constants() {
        let mut e = rig();
        let (geometry, object) = textured(&mut e);
        for index in 0..8u32 {
            e.mem.set_u32(object + 0xc + 4 * index, 0x10 + index);
        }
        e.mem.set_u32(object + 0x4c, 0x7f80_0001);
        e.mem.set_f32(object + 0x54, 2.0);
        let fog = e.mem.alloc(0x64);
        e.mem.set_f32(fog + 0x2c, 10.0);
        e.mem.set_f32(fog + 0x30, 30.0);
        e.mem.set_f32(fog + 0x20, 0.5);
        e.mem.set_f32(fog + 0x60, 2.0);
        e.set_global(0x0120_0f00, fog);
        e.register(GET_FOG_PROPERTY_00B55520, |e, _| {
            e.mem.u32(0x0120_0f00).into_ret()
        });
        e.set_global(FOG_01200C28 + 0x18, 5u32);
        e.call(0x00bb_de20, &args![0u32, 0u32, geometry, 0u32]);
        assert_eq!(e.global::<u32>(LIGHT_01200BF8 + 0x1c), 0x17);
        assert_eq!(e.global::<u32>(LIGHT_01200BE8), 0x7fc0_0001);
        assert_eq!(e.global::<f32>(LIGHT_01200C18), 2.0);
        assert_eq!(e.global::<f32>(LIGHT_01200C18 + 4), 1.0);
        assert_eq!(e.global::<u32>(FOG_01200C28 + 0x18), 0);
        assert!(calls(&e, GET_FOG_PROPERTY_00B55520).is_empty());
        e.call(0x00bb_de20, &args![0u32, 1u32, geometry, 0u32]);
        let fogs: Vec<f32> = (0..8)
            .map(|i| e.global::<f32>(FOG_01200C28 + 4 * i))
            .collect();
        assert_eq!(fogs, vec![0.5, 0.0, 0.0, 0.0, 30.0, 20.0, 2.0, 0.0]);
        // No range: the cleared constants stay.
        e.mem.set_f32(fog + 0x2c, 0.0);
        e.mem.set_f32(fog + 0x30, 0.0);
        e.call(0x00bb_de20, &args![0u32, 1u32, geometry, 0u32]);
        assert_eq!(e.global::<f32>(FOG_01200C28 + 0x10), 0.0);
    }
}
