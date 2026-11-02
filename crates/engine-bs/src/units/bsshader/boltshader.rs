//! `BSShader/boltshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bc0c90` (engine map unit `boltshader.cpp`, unnamed,
/// `thiscall`, `RET 4`: the declaration).
const SHADER_CONSTRUCT_00BC0C90: u32 = 0x00bc_0c90;

// Translated from 00bc10e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `boltshader.cpp`): the creator `GetShader` (`00b55560`)
/// calls for shader 0x19. No arguments. Makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 5 entries, 1 stream),
/// sets its entries through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3,
/// 3, 2, 0), (2, 5, 4, 3, 0), (3, 7, 5, 1, 0)), calls slot +0x94 with (0, 4, 6, 0x80000000,
/// 0, 0xa, 1, 0) and slot +0xb4. Then allocates 0x194 bytes and runs the constructor
/// `00bc0c90` with the declaration (a null allocation is used as it is), calls the shader's
/// slots +0x4c, +0x13c, +0x140, +0x11c, then +0xc0 with the declaration, and answers the
/// shader.
// Translation incomplete: 00bc10e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bc10e0(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 5u32, 1u32])
        .u32();
    for entry in [
        [0u32, 0, 0, 2, 0],
        [1, 3, 3, 2, 0],
        [2, 5, 4, 3, 0],
        [3, 7, 5, 1, 0],
    ] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 4u32, 6u32, 0x8000_0000u32, 0u32, 0xau32, 1u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x194u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BC0C90, &args![block, declaration])
            .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x140, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The bolt shader's vtable (`010ba3b0`; no RTTI locator before it).
const BOLT_VTABLE_010BA3B0: u32 = 0x010b_a3b0;
/// The four words copied twice (`011a9be0`), the shared segment count (`01201f50`, set on
/// first use from the shader model setting `011f91c0`), the `float` 2^32 added to a negative
/// `FILD` (`01099378`) and the `double` the count is scaled by (`010290b0`).
const COLOR_011A9BE0: u32 = 0x011a_9be0;
const SEGMENT_COUNT_01201F50: u32 = 0x0120_1f50;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const QUARTER_010290B0: u32 = 0x0102_90b0;
/// CRT `ceil` (`00ec9e10`, the `double` on the stack, the result in `ST0`).
const CEIL: u32 = 0x00ec_9e10;

// Translated from 00bc0c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `boltshader.cpp`): the bolt shader's constructor (the creator
/// `00bc0fb0` runs it), `thiscall` with one stack word (the declaration). Runs
/// `BSShader::BSShader` (`00be2200`), stores the vtable `010ba3b0`, 0.0 at +0x118 to +0x124 and
/// +0x16c to +0x188, clears +0x168, +0x18c, +0x190, sets +0x68 to 0xf, the byte +0x24 to 1 and
/// +0x6c to 0x242; clears the pass pointer at +0x168, assigns the declaration to the `NiPointer`
/// at +0x28, clears the `NiPointer`s at +0x18c and +0x190, stores 0.0 at +0x74 and +0x118 to
/// +0x124 and copies the four words at `011a9be0` to +0x16c and +0x17c. The segment count at
/// `01201f50` becomes 0x4b (shader model below 2, signed) or 0xeb when it is 0; `ceil` of the
/// count (as unsigned, through the FPU) times the `double` at `010290b0`, rounded to a `float`
/// before and after, truncated (`FISTP qword` with truncation, low word) gives n: 64 * n bytes
/// (the `operator new[]` size check on 4 * n elements of 16) from the memory manager go to +0x70
/// and are cleared, as are 0x40 bytes at +0x128 and 0xa0 bytes at +0x78 (`memset`). Answers
/// `this`.
// Translation incomplete: 00bc0c90 - the C++ exception frame (FS:[0] chain, unwind state 3 for the base and the pointers) is not translated.
pub fn fn_00bc0c90(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, BOLT_VTABLE_010BA3B0);
    for offset in [0x118u32, 0x11c, 0x120, 0x124] {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x168, 0);
    for offset in (0x16cu32..=0x188).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x18c, 0);
    e.mem.set_u32(base + 0x190, 0);
    e.mem.set_u32(base + 0x68, 0xf);
    e.mem.set_u8(base + 0x24, 1);
    e.mem.set_u32(base + 0x6c, 0x242);
    pass_pointer_clear(e, base + 0x168);
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    ni_pointer_clear(e, base + 0x18c);
    ni_pointer_clear(e, base + 0x190);
    e.mem.set_f32(base + 0x74, 0.0);
    for offset in [0x118u32, 0x11c, 0x120, 0x124] {
        e.mem.set_f32(base + offset, 0.0);
    }
    for copy in [0x16cu32, 0x17c] {
        for index in 0..4u32 {
            let word = e.global::<u32>(COLOR_011A9BE0 + 4 * index);
            e.mem.set_u32(base + copy + 4 * index, word);
        }
    }
    if e.global::<u32>(SEGMENT_COUNT_01201F50) == 0 {
        let count = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
            0x4b
        } else {
            0xeb
        };
        e.set_global::<u32>(SEGMENT_COUNT_01201F50, count);
    }
    let count = e.global::<u32>(SEGMENT_COUNT_01201F50);
    let mut loaded = f64::from(count as i32);
    if (count as i32) < 0 {
        loaded = x87_add(loaded, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
    }
    let quarter = f64::from_bits(e.global::<u64>(QUARTER_010290B0));
    let scaled = x87_load32(x87_store32(x87_mul(loaded, quarter)));
    let rounded = e.call(CEIL, &args![scaled]).f64();
    let rounded = x87_load32(x87_store32(rounded));
    // FISTP qword with the rounding control at truncate: the integer indefinite (low word 0)
    // for NaN and values out of range.
    let truncated = rounded.trunc();
    let segments = if truncated.is_nan()
        || truncated < -9_223_372_036_854_775_808.0
        || truncated >= 9_223_372_036_854_775_808.0
    {
        0
    } else {
        truncated as i64 as u32
    };
    let elements = segments.wrapping_mul(4);
    let buffer = allocate_array(e, elements, 0x10);
    e.mem.set_u32(base + 0x70, buffer);
    e.call(MEMSET, &args![buffer, 0u32, elements.wrapping_shl(4)]);
    e.call(MEMSET, &args![base + 0x128, 0u32, 0x40u32]);
    e.call(MEMSET, &args![base + 0x78, 0u32, 0xa0u32]);
    this
}

use crate::units::bsshader::particleshader::constant_maps::{
    add_entry, make_constant_map, EMPTY_STRING_01011584,
};
use crate::units::bsshader::particleshader::{
    pass_drop, pass_hold, pass_release, reference_release, set_up_pass,
};

/// The shared `NiPointer` the destructor clears (`01201f58`).
const SHARED_POINTER_01201F58: u32 = 0x0120_1f58;
/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `BSShader::~BSShader` (engine map, `00be0f10`, `thiscall`, no stack words).
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
/// The sized free `00aa1460` (`cdecl`: block, size).
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00bc0f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `boltshader.cpp`): the bolt shader's destructor, `thiscall`,
/// no stack words. Stores the vtable `010ba3b0`, clears the shared `NiPointer` at `01201f58`,
/// frees the segment buffer at +0x70 (the memory manager's free `00aa4060`) and clears the word,
/// clears the pass pointer at +0x168 and the `NiPointer`s at +0x18c and +0x190; then the
/// members' destructors run on what is left (+0x190, +0x18c, the pass +0x168) and
/// `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00bc0f80 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the members) is not translated.
pub fn fn_00bc0f80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BOLT_VTABLE_010BA3B0);
    ni_pointer_clear(e, SHARED_POINTER_01201F58);
    let buffer = e.mem.u32(base + 0x70);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
    e.mem.set_u32(base + 0x70, 0);
    pass_pointer_clear(e, base + 0x168);
    ni_pointer_clear(e, base + 0x18c);
    ni_pointer_clear(e, base + 0x190);
    for offset in [0x190u32, 0x18c] {
        let object = e.mem.u32(base + offset);
        reference_release(e, object);
    }
    let pass = e.mem.u32(base + 0x168);
    pass_release(e, pass);
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The render states (state, value) of the bolt shader's pass, in order.
const BOLT_RENDER_STATES: [(u32, u32); 10] = [
    (0x16, 1),
    (0xa8, 0xf),
    (0x1b, 1),
    (0x13, 5),
    (0x14, 2),
    (0xf, 0),
    (0x34, 0),
    (7, 1),
    (0x17, 4),
    (0xe, 0),
];

// Translated from 00bc1210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `boltshader.cpp`): the bolt shader's pass set-up, `thiscall`,
/// no stack words. Holds the pass at +0x168 (count at +0x64 up by one when not null), gives it a
/// texture stage when it has none (`SetupDefaultStage(stage, 0, 1, 2, 1)`), the vertex
/// (+0x18c) and pixel (+0x190) shaders and the render states (0x16, 1), (0xa8, 0xf), (0x1b,
/// 1), (0x13, 5), (0x14, 2), (0xf, 0), (0x34, 0), (7, 1), (0x17, 4), (0xe, 0); then drops the
/// stage's reference and the pass's (no null test on the pass).
// Translation incomplete: 00bc1210 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the stage and the pass) is not translated.
pub fn fn_00bc1210(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        let first = e.mem.u32(base + 0x168);
        let pass = pass_hold(e, first);
        set_up_pass(
            e,
            pass,
            local,
            base + 0x18c,
            base + 0x190,
            &BOLT_RENDER_STATES,
        );
        let stage = e.mem.u32(local);
        stage_release(e, stage);
        pass_drop(e, pass);
    });
}

// Translated from 00bc1540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `boltshader.cpp`): the bolt shader's scalar deleting
/// destructor, `thiscall`, one stack word (the flags). Runs the destructor `00bc0f80`; with bit
/// 0 set, frees the 0x194 bytes (`00aa1460`). Answers `this`.
pub fn fn_00bc1540(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bc0f80(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x194u32]);
    }
    this
}

// Translated from 00bc1570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `boltshader.cpp`): `thiscall`, no stack words. Gives the pass
/// pointer at +0x168 a new pass (`00e804d0`) when it is null, sets it up (`00bc1210`) and
/// answers true.
pub fn fn_00bc1570(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    if e.mem.u32(base + 0x168) == 0 {
        e.call(NEW_PASS_00E804D0, &args![base + 0x168]);
    }
    fn_00bc1210(e, this);
    true
}

/// Strings of `BoltShader::CreateConstantMaps` (read in Ghidra): `"WorldViewProjTranspose"`
/// (`010a1e50`), `"fVars"` (`010af454`), `"EyePosition"` (`010af5d8`), `"bolt data"`
/// (`010ba50c`), `"CoreColor"` (`010ba500`), `"GlowColor"` (`010ba4f4`).
const NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const NAME_FVARS_010AF454: u32 = 0x010a_f454;
const NAME_EYE_POSITION_010AF5D8: u32 = 0x010a_f5d8;
const NAME_BOLT_DATA_010BA50C: u32 = 0x010b_a50c;
const NAME_CORE_COLOR_010BA500: u32 = 0x010b_a500;
const NAME_GLOW_COLOR_010BA4F4: u32 = 0x010b_a4f4;

// Translated from 00bc1950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoltShader::CreateConstantMaps` (engine map), `thiscall`, no stack words. When the
/// `NiPointer` at +0x34 is null: a constant map (program type 0, the word at +0x18 as the
/// renderer) into it with the entries (vtable slot +0x8) `"WorldViewProjTranspose"`
/// (0x20000009, 4 registers, no data), `"fVars"` (0x10000009, register 4, 4 registers, 0x40
/// bytes at +0x128), `"EyePosition"` (0x10000007, register 8, 0x10 bytes at +0x118) and `"bolt
/// data"` (0x10000009, register 0xe, one register per segment, 0x10 bytes each, the buffer at
/// +0x70); the segment count (`01201f50`) becomes 0x4b (shader model setting below 2, signed) or
/// 0xeb when it is 0. When the `NiPointer` at +0x30 is null: a constant map of program type 2
/// into it with `"CoreColor"` and `"GlowColor"` (0x10000007, registers 0 and 1, 0x10 bytes at
/// +0x16c and +0x17c).
// Translation incomplete: 00bc1950 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn bolt_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let vertex_map = base + 0x34;
    if e.mem.u32(vertex_map) == 0 {
        make_constant_map(e, vertex_map, base + 0x18, 0);
        add_entry(
            e,
            vertex_map,
            &[
                NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                0x2000_0009,
                0,
                0,
                4,
                0,
                0,
                0,
                0,
                0,
            ],
        );
        add_entry(
            e,
            vertex_map,
            &[
                NAME_FVARS_010AF454,
                0x1000_0009,
                0,
                4,
                4,
                EMPTY_STRING_01011584,
                0x40,
                4,
                base + 0x128,
                0,
            ],
        );
        add_entry(
            e,
            vertex_map,
            &[
                NAME_EYE_POSITION_010AF5D8,
                0x1000_0007,
                0,
                8,
                1,
                EMPTY_STRING_01011584,
                0x10,
                4,
                base + 0x118,
                0,
            ],
        );
        if e.global::<u32>(SEGMENT_COUNT_01201F50) == 0 {
            let count = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
                0x4b
            } else {
                0xeb
            };
            e.set_global::<u32>(SEGMENT_COUNT_01201F50, count);
        }
        let count = e.global::<u32>(SEGMENT_COUNT_01201F50);
        let buffer = e.mem.u32(base + 0x70);
        add_entry(
            e,
            vertex_map,
            &[
                NAME_BOLT_DATA_010BA50C,
                0x1000_0009,
                0,
                0xe,
                count,
                EMPTY_STRING_01011584,
                count.wrapping_shl(4),
                4,
                buffer,
                0,
            ],
        );
    }
    let pixel_map = base + 0x30;
    if e.mem.u32(pixel_map) == 0 {
        make_constant_map(e, pixel_map, base + 0x18, 2);
        for (name, register, data) in [
            (NAME_CORE_COLOR_010BA500, 0u32, base + 0x16c),
            (NAME_GLOW_COLOR_010BA4F4, 1, base + 0x17c),
        ] {
            add_entry(
                e,
                pixel_map,
                &[
                    name,
                    0x1000_0007,
                    0,
                    register,
                    1,
                    EMPTY_STRING_01011584,
                    0x10,
                    4,
                    data,
                    0,
                ],
            );
        }
    }
}

/// The segment index block (`01201f4c`), the segments per bolt (`011ae61c`, a signed word)
/// and the bolts' started count (`01201f54`).
const SEGMENT_INDICES_01201F4C: u32 = 0x0120_1f4c;
const SEGMENTS_011AE61C: u32 = 0x011a_e61c;
const STARTED_01201F54: u32 = 0x0120_1f54;

/// The bolt count (`01201f50`), set on first use to 0x4b (shader model setting `011f91c0`
/// below 2, signed) or 0xeb.
fn bolt_count(e: &mut Engine) -> u32 {
    let mut count = e.global::<u32>(SEGMENT_COUNT_01201F50);
    if count == 0 {
        count = if (e.global::<u32>(SHADER_MODEL_011F91C0) as i32) < 2 {
            0x4b
        } else {
            0xeb
        };
        e.set_global(SEGMENT_COUNT_01201F50, count);
    }
    count
}

// Translated from 00bc0b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoltShader::GetSegmentIndices` (Xbox PDB), no arguments: the segment index block
/// (`01201f4c`), made on first use: `n` = bolt count (`01201f50`, set on first use) times the
/// segments per bolt (`011ae61c`) entries of four `float`s from the memory manager (size
/// checked for overflow), entry `i` = (i / m, i / m, (i + 1) / m, (i + 1) / m) with `i` read
/// unsigned and `m` the segments per bolt (x87, through `float`s).
pub fn bolt_shader_get_segment_indices(e: &mut Engine) -> u32 {
    let block = e.global::<u32>(SEGMENT_INDICES_01201F4C);
    if block != 0 {
        return block;
    }
    let count = bolt_count(e).wrapping_mul(e.global::<u32>(SEGMENTS_011AE61C));
    let size = count.wrapping_mul(4).checked_mul(4).unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.set_global(SEGMENT_INDICES_01201F4C, block);
    let mut index = 0u32;
    while index < count {
        let mut value = f64::from(index as i32);
        if (index as i32) < 0 {
            value = x87_add(value, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
        }
        let value_bits = x87_store32(value);
        index += 1;
        let segments_bits = x87_store32(f64::from(e.global::<u32>(SEGMENTS_011AE61C) as i32));
        let value = x87_load32(value_bits);
        let segments = x87_load32(segments_bits);
        let low_bits = x87_store32(x87_div(value, segments));
        let high_bits = x87_store32(x87_div(x87_add(value, 1.0), segments));
        let entry = block.wrapping_add((index - 1).wrapping_mul(0x10));
        let low = x87_quiet_bits(low_bits);
        let high = x87_quiet_bits(high_bits);
        for (offset, word) in [(0u32, low), (4, low), (8, high), (0xc, high)] {
            e.mem.set_u32(entry.wrapping_add(offset), word);
        }
    }
    e.global::<u32>(SEGMENT_INDICES_01201F4C)
}

// Translated from 00bc0c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `boltshader.cpp`), no arguments: frees the segment index
/// block (`01201f4c`, the memory manager's `Deallocate`) when set and clears the word.
pub fn fn_00bc0c50(e: &mut Engine) {
    let block = e.global::<u32>(SEGMENT_INDICES_01201F4C);
    if block != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    e.set_global(SEGMENT_INDICES_01201F4C, 0u32);
}

// Translated from 00bc0c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 73 of the vtable `010ba3b0`, `thiscall`, no
/// stack words: `this`'s slot +0x128 with the pass at +0x168; what it answers in EAX, EDX and
/// ST0 is this function's result.
pub fn fn_00bc0c70(e: &mut Engine, this: Ptr) -> Ret {
    let pass = e.mem.u32(this.addr() + 0x168);
    e.vcall(this.addr(), 0x128, &args![pass])
}

/// The uniform-form entry of [`fn_00bc0c70`]: `this`; the call's registers pass through.
fn fn_00bc0c70_entry(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00bc0c70(e, Ptr::new(a[0]))
}

/// What `00bc0f70` answers: the address `01201f5c` (no RTTI locator for the vtable; the word
/// is the class's `NiRTTI` by its place in the vtable, unconfirmed).
const RECORD_01201F5C: u32 = 0x0120_1f5c;

// Translated from 00bc0f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 2 of the vtable `010ba3b0`, `thiscall`
/// without reading `this`: answers the address `01201f5c`.
pub fn fn_00bc0f70(_e: &mut Engine, _this: Ptr) -> u32 {
    RECORD_01201F5C
}

/// `NiAVObject::GetProperty` (engine map, `00a59d30`; `thiscall`, the property type).
const GET_PROPERTY_00A59D30: u32 = 0x00a5_9d30;

/// A node child's bolt property (type 0xf among its type-3 properties), else null; a null
/// child gives null without asking.
fn child_bolt_property(e: &mut Engine, child: u32) -> u32 {
    if child == 0 {
        return 0;
    }
    let property = e.call(GET_PROPERTY_00A59D30, &args![child, 3u32]).u32();
    if property != 0 && e.mem.u32(property + 0x1c) == 0xf {
        property
    } else {
        0
    }
}

/// The shared `NiPointer`'s object released: count +4 decremented, slot +4 at 0.
fn release_shared(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object + 4])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00bc1470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoltShader::Stop` (Xbox PDB), `cdecl`, two words (a node, a bolt property): with both,
/// each child of the node (count u16 +0xa8, array +0xa0) whose bolt property is the given one
/// is removed (the node's slot +0xf0 with the index); then the started count (`01201f54`)
/// goes down and at 0 or below the shared geometry (`01201f58`) is released and cleared and
/// the count set to 0.
pub fn bolt_shader_stop(e: &mut Engine, node: u32, property: u32) {
    if node == 0 || property == 0 {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node + 0xa8)) {
        let child = e.mem.u32(e.mem.u32(node + 0xa0).wrapping_add(4 * index));
        let found = child_bolt_property(e, child);
        if found != 0 && found == property {
            e.vcall(node, 0xf0, &args![index]);
        }
        index += 1;
    }
    let started = e.global::<u32>(STARTED_01201F54).wrapping_sub(1);
    e.set_global(STARTED_01201F54, started);
    if started as i32 <= 0 {
        let shared = e.global::<u32>(SHARED_POINTER_01201F58);
        if shared != 0 {
            release_shared(e, shared);
            e.set_global(SHARED_POINTER_01201F58, 0u32);
        }
        e.set_global(STARTED_01201F54, 0u32);
    }
}

/// What `GetBoltGeometry` makes (engine map): `NiTriShapeData::NiTriShapeData` (`00a7b630`),
/// `NiAdditionalGeometryData::NiAdditionalGeometryData` (`00ba63e0`, unit `particleshader.cpp`),
/// `::SetDataBlockCount` (`00a72ca0`), `::SetDataBlock` (`00a72bb0`), `::SetDataStream`
/// (`00a726a0`), `00a67260` (unnamed), `NiTriShape::NiTriShape` (`00a74480`), the bolt
/// property constructor `00bc1d40` (unnamed), `NiAVObject::AttachProperty` (`00439410`), the
/// `float`s -1.0 (`01012054`) and the bound radius (`01018bfc`).
const TRI_SHAPE_DATA_00A7B630: u32 = 0x00a7_b630;
const ADDITIONAL_DATA_00BA63E0: u32 = 0x00ba_63e0;
const SET_DATA_BLOCK_COUNT_00A72CA0: u32 = 0x00a7_2ca0;
const SET_DATA_BLOCK_00A72BB0: u32 = 0x00a7_2bb0;
const SET_DATA_STREAM_00A726A0: u32 = 0x00a7_26a0;
const SET_ADDITIONAL_00A67260: u32 = 0x00a6_7260;
const TRI_SHAPE_00A74480: u32 = 0x00a7_4480;
const PROPERTY_CONSTRUCT_00BC1D40: u32 = 0x00bc_1d40;
const ATTACH_PROPERTY_00439410: u32 = 0x0043_9410;
const MINUS_ONE_01012054: u32 = 0x0101_2054;
const RADIUS_01018BFC: u32 = 0x0101_8bfc;

// Translated from 00bc15a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc15a0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the allocations) and the security-cookie bookkeeping are not translated.
/// `BoltShader::GetBoltGeometry` (Xbox PDB), no arguments: the shared geometry (`01201f58`)
/// when there is one; null with a bolt count (`01201f50`, set on first use) of 0. Otherwise,
/// for `n` = segments per bolt (`011ae61c`) times the bolt count: `4n` vertices (16-bit) at
/// (-1, -1, 0), (1, -1, 0), (1, 1, 0), (-1, 1, 0) per quad and `2n` triangles (v, v+1, v+2),
/// (v+3, v, v+2) from the memory manager (sizes checked for overflow); an `NiTriShapeData`
/// over them (0x58 bytes, no normals, colours or UVs), an `NiAdditionalGeometryData` (0x2c
/// bytes) with one data block (the segment indices, `GetSegmentIndices`, `4 * 4n` bytes) and
/// one stream given to the data, an `NiTriShape` (0xc4 bytes) with the bound radius
/// `[01018bfc]` and a new bolt property (0x190 bytes, `00bc1d40`). It replaces the shared
/// `NiPointer` (old one released, new one held). Null allocations are used as they are.
pub fn bolt_shader_get_bolt_geometry(e: &mut Engine) -> u32 {
    let shared = e.global::<u32>(SHARED_POINTER_01201F58);
    if shared != 0 {
        return shared;
    }
    let count = bolt_count(e);
    if count == 0 {
        return 0;
    }
    let quads = e.global::<u32>(SEGMENTS_011AE61C).wrapping_mul(count);
    let vertices = quads.wrapping_mul(4) & 0xffff;
    let triangles = quads.wrapping_mul(2) & 0xffff;
    let allocate =
        |e: &mut Engine, size: u32| e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    let positions = allocate(e, vertices.checked_mul(0xc).unwrap_or(u32::MAX));
    let index_bytes = triangles.wrapping_mul(3).checked_mul(2).unwrap_or(u32::MAX);
    let indices = allocate(e, index_bytes);
    if quads != 0 {
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE_01012054));
        let one = 0x3f80_0000u32;
        let quad = [
            minus_one, minus_one, 0, one, minus_one, 0, one, one, 0, minus_one, one, 0,
        ];
        let mut vertex = 0u32;
        for q in 0..quads {
            let at = positions.wrapping_add(q.wrapping_mul(0x30));
            for (index, word) in quad.iter().enumerate() {
                e.mem.set_u32(at.wrapping_add(4 * index as u32), *word);
            }
            let v = vertex as u16;
            let words = [
                v,
                v.wrapping_add(1),
                v.wrapping_add(2),
                v.wrapping_add(3),
                v,
                v.wrapping_add(2),
            ];
            let cursor = indices.wrapping_add(q.wrapping_mul(0xc));
            for (index, word) in words.iter().enumerate() {
                e.mem.set_u16(cursor.wrapping_add(2 * index as u32), *word);
            }
            vertex = vertex.wrapping_add(4);
        }
    }
    let block = e.call(NI_ALLOCATE, &args![0x58u32]).u32();
    let data = if block != 0 {
        e.call(
            TRI_SHAPE_DATA_00A7B630,
            &args![block, vertices, positions, 0u32, 0u32, 0u32, 0u32, 0u32, triangles, indices],
        )
        .u32()
    } else {
        0
    };
    let block = e.call(NI_ALLOCATE, &args![0x2cu32]).u32();
    let extra = if block != 0 {
        e.call(ADDITIONAL_DATA_00BA63E0, &args![block, vertices, 1u32])
            .u32()
    } else {
        0
    };
    e.call(SET_DATA_BLOCK_COUNT_00A72CA0, &args![extra, 1u32]);
    let bytes = vertices.wrapping_mul(4);
    let segments = bolt_shader_get_segment_indices(e);
    e.call(
        SET_DATA_BLOCK_00A72BB0,
        &args![extra, 0u32, segments, bytes, 1u32],
    );
    e.call(
        SET_DATA_STREAM_00A726A0,
        &args![extra, 0u32, 0u32, 0u32, 1u32, vertices, 4u32, 4u32],
    );
    e.call(SET_ADDITIONAL_00A67260, &args![data, extra]);
    let block = e.call(NI_ALLOCATE, &args![0xc4u32]).u32();
    let shape = if block != 0 {
        e.call(TRI_SHAPE_00A74480, &args![block, data]).u32()
    } else {
        0
    };
    let bound = e.mem.u32(shape.wrapping_add(0xb8));
    let center = [0x10u32, 0x14, 0x18].map(|offset| e.mem.u32(bound + offset));
    let radius = x87_quiet_bits(e.global::<u32>(RADIUS_01018BFC));
    e.mem.set_u32(bound + 0x1c, radius);
    for (offset, word) in [0x10u32, 0x14, 0x18].iter().zip(center) {
        e.mem.set_u32(bound + offset, word);
    }
    let block = e.call(NI_ALLOCATE, &args![0x190u32]).u32();
    let property = if block != 0 {
        e.call(PROPERTY_CONSTRUCT_00BC1D40, &args![block]).u32()
    } else {
        0
    };
    e.call(ATTACH_PROPERTY_00439410, &args![shape, property]);
    let old = e.global::<u32>(SHARED_POINTER_01201F58);
    if old != shape {
        if old != 0 {
            release_shared(e, old);
        }
        e.set_global(SHARED_POINTER_01201F58, shape);
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![shape.wrapping_add(4)]);
    }
    e.global::<u32>(SHARED_POINTER_01201F58)
}

/// `NiObject::Clone` (engine map, `00a5d680`; `thiscall`) and `BSShaderManager::PrepareObject`
/// (engine map, `00b57e30`; `cdecl`, three words).
const CLONE_00A5D680: u32 = 0x00a5_d680;
const PREPARE_OBJECT_00B57E30: u32 = 0x00b5_7e30;

// Translated from 00bc18d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoltShader::Start` (Xbox PDB), `cdecl`, one word (a node): clones the bolt geometry
/// (`GetBoltGeometry`, `NiObject::Clone`), attaches the clone (the node's slot +0xdc with 1),
/// prepares it (`PrepareObject(clone, 0, 0)`); with a bolt property (type 0xf) on the clone,
/// `GetBoltGeometry` runs again, the started count (`01201f54`) goes up and the property's slot
/// +0x9c gets the clone. Answers that property (null without a node or property).
pub fn bolt_shader_start(e: &mut Engine, node: u32) -> u32 {
    if node == 0 {
        return 0;
    }
    let geometry = bolt_shader_get_bolt_geometry(e);
    let clone = e.call(CLONE_00A5D680, &args![geometry]).u32();
    e.vcall(node, 0xdc, &args![clone, 1u32]);
    e.call(PREPARE_OBJECT_00B57E30, &args![clone, 0u32, 0u32]);
    let found = e.call(GET_PROPERTY_00A59D30, &args![clone, 3u32]).u32();
    let property = if found != 0 && e.mem.u32(found + 0x1c) == 0xf {
        found
    } else {
        0
    };
    if property == 0 {
        return 0;
    }
    bolt_shader_get_bolt_geometry(e);
    let started = e.global::<u32>(STARTED_01201F54).wrapping_add(1);
    e.set_global(STARTED_01201F54, started);
    e.vcall(property, 0x9c, &args![clone]);
    property
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_0b70, bolt_shader_get_segment_indices() -> u32),
        entry!(0x00bc_0c50, fn_00bc0c50()),
        (0x00bc_0c70, fn_00bc0c70_entry as AbiFn),
        entry!(0x00bc_0f70, fn_00bc0f70(Ptr) -> u32),
        entry!(0x00bc_1470, bolt_shader_stop(u32, u32)),
        entry!(0x00bc_15a0, bolt_shader_get_bolt_geometry() -> u32),
        entry!(0x00bc_18d0, bolt_shader_start(u32) -> u32),
        entry!(0x00bc_0c90, fn_00bc0c90(Ptr, Ptr) -> Ptr),
        entry!(0x00bc_10e0, fn_00bc10e0() -> Ptr),
        entry!(0x00bc_0f80, fn_00bc0f80(Ptr)),
        entry!(0x00bc_1210, fn_00bc1210(Ptr)),
        entry!(0x00bc_1540, fn_00bc1540(Ptr, u32) -> Ptr),
        entry!(0x00bc_1570, fn_00bc1570(Ptr) -> bool),
        entry!(0x00bc_1950, bolt_shader_create_constant_maps(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bc10e0_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BC0C90, &[]);
        let (shader, log) = run(&mut e, 0x00bc_10e0);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 5, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 3, 3, 2, 0]),
                (declaration_slot(0x8c), vec![d, 2, 5, 4, 3, 0]),
                (declaration_slot(0x8c), vec![d, 3, 7, 5, 1, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 0, 4, 6, 0x8000_0000, 0, 0xa, 1, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x194]),
                (SHADER_CONSTRUCT_00BC0C90, vec![SHADER, d]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x140), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(count: u32, model: i32) -> (Engine, u32, u32) {
        let mut e = rig();
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| {
            e.mem.alloc(a[1].min(0x1000)).into_ret()
        });
        e.register(CEIL, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            x.ceil().into_ret()
        });
        for page in [
            0x011a_9000u32,
            0x0120_1000,
            0x011f_9000,
            0x0109_9000,
            0x0102_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global::<f32>(TWO_TO_32_01099378, 4_294_967_296.0);
        e.set_global::<f64>(QUARTER_010290B0, 0.25);
        for index in 0..4u32 {
            e.set_global::<u32>(COLOR_011A9BE0 + 4 * index, 0x50 + index);
        }
        e.set_global::<u32>(SEGMENT_COUNT_01201F50, count);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x194);
        e.mem.write(this, &[0xcc; 0x194]);
        e.mem.set_u32(this + 0x28, 0);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00bc_0c90, &args![this, declaration]).u32(), this);
        (e, this, declaration)
    }

    #[test]
    fn fn_00bc0c90_builds_the_bolt_shader() {
        let (e, this, declaration) = construct(0, 3);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), BOLT_VTABLE_010BA3B0);
        assert_eq!(e.mem.u32(this + 0x68), 0xf);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x6c), 0x242);
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        for offset in [0x74u32, 0x118, 0x11c, 0x120, 0x124, 0x168, 0x18c, 0x190] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        for index in 0..4u32 {
            assert_eq!(e.mem.u32(this + 0x16c + 4 * index), 0x50 + index);
            assert_eq!(e.mem.u32(this + 0x17c + 4 * index), 0x50 + index);
        }
        // 0xeb segments: ceil(235 * 0.25) = 59, 4 * 59 elements of 16 bytes.
        assert_eq!(e.global::<u32>(SEGMENT_COUNT_01201F50), 0xeb);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xec0]]
        );
        let buffer = e.mem.u32(this + 0x70);
        assert_eq!(
            calls(&e, MEMSET),
            vec![
                vec![buffer, 0, 0xec0],
                vec![this + 0x128, 0, 0x40],
                vec![this + 0x78, 0, 0xa0]
            ]
        );
    }

    #[test]
    fn fn_00bc0c90_segment_count_follows_the_model_and_an_unsigned_count() {
        let (e, _, _) = construct(0, 1);
        assert_eq!(e.global::<u32>(SEGMENT_COUNT_01201F50), 0x4b);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x4c0]]
        );
        // A count with the sign bit set is loaded as unsigned: 2^31 / 4 segments, whose
        // byte count overflows the size check.
        let (e, _, _) = construct(0x8000_0000, 3);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, u32::MAX]]
        );
    }
}

#[cfg(test)]
mod b056_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;
    use crate::units::bsshader::particleshader::constant_map_rig::*;
    use crate::units::bsshader::particleshader::constant_maps::CONSTANT_MAP_CONSTRUCT;

    fn destructor_engine() -> Engine {
        let mut e = rig();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(BS_SHADER_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE_SIZED_00AA1460, |_, _| Ret::default());
        e.map(0x0120_1000, 0x1000);
        e
    }

    #[test]
    fn fn_00bc0f80_destroys_the_members_and_keeps_the_memory() {
        let mut e = destructor_engine();
        let shared = object(&mut e, 2);
        e.set_global::<u32>(SHARED_POINTER_01201F58, shared);
        let this = e.mem.alloc(0x194);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        let vertex = object(&mut e, 1);
        e.mem.set_u32(this + 0x70, 0x0e55_0000);
        e.mem.set_u32(this + 0x168, pass);
        e.mem.set_u32(this + 0x18c, vertex);
        e.call(0x00bc_0f80, &args![this]);
        assert_eq!(e.mem.u32(this), BOLT_VTABLE_010BA3B0);
        assert_eq!(e.global::<u32>(SHARED_POINTER_01201F58), 0);
        assert_eq!(e.mem.u32(shared + 4), 1);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x0e55_0000]]
        );
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![vertex]]);
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
        assert!(calls(&e, NI_FREE_SIZED_00AA1460).is_empty());
    }

    #[test]
    fn fn_00bc1210_sets_up_the_pass_it_is_given() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0x194);
        e.call(NEW_PASS_00E804D0, &args![this + 0x168]);
        let pass = e.mem.u32(this + 0x168);
        let count = e.mem.u32(pass + 0x64);
        let vertex = object(&mut e, 1);
        let pixel = object(&mut e, 1);
        e.mem.set_u32(this + 0x18c, vertex);
        e.mem.set_u32(this + 0x190, pixel);
        e.call(0x00bc_1210, &args![this]);
        assert_eq!(e.mem.u32(this + 0x168), pass);
        assert_eq!(calls(&e, SETUP_DEFAULT_STAGE)[0][1..], [0, 1, 2, 1]);
        assert_eq!(calls(&e, PASS_SET_STAGE)[0][..2], [pass, 0]);
        assert_eq!(e.mem.u32(pass + 0x5c), vertex);
        assert_eq!(e.mem.u32(pass + 0x44), pixel);
        assert_eq!(render_states(&e, pass), BOLT_RENDER_STATES.to_vec());
        // Held and dropped again.
        assert_eq!(e.mem.u32(pass + 0x64), count);
    }

    #[test]
    fn fn_00bc1540_destroys_the_bolt_shader() {
        let mut e = destructor_engine();
        let shared = object(&mut e, 2);
        e.set_global::<u32>(SHARED_POINTER_01201F58, shared);
        let this = e.mem.alloc(0x194);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        let vertex = object(&mut e, 1);
        let pixel = object(&mut e, 3);
        e.mem.set_u32(this + 0x70, 0x0e55_0000);
        e.mem.set_u32(this + 0x168, pass);
        e.mem.set_u32(this + 0x18c, vertex);
        e.mem.set_u32(this + 0x190, pixel);
        assert_eq!(e.call(0x00bc_1540, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BOLT_VTABLE_010BA3B0);
        assert_eq!(e.global::<u32>(SHARED_POINTER_01201F58), 0);
        assert_eq!(e.mem.u32(shared + 4), 1);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x0e55_0000]]
        );
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![vertex]]);
        assert_eq!(e.mem.u32(pixel + 4), 2);
        for offset in [0x70u32, 0x168, 0x18c, 0x190] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0x194]]);
        e.call(0x00bc_1540, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460).len(), 1);
    }

    #[test]
    fn fn_00bc1570_makes_and_sets_up_the_pass() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0x194);
        let vertex = object(&mut e, 1);
        let pixel = object(&mut e, 1);
        e.mem.set_u32(this + 0x18c, vertex);
        e.mem.set_u32(this + 0x190, pixel);
        assert!(e.call(0x00bc_1570, &args![this]).bool());
        let pass = e.mem.u32(this + 0x168);
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![this + 0x168]]);
        assert_eq!(calls(&e, SETUP_DEFAULT_STAGE)[0][1..], [0, 1, 2, 1]);
        assert_eq!(calls(&e, PASS_SET_STAGE)[0][..2], [pass, 0]);
        assert_eq!(e.mem.u32(pass + 0x5c), vertex);
        assert_eq!(e.mem.u32(pass + 0x44), pixel);
        assert_eq!(render_states(&e, pass), BOLT_RENDER_STATES.to_vec());
        assert_eq!(e.mem.u32(pass + 0x64), 1);
        // The stage's reference is dropped: the pass keeps the one `SetStage` took.
        let stage = calls(&e, PASS_SET_STAGE)[0][2];
        assert_eq!(e.mem.u32(stage + 0x60), 1);
        // A pass that is there and has a stage: no new pass and no stage.
        e.mem.set_u32(pass + 0x18, 1);
        e.call(0x00bc_1570, &args![this]);
        assert_eq!(calls(&e, NEW_PASS_00E804D0).len(), 1);
        assert_eq!(calls(&e, CREATE_NEW_STAGE).len(), 1);
    }

    #[test]
    fn bolt_create_constant_maps_adds_the_entries() {
        let mut e = map_rig();
        for page in [0x0120_1000u32, 0x011f_9000] {
            e.map(page, 0x1000);
        }
        e.set_global::<i32>(SHADER_MODEL_011F91C0, 3);
        let this = e.mem.alloc(0x194);
        e.mem.set_u32(this + 0x18, 0x0e50_0000);
        e.mem.set_u32(this + 0x70, 0x0e56_0000);
        e.call(0x00bc_1950, &args![this]);
        let vertex = e.mem.u32(this + 0x34);
        let pixel = e.mem.u32(this + 0x30);
        assert_eq!(map_made_with(&e, vertex), (0x0e50_0000, 0));
        assert_eq!(map_made_with(&e, pixel), (0x0e50_0000, 2));
        assert_eq!(e.global::<u32>(SEGMENT_COUNT_01201F50), 0xeb);
        let added = entries(&e, vertex);
        assert_eq!(
            added,
            vec![
                vec![
                    NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                    0x2000_0009,
                    0,
                    0,
                    4,
                    0,
                    0,
                    0,
                    0,
                    0
                ],
                vec![
                    NAME_FVARS_010AF454,
                    0x1000_0009,
                    0,
                    4,
                    4,
                    EMPTY_STRING_01011584,
                    0x40,
                    4,
                    this + 0x128,
                    0
                ],
                vec![
                    NAME_EYE_POSITION_010AF5D8,
                    0x1000_0007,
                    0,
                    8,
                    1,
                    EMPTY_STRING_01011584,
                    0x10,
                    4,
                    this + 0x118,
                    0
                ],
                vec![
                    NAME_BOLT_DATA_010BA50C,
                    0x1000_0009,
                    0,
                    0xe,
                    0xeb,
                    EMPTY_STRING_01011584,
                    0xeb0,
                    4,
                    0x0e56_0000,
                    0
                ],
            ]
        );
        let colours = entries(&e, pixel);
        assert_eq!(colours.len(), 2);
        assert_eq!(
            colours[0][..4],
            [NAME_CORE_COLOR_010BA500, 0x1000_0007, 0, 0]
        );
        assert_eq!(colours[0][8], this + 0x16c);
        assert_eq!(
            colours[1][..4],
            [NAME_GLOW_COLOR_010BA4F4, 0x1000_0007, 0, 1]
        );
        assert_eq!(colours[1][8], this + 0x17c);
        // Both maps there: nothing more; a model below 2 gives 0x4b segments.
        e.call(0x00bc_1950, &args![this]);
        assert_eq!(calls(&e, CONSTANT_MAP_CONSTRUCT).len(), 2);
        e.set_global::<u32>(SEGMENT_COUNT_01201F50, 0);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, 1);
        let this = e.mem.alloc(0x194);
        let held = object(&mut e, 1);
        e.mem.set_u32(this + 0x30, held);
        e.call(0x00bc_1950, &args![this]);
        assert_eq!(e.global::<u32>(SEGMENT_COUNT_01201F50), 0x4b);
        assert!(entries(&e, held).is_empty());
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
    const SLOTS: u32 = 0x0ee0_0000;

    fn object(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x170);
        for offset in (0..0x170u32).step_by(4) {
            e.mem.set_u32(vtable + offset, SLOTS + offset);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0120_1000u32,
            0x011f_9000,
            0x011a_e000,
            0x0101_2000,
            0x0101_8000,
            0x0109_9000,
        ] {
            e.map(page, 0x1000);
        }
        for offset in (0..0x170u32).step_by(4) {
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
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn get_segment_indices_builds_once() {
        let mut e = rig();
        e.set_global(SEGMENT_COUNT_01201F50, 1u32);
        e.set_global(SEGMENTS_011AE61C, 4u32);
        let block = e.call(0x00bc_0b70, &args![]).u32();
        assert_eq!(calls(&e, MEMORY_ALLOCATE)[0][1], 0x40);
        let entry = |e: &Engine, i: u32| -> Vec<f32> {
            (0..4).map(|k| e.mem.f32(block + 16 * i + 4 * k)).collect()
        };
        assert_eq!(entry(&e, 0), vec![0.0, 0.0, 0.25, 0.25]);
        assert_eq!(entry(&e, 3), vec![0.75, 0.75, 1.0, 1.0]);
        assert_eq!(e.call(0x00bc_0b70, &args![]).u32(), block);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
        e.call(0x00bc_0c50, &args![]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(e.global::<u32>(SEGMENT_INDICES_01201F4C), 0);
        e.call(0x00bc_0c50, &args![]);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), 1);
        // A zero count from the model setting (2 or more: 0xeb).
        e.set_global(SEGMENT_COUNT_01201F50, 0u32);
        e.set_global(SHADER_MODEL_011F91C0, 2u32);
        e.set_global(SEGMENTS_011AE61C, 1u32);
        e.call(0x00bc_0b70, &args![]);
        assert_eq!(e.global::<u32>(SEGMENT_COUNT_01201F50), 0xeb);
        assert_eq!(calls(&e, MEMORY_ALLOCATE)[1][1], 0xeb * 16);
    }

    #[test]
    fn fn_00bc0c70_and_00bc0f70() {
        let mut e = rig();
        e.register(SLOTS + 0x128, |_, _| 3u32.into_ret());
        let this = object(&mut e, 0x16c);
        e.mem.set_u32(this + 0x168, 0x1680);
        assert_eq!(e.call(0x00bc_0c70, &args![this]).u32(), 3);
        assert_eq!(calls(&e, SLOTS + 0x128), vec![vec![this, 0x1680]]);
        assert_eq!(e.call(0x00bc_0f70, &args![this]).u32(), 0x0120_1f5c);
    }

    #[test]
    fn stop_removes_matching_children_and_releases() {
        let mut e = rig();
        let wanted = e.mem.alloc(0x20);
        e.mem.set_u32(wanted + 0x1c, 0xf);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other + 0x1c, 0x11);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first + 4, other);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second + 4, wanted);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children, first);
        e.mem.set_u32(children + 4, second);
        e.register(GET_PROPERTY_00A59D30, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let node = object(&mut e, 0xac);
        e.mem.set_u32(node + 0xa0, children);
        e.mem.set_u16(node + 0xa8, 2);
        let shared = object(&mut e, 8);
        e.mem.set_u32(shared + 4, 1);
        e.set_global(SHARED_POINTER_01201F58, shared);
        e.set_global(STARTED_01201F54, 1u32);
        e.call(0x00bc_1470, &args![node, wanted]);
        assert_eq!(calls(&e, SLOTS + 0xf0), vec![vec![node, 1]]);
        assert_eq!(e.global::<u32>(SHARED_POINTER_01201F58), 0);
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![shared]]);
        assert_eq!(e.global::<u32>(STARTED_01201F54), 0);
        e.set_global(STARTED_01201F54, 4u32);
        e.call(0x00bc_1470, &args![node, 0u32]);
        assert_eq!(e.global::<u32>(STARTED_01201F54), 4);
    }

    fn geometry_rig() -> Engine {
        let mut e = rig();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        for target in [
            TRI_SHAPE_DATA_00A7B630,
            ADDITIONAL_DATA_00BA63E0,
            PROPERTY_CONSTRUCT_00BC1D40,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.register(TRI_SHAPE_00A74480, |e, a| {
            let bound = e.mem.alloc(0x20);
            e.mem.set_u32(bound + 0x14, 0x1414);
            e.mem.set_u32(a[0] + 0xb8, bound);
            a[0].into_ret()
        });
        for target in [
            SET_DATA_BLOCK_COUNT_00A72CA0,
            SET_DATA_BLOCK_00A72BB0,
            SET_DATA_STREAM_00A726A0,
            SET_ADDITIONAL_00A67260,
            ATTACH_PROPERTY_00439410,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.set_global(MINUS_ONE_01012054, -1.0f32);
        e.set_global(RADIUS_01018BFC, 1000.0f32);
        e.set_global(SEGMENT_COUNT_01201F50, 1u32);
        e.set_global(SEGMENTS_011AE61C, 2u32);
        e
    }

    #[test]
    fn get_bolt_geometry_builds_the_shared_shape() {
        let mut e = geometry_rig();
        let shape = e.call(0x00bc_15a0, &args![]).u32();
        assert_eq!(e.global::<u32>(SHARED_POINTER_01201F58), shape);
        assert_eq!(e.mem.u32(shape + 4), 1);
        let data = &calls(&e, TRI_SHAPE_DATA_00A7B630)[0];
        assert_eq!(data[1], 8);
        assert_eq!(data[3..8], [0, 0, 0, 0, 0]);
        assert_eq!(data[8], 4);
        let positions: Vec<f32> = (0..12).map(|i| e.mem.f32(data[2] + 4 * i)).collect();
        assert_eq!(
            positions,
            vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0]
        );
        let words: Vec<u16> = (0..12).map(|i| e.mem.u16(data[9] + 2 * i)).collect();
        assert_eq!(words, vec![0, 1, 2, 3, 0, 2, 4, 5, 6, 7, 4, 6]);
        let segments = e.global::<u32>(SEGMENT_INDICES_01201F4C);
        assert_eq!(
            calls(&e, SET_DATA_BLOCK_00A72BB0)[0][1..],
            [0, segments, 32, 1]
        );
        assert_eq!(calls(&e, ADDITIONAL_DATA_00BA63E0)[0][1..], [8, 1]);
        let bound = e.mem.u32(shape + 0xb8);
        assert_eq!(e.mem.f32(bound + 0x1c), 1000.0);
        assert_eq!(e.mem.u32(bound + 0x14), 0x1414);
        assert_eq!(calls(&e, NI_ALLOCATE).last().unwrap(), &vec![0x190]);
        assert_eq!(e.call(0x00bc_15a0, &args![]).u32(), shape);
        assert_eq!(calls(&e, TRI_SHAPE_00A74480).len(), 1);
    }

    #[test]
    fn start_attaches_a_clone_and_counts_it() {
        let mut e = geometry_rig();
        let clone = e.mem.alloc(0xc0);
        e.set_global(0x0120_1f00, clone);
        e.register(CLONE_00A5D680, |e, _| e.mem.u32(0x0120_1f00).into_ret());
        e.register(PREPARE_OBJECT_00B57E30, |_, _| Ret::default());
        let property = object(&mut e, 0x20);
        e.mem.set_u32(property + 0x1c, 0xf);
        e.set_global(0x0120_1f04, property);
        e.register(GET_PROPERTY_00A59D30, |e, _| {
            e.mem.u32(0x0120_1f04).into_ret()
        });
        let node = object(&mut e, 0xac);
        assert_eq!(e.call(0x00bc_18d0, &args![node]).u32(), property);
        assert_eq!(calls(&e, SLOTS + 0xdc), vec![vec![node, clone, 1]]);
        assert_eq!(calls(&e, PREPARE_OBJECT_00B57E30), vec![vec![clone, 0, 0]]);
        assert_eq!(calls(&e, SLOTS + 0x9c), vec![vec![property, clone]]);
        assert_eq!(e.global::<u32>(STARTED_01201F54), 1);
        e.mem.set_u32(property + 0x1c, 0x11);
        assert_eq!(e.call(0x00bc_18d0, &args![node]).u32(), 0);
        assert_eq!(e.global::<u32>(STARTED_01201F54), 1);
        assert_eq!(e.call(0x00bc_18d0, &args![0u32]).u32(), 0);
    }
}
