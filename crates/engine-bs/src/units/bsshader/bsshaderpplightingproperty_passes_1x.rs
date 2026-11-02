//! `BSShader/bsshaderpplightingproperty_passes_1x.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The pass builders here share one shape: from their flag words they pick a render-pass
//! enum and, when the pass-count byte is 1, add that pass to the property's render-pass list
//! (`pRenderPassList`, +0x3c) with `BSShaderProperty::RenderPassArray::Add` (`00ba9ee0`);
//! otherwise they count one pass in the caller's 16-bit counter. Either way the caller's
//! first-pass byte is cleared afterwards.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::bsshaderproperty::BSShaderProperty;

/// `BSShaderProperty::RenderPassArray::Add` (`thiscall`, eight words: geometry, pass enum,
/// first-pass flag, light count, four light words).
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// The formatter `00406d00` (`cdecl`: buffer, size, format, arguments).
const FORMAT: u32 = 0x0040_6d00;
/// `"Parent:%s,Child:%s"`.
const PARENT_CHILD_FORMAT: u32 = 0x010b_c5f4;
/// `"Parent:%s"`.
const PARENT_FORMAT: u32 = 0x010b_c5e8;
/// The one-string format at `01019f08` the code uses for the geometry's own name.
const NAME_FORMAT_01019F08: u32 = 0x0101_9f08;
/// The format without arguments at `010250cc` the code uses when there is no name.
const NO_NAME_FORMAT_010250CC: u32 = 0x0102_50cc;
/// `"SHADER ERROR (%s)(%08X) : no shader to handle TEXTURE_SFgVc ( texture skinned facegen
/// vertexcolors )"`.
const SKINNED_FACEGEN_ERROR_FORMAT: u32 = 0x010b_c580;

/// The `Add` call every builder makes: the pass with the first-pass byte, no lights.
fn add_pass(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    pass: u32,
    first_pass: Ptr<u8>,
) {
    let first = u32::from(e.mem.u8(first_pass.addr()));
    let list = e.get(this, BSShaderProperty::pRenderPassList);
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, first, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
}

/// Adds `pass` when `pass_count` is 1, else counts one more pass in `counter`.
fn add_or_count(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    pass: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
) {
    if pass_count == 1 {
        add_pass(e, this, geometry, pass, first_pass);
    } else {
        let count = e.mem.u16(counter.addr());
        e.mem.set_u16(counter.addr(), count.wrapping_add(1));
    }
}

// Translated from 00bd9e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x14`: geometry, pass counter, pass count,
/// first-pass byte, flag): pass `0x1e6` when the flag is set, `0x1e5` otherwise; added when
/// the pass count is 1, counted otherwise. Clears the first-pass byte and returns 0.
pub fn fn_00bd9e60(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
) -> u32 {
    let pass = if flag { 0x1e6 } else { 0x1e5 };
    add_or_count(e, this, geometry, pass, counter, pass_count, first_pass);
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

// Translated from 00bd9f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x18`: geometry, pass counter, pass count,
/// first-pass byte, two flags): pass 3 when the first flag is set, otherwise 2 when the
/// second flag is set and 1 when not; added when the pass count is 1, counted otherwise.
/// Clears the first-pass byte and returns 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bd9f00(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
    second_flag: bool,
) -> u32 {
    let pass = if flag { 3 } else { u32::from(second_flag) + 1 };
    add_or_count(e, this, geometry, pass, counter, pass_count, first_pass);
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

// Translated from 00bdac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddTexturePass_1x` (engine map; the body matches;
/// `thiscall`, `RET 0x2c`: geometry, an unread word, pass counter, pass count, first-pass
/// byte, then the flags at stack words 6 and 8 to 11, word 7 unread). The pass:
///
/// - flag 6 clear: `0x151` when flag 10 is set; else with flag 9 set `0x14c` (flag 11 set)
///   or `0x14e`; else `0x14b` (flag 8 set) or `0x14a`;
/// - flag 6 set, flag 8 clear: `0x14d` when flag 9 is clear, else `0x152` (flag 11 set) or
///   `0x14f`;
/// - flags 6 and 8 set: `0x150` when flag 9 is clear. With flag 9 set there is no pass: the
///   code formats the geometry's description (`"Parent:%s,Child:%s"` with its parent's and
///   its own name, `"Parent:%s"`, the one-name format `01019f08`, or the format `010250cc`
///   without a name or geometry; names are the `NiFixedString` handles at +0x08 of the
///   geometry and of its parent at +0x18) into a 128-byte buffer and then the
///   `TEXTURE_SFgVc` shader error message into a 255-byte one, and neither adds nor counts.
///
/// A chosen pass is added when the pass count is 1 and counted otherwise. The first-pass
/// byte is cleared on every path; the result is 0.
///
/// The /GS stack cookie check (`00ec408c`) is not translated.
// Translation incomplete: 00bdac00 - the /GS stack cookie check (00ec408c) is not translated
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_texture_pass_1x(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    _unused_2: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag6: bool,
    _unused_7: u32,
    flag8: bool,
    flag9: bool,
    flag10: bool,
    flag11: bool,
) -> u32 {
    let pass = if !flag6 {
        if flag10 {
            Some(0x151)
        } else if flag9 {
            Some(if flag11 { 0x14c } else { 0x14e })
        } else if flag8 {
            Some(0x14b)
        } else {
            Some(0x14a)
        }
    } else if !flag8 {
        if !flag9 {
            Some(0x14d)
        } else if flag11 {
            Some(0x152)
        } else {
            Some(0x14f)
        }
    } else if !flag9 {
        Some(0x150)
    } else {
        None
    };
    match pass {
        Some(pass) => add_or_count(e, this, geometry, pass, counter, pass_count, first_pass),
        None => e.with_stack(0x180, |e, buffers| {
            let description = buffers.addr();
            let message = description + 0x80;
            if geometry == 0 {
                e.call(
                    FORMAT,
                    &args![description, 0x80u32, NO_NAME_FORMAT_010250CC],
                );
            } else {
                // NiAVObject::m_pkParent (Xbox PDB) +0x18, m_kName +0x08.
                let parent = e.mem.u32(geometry + 0x18);
                let parent_name = if parent != 0 {
                    e.mem.u32(parent + 8)
                } else {
                    0
                };
                let name = e.mem.u32(geometry + 8);
                if parent_name != 0 {
                    if name != 0 {
                        e.call(
                            FORMAT,
                            &args![description, 0x80u32, PARENT_CHILD_FORMAT, parent_name, name],
                        );
                    } else {
                        e.call(
                            FORMAT,
                            &args![description, 0x80u32, PARENT_FORMAT, parent_name],
                        );
                    }
                } else if name != 0 {
                    e.call(
                        FORMAT,
                        &args![description, 0x80u32, NAME_FORMAT_01019F08, name],
                    );
                } else {
                    e.call(
                        FORMAT,
                        &args![description, 0x80u32, NO_NAME_FORMAT_010250CC],
                    );
                }
            }
            e.call(
                FORMAT,
                &args![
                    message,
                    0xffu32,
                    SKINNED_FACEGEN_ERROR_FORMAT,
                    description,
                    geometry
                ],
            );
        }),
    }
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

/// `BSShaderProperty::RenderPass::RenderPass` (`00ba8ec0`, `cdecl`: the pass, geometry, pass
/// enum, first-pass flag, light count, lights), the memory manager's allocation and the
/// colour compare `00439090` (`thiscall` on a colour, the address of another: answers a
/// byte) with the colour at `011f4998`.
const RENDER_PASS_CONSTRUCT: u32 = 0x00ba_8ec0;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const COLOR_COMPARE_00439090: u32 = 0x0043_9090;
const COLOR_011F4998: u32 = 0x011f_4998;

/// The `Add` call with the first-pass flag 1 and the given lights (count, then up to four).
fn add_pass_lit(e: &mut Engine, list: u32, geometry: u32, pass: u32, first: u32, lights: [u32; 5]) {
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, first, lights[0], lights[1], lights[2], lights[3], lights[4]],
    );
}

// Translated from 00bd9770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x20`: geometry, two lights, pass counter, pass
/// count, first-pass byte, two flags): pass `0x7d` when the first flag is set, else `0x7e`
/// with the second flag and `0x7c` without; added (first-pass flag 1, the two lights) when
/// the pass count is 1, counted otherwise. Clears the first-pass byte and returns 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bd9770(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    first_light: u32,
    second_light: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
    second_flag: bool,
) -> u32 {
    let pass = if flag {
        0x7d
    } else if second_flag {
        0x7e
    } else {
        0x7c
    };
    if pass_count == 1 {
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        add_pass_lit(
            e,
            list,
            geometry,
            pass,
            1,
            [2, first_light, second_light, 0, 0],
        );
    } else {
        let count = e.mem.u16(counter.addr());
        e.mem.set_u16(counter.addr(), count.wrapping_add(1));
    }
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

// Translated from 00bd99c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x2c`: geometry, a light, pass counter, pass
/// count, first-pass byte, two flags, four unread words): pass `0x156` (second flag clear) or
/// `0x157` when the first flag is set, else `0x154` or `0x155`; added with the first-pass
/// byte and the one light when the pass count is 1, counted otherwise. Clears the first-pass
/// byte and returns 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bd99c0(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    light: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
    second_flag: bool,
    _unused_8: u32,
    _unused_9: u32,
    _unused_10: u32,
    _unused_11: u32,
) -> u32 {
    let pass = match (flag, second_flag) {
        (false, false) => 0x154,
        (false, true) => 0x155,
        (true, false) => 0x156,
        (true, true) => 0x157,
    };
    if pass_count == 1 {
        let first = u32::from(e.mem.u8(first_pass.addr()));
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        add_pass_lit(e, list, geometry, pass, first, [1, light, 0, 0, 0]);
    } else {
        let count = e.mem.u16(counter.addr());
        e.mem.set_u16(counter.addr(), count.wrapping_add(1));
    }
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

// Translated from 00bd9ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x18`: geometry, pass counter, pass count,
/// first-pass byte, two flags): pass `0x246` (second flag clear) or `0x247` when the first
/// flag is set, else `0x244` or `0x245`; added when the pass count is 1, counted otherwise.
/// Clears the first-pass byte and returns 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bd9ac0(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
    second_flag: bool,
) -> u32 {
    let pass = match (flag, second_flag) {
        (false, false) => 0x244,
        (false, true) => 0x245,
        (true, false) => 0x246,
        (true, true) => 0x247,
    };
    add_or_count(e, this, geometry, pass, counter, pass_count, first_pass);
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

// Translated from 00bd9d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddTextureEffectPass_1x` (engine map; the body matches;
/// `thiscall`, `RET 0x14`: geometry, pass counter, pass count, first-pass byte, flag): pass
/// `0x24f` with the flag, `0x24e` without; added when the pass count is 1, counted otherwise.
/// Clears the first-pass byte and returns 0.
pub fn bs_shader_pp_lighting_property_add_texture_effect_pass_1x(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
) -> u32 {
    let pass = if flag { 0x24f } else { 0x24e };
    add_or_count(e, this, geometry, pass, counter, pass_count, first_pass);
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

// Translated from 00bd9da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x18`: geometry, pass counter, pass count,
/// first-pass byte, two flags): pass `0x1e4` when the second flag is set, else `0x1e3` with
/// the first flag and `0x1e2` without; added when the pass count is 1, counted otherwise.
/// Clears the first-pass byte and returns 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bd9da0(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
    second_flag: bool,
) -> u32 {
    let pass = if second_flag {
        0x1e4
    } else if flag {
        0x1e3
    } else {
        0x1e2
    };
    add_or_count(e, this, geometry, pass, counter, pass_count, first_pass);
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

// Translated from 00bd9f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x14`: geometry, pass counter, pass count,
/// first-pass byte, flag): pass 5 with the flag, 4 without; added when the pass count is 1,
/// counted otherwise. Clears the first-pass byte and returns 0.
pub fn fn_00bd9f90(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
) -> u32 {
    let pass = if flag { 5 } else { 4 };
    add_or_count(e, this, geometry, pass, counter, pass_count, first_pass);
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

// Translated from 00bda030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`: geometry, an unread word, a byte, a
/// flag): with the byte set and an `SIBlock` pass list (`+0x4c`), adds pass `0x15` (flag set)
/// or `0x14` to it (not a first pass, no lights). Returns 0.
pub fn fn_00bda030(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    _unused_2: u32,
    wanted: u8,
    flag: bool,
) -> u32 {
    if wanted == 0 {
        return 0;
    }
    let list = e.get(this, BSShaderProperty::pSIBlockRenderPassList);
    if list == 0 {
        return 0;
    }
    let pass = 0x14 + u32::from(flag);
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    0
}

// Translated from 00bda060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: geometry, two flags): adds to the
/// constant-alpha pass list (`+0x44`) pass 8 when the first flag is set, else 7 with the
/// second flag and 6 without (first pass, no lights). Answers what `Add` leaves in `EAX`.
pub fn fn_00bda060(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    flag: u8,
    second_flag: u8,
) -> u32 {
    let list = e.get(this, BSShaderProperty::pConstAlphaRenderPassList);
    let pass = if flag != 0 {
        8
    } else {
        6 + u32::from(second_flag != 0)
    };
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00bdb380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bdb380 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0x14`: geometry, pass counter, pass count, a
/// flag, an unread word): the emittance pass. Null unless the geometry's object at `+0xa4`
/// exists and its colour (the three words its `+0x34` points to, or its own `+0x28` when that
/// is null, copied into a local) compares as different from the colour at `011f4998`
/// (`00439090`). Then the last pass of the render-pass list gets its last-pass byte (`+8`)
/// set when there is one; with a pass count other than 1 the counter goes up by one and the
/// answer is null; otherwise a new pass (0x10 bytes, `00ba8ec0` with the geometry, `0x2d7`
/// with the flag or `0x2d6` without, no first pass, no lights; null when the allocation
/// fails) is the answer.
pub fn fn_00bdb380(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    flag: u8,
    _unused_5: u32,
) -> u32 {
    let object = e.mem.u32(geometry.wrapping_add(0xa4));
    if object == 0 {
        return 0;
    }
    let differs = e.with_stack(0xc, |e, color| {
        let mut source = e.mem.u32(object.wrapping_add(0x34));
        if source == 0 {
            source = object.wrapping_add(0x28);
        }
        for k in 0..3u32 {
            let v = e.mem.u32(source.wrapping_add(4 * k));
            e.mem.set_u32(color.addr() + 4 * k, v);
        }
        e.call(COLOR_COMPARE_00439090, &args![color, COLOR_011F4998])
            .u8()
    });
    if differs == 0 {
        return 0;
    }
    let list = e.get(this, BSShaderProperty::pRenderPassList);
    let used = e.mem.u32(list.wrapping_add(0x10));
    let base = e.mem.u32(list.wrapping_add(4));
    let last = e
        .mem
        .u32(base.wrapping_add(used.wrapping_mul(4)).wrapping_sub(4));
    if last != 0 {
        e.mem.set_u8(last.wrapping_add(8), 1);
    }
    if pass_count != 1 {
        let count = e.mem.u16(counter.addr());
        e.mem.set_u16(counter.addr(), count.wrapping_add(1));
        return 0;
    }
    let pass = if flag != 0 { 0x2d7 } else { 0x2d6 };
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
        .u32();
    if block == 0 {
        return 0;
    }
    e.call(
        RENDER_PASS_CONSTRUCT,
        &args![block, geometry, pass, 0u32, 0u32, 0u32],
    )
    .u32()
}

/// The landscape globals: the shader level (`011f91c0`), the most layers drawn (`011ad828`)
/// and the byte that selects the simple landscape passes (`011f91a7`).
const SHADER_LEVEL_011F91C0: u32 = 0x011f_91c0;
const LANDSCAPE_LAYERS_011AD828: u32 = 0x011a_d828;

/// Whether a light's colour (its `+0xf8` object's `+0xd4` to `+0xdc`) differs from the
/// colour at `011f4998`, compared one channel at a time (`FUCOMPP`; NaN counts as different).
fn light_is_lit(e: &Engine, light: u32) -> bool {
    let color = e.mem.u32(light.wrapping_add(0xf8)).wrapping_add(0xd4);
    (0..3u32).any(|k| {
        let channel = x87_m32(e.mem.u32(color.wrapping_add(4 * k)));
        let black = x87_m32(e.mem.u32(COLOR_011F4998 + 4 * k));
        black != channel
    })
}

/// The landscape builders' `Add`: then the new last pass of the render-pass list gets the
/// layer number plus one in its `ucExtraParam` (`+0xb`).
#[allow(clippy::too_many_arguments)]
fn add_layer_pass(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    pass: u32,
    first: u32,
    lights: [u32; 5],
    layer: u8,
) {
    let list = e.get(this, BSShaderProperty::pRenderPassList);
    add_pass_lit(e, list, geometry, pass, first, lights);
    let list = e.get(this, BSShaderProperty::pRenderPassList);
    let used = e.mem.u32(list + 0x10);
    let last = e.mem.u32(
        e.mem
            .u32(list + 4)
            .wrapping_add(used.wrapping_mul(4))
            .wrapping_sub(4),
    );
    e.mem.set_u8(last.wrapping_add(0xb), layer.wrapping_add(1));
}

// Translated from 00bdaf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddLandscapePasses_1x` (engine map; the body matches;
/// `thiscall`, `RET 0x18`: geometry, a light (the sun), pass counter, pass count, first-pass
/// byte, a flag). The layers are the property's texture count (`+0xc8`) or the setting at
/// `011ad828` when that is lower; the textures are those of the geometry's shader property
/// (`GetProperty(3)`). Each pass is added when the pass count is 1 (with the first-pass byte)
/// and counted otherwise; layer passes get their layer number plus one in `ucExtraParam`.
///
/// - With the byte at `011f91a7` set: pass `0x93`, the first-pass byte cleared, then pass
///   `0x1f2` for each layer that has a second texture (the base array's next slot).
/// - Otherwise: pass `0x94` (flag set) or `0x93` with the light; the first-pass byte cleared;
///   pass `0x1f1` (not a first pass) when the glow array (`+0xb4`) has a first texture; at
///   shader level 1 or below, pass `0x1ef` with each active non-shadow light whose colour is
///   not black (`00b70600`/`00b70700`, the iterator in the first-pass argument's stack word);
///   then for each layer with a texture pass `0x1f3` (flag set) or `0x1f2` with the light;
///   then for each layer with a texture, pass `0x1f4` when its glow texture exists and, at
///   shader level 1 or below, pass `0x1f5` with each non-black light (the iterator in the
///   flag's stack word).
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_landscape_passes_1x(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    light: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    flag: bool,
) {
    let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
    let low_level = e.mem.i32(SHADER_LEVEL_011F91C0) <= 1;
    let setting = e.mem.u32(LANDSCAPE_LAYERS_011AD828);
    let own = u32::from(e.mem.u16(this.addr() + 0xc8));
    let layers = if own < setting {
        own as u16
    } else {
        setting as u16
    };
    let count_one = |e: &mut Engine| {
        let n = e.mem.u16(counter.addr());
        e.mem.set_u16(counter.addr(), n.wrapping_add(1));
    };
    let has_texture = |e: &Engine, offset: u32, layer: u8| {
        let array = e.mem.u32(property.wrapping_add(offset));
        e.mem.u32(array.wrapping_add(u32::from(layer) * 4 + 4)) != 0
    };
    if e.mem.u8(SWITCH_011F91A7) != 0 {
        if pass_count == 1 {
            let first = u32::from(e.mem.u8(first_pass.addr()));
            let list = e.get(this, BSShaderProperty::pRenderPassList);
            add_pass_lit(e, list, geometry, 0x93, first, [0; 5]);
        } else {
            count_one(e);
        }
        e.mem.set_u8(first_pass.addr(), 0);
        let mut layer = 0u8;
        while u16::from(layer) < layers {
            if has_texture(e, 0xac, layer) {
                if pass_count == 1 {
                    let first = u32::from(e.mem.u8(first_pass.addr()));
                    add_layer_pass(e, this, geometry, 0x1f2, first, [0; 5], layer);
                } else {
                    count_one(e);
                }
            }
            layer = layer.wrapping_add(1);
        }
        return;
    }
    if pass_count == 1 {
        let first = u32::from(e.mem.u8(first_pass.addr()));
        let pass = if flag { 0x94 } else { 0x93 };
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        add_pass_lit(e, list, geometry, pass, first, [1, light, 0, 0, 0]);
    } else {
        count_one(e);
    }
    e.mem.set_u8(first_pass.addr(), 0);
    let glow = e.mem.u32(property.wrapping_add(0xb4));
    if e.mem.u32(glow) != 0 {
        if pass_count == 1 {
            let list = e.get(this, BSShaderProperty::pRenderPassList);
            add_pass_lit(e, list, geometry, 0x1f1, 0, [0; 5]);
        } else {
            count_one(e);
        }
    }
    e.with_stack(8, |e, iterators| {
        let first_iterator = iterators.addr();
        let layer_iterator = first_iterator + 4;
        if low_level {
            let mut lit = e.call(FIRST_LIGHT, &args![this, first_iterator]).u32();
            while lit != 0 {
                if light_is_lit(e, lit) {
                    if pass_count == 1 {
                        let first = u32::from(e.mem.u8(first_pass.addr()));
                        let list = e.get(this, BSShaderProperty::pRenderPassList);
                        add_pass_lit(e, list, geometry, 0x1ef, first, [1, lit, 0, 0, 0]);
                    } else {
                        count_one(e);
                    }
                }
                lit = e.call(NEXT_LIGHT, &args![this, first_iterator]).u32();
            }
        }
        let mut layer = 0u8;
        while u16::from(layer) < layers {
            if has_texture(e, 0xac, layer) {
                if pass_count == 1 {
                    let first = u32::from(e.mem.u8(first_pass.addr()));
                    let pass = if flag { 0x1f3 } else { 0x1f2 };
                    add_layer_pass(e, this, geometry, pass, first, [1, light, 0, 0, 0], layer);
                } else {
                    count_one(e);
                }
            }
            layer = layer.wrapping_add(1);
        }
        let mut layer = 0u8;
        while u16::from(layer) < layers {
            if has_texture(e, 0xac, layer) {
                if has_texture(e, 0xb4, layer) {
                    if pass_count == 1 {
                        let first = u32::from(e.mem.u8(first_pass.addr()));
                        add_layer_pass(e, this, geometry, 0x1f4, first, [0; 5], layer);
                    } else {
                        count_one(e);
                    }
                }
                if low_level {
                    let mut lit = e.call(FIRST_LIGHT, &args![this, layer_iterator]).u32();
                    while lit != 0 {
                        if light_is_lit(e, lit) {
                            if pass_count == 1 {
                                let first = u32::from(e.mem.u8(first_pass.addr()));
                                add_layer_pass(
                                    e,
                                    this,
                                    geometry,
                                    0x1f5,
                                    first,
                                    [1, lit, 0, 0, 0],
                                    layer,
                                );
                            } else {
                                count_one(e);
                            }
                        }
                        lit = e.call(NEXT_LIGHT, &args![this, layer_iterator]).u32();
                    }
                }
            }
            layer = layer.wrapping_add(1);
        }
    });
}

// Translated from 00bda0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x30`: geometry, a light, pass counter, pass
/// count, first-pass byte and seven flags): the unlit 1.x pass. The seven flags choose one of
/// 44 passes (`0x64` to `0x92`) or none (the decision tree below, read from the code's
/// compare-and-jump chain in Ghidra); a pass is added with the light (first pass, one light)
/// when the pass count is 1 and counted otherwise. The first-pass byte is cleared on every path
/// and the answer is 0.
// The exe's decision tree has neighbouring arms with the same leaf (no pass); kept as it is.
#[allow(clippy::if_same_then_else)]
#[allow(clippy::too_many_arguments)]
pub fn fn_00bda0a0(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    light: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    f6: bool,
    f7: bool,
    f8: bool,
    f9: bool,
    f10: bool,
    f11: bool,
    f12: bool,
) -> u32 {
    let pass: Option<u32> = {
        if f6 {
            if f7 {
                if f8 {
                    if f9 {
                        if f10 {
                            if f11 {
                                None
                            } else {
                                Some(0x92)
                            }
                        } else if f11 {
                            None
                        } else {
                            Some(0x79)
                        }
                    } else if f10 {
                        if f11 {
                            Some(0x91)
                        } else {
                            Some(0x90)
                        }
                    } else if f11 {
                        Some(0x77)
                    } else {
                        Some(0x76)
                    }
                } else if f9 {
                    if f10 {
                        None
                    } else if f11 {
                        None
                    } else {
                        Some(0x78)
                    }
                } else if f10 {
                    if f11 {
                        Some(0x8e)
                    } else {
                        Some(0x8d)
                    }
                } else if f11 {
                    Some(0x75)
                } else {
                    Some(0x74)
                }
            } else if f8 {
                if f9 {
                    None
                } else if f10 {
                    if f11 {
                        Some(0x8c)
                    } else {
                        Some(0x8b)
                    }
                } else if f11 {
                    Some(0x72)
                } else {
                    Some(0x71)
                }
            } else if f9 {
                if f10 {
                    if f11 {
                        None
                    } else {
                        Some(0x8f)
                    }
                } else if f11 {
                    None
                } else {
                    Some(0x73)
                }
            } else if f10 {
                if f11 {
                    Some(0x8a)
                } else {
                    Some(0x89)
                }
            } else if f11 {
                Some(0x70)
            } else {
                Some(0x6f)
            }
        } else if f7 {
            if f8 {
                if f9 {
                    if f10 {
                        if f11 {
                            None
                        } else {
                            Some(0x88)
                        }
                    } else if f11 {
                        None
                    } else {
                        Some(0x6e)
                    }
                } else if f10 {
                    if f11 {
                        Some(0x87)
                    } else {
                        Some(0x86)
                    }
                } else if f11 {
                    Some(0x6c)
                } else {
                    Some(0x6b)
                }
            } else if f9 {
                if f10 {
                    None
                } else if f11 {
                    None
                } else {
                    Some(0x6d)
                }
            } else if f10 {
                if f11 {
                    Some(0x84)
                } else {
                    Some(0x83)
                }
            } else if f11 {
                Some(0x6a)
            } else {
                Some(0x69)
            }
        } else if f8 {
            if f9 {
                None
            } else if f10 {
                if f11 {
                    Some(0x82)
                } else {
                    Some(0x81)
                }
            } else if f11 {
                Some(0x67)
            } else {
                Some(0x66)
            }
        } else if f9 {
            if f10 {
                if f11 {
                    None
                } else {
                    Some(0x85)
                }
            } else if f11 {
                None
            } else {
                Some(0x68)
            }
        } else if f10 {
            if f11 {
                if f12 {
                    Some(0x7b)
                } else {
                    Some(0x80)
                }
            } else {
                Some(0x7f)
            }
        } else if f11 {
            if f12 {
                Some(0x7a)
            } else {
                Some(0x65)
            }
        } else {
            Some(0x64)
        }
    };
    if let Some(pass) = pass {
        if pass_count == 1 {
            let list = e.get(this, BSShaderProperty::pRenderPassList);
            add_pass_lit(e, list, geometry, pass, 1, [1, light, 0, 0, 0]);
        } else {
            let count = e.mem.u16(counter.addr());
            e.mem.set_u16(counter.addr(), count.wrapping_add(1));
        }
    }
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

/// The calls `GetRenderPasses_1x` makes outside this unit: `NiObject`'s kind test (`00653290`,
/// `thiscall`: the run-time type record) with the three records it asks about (`01200788`,
/// `012024c8`, `012007a0`), `NiAVObject::GetProperty` (`00a59d30`), the accumulator
/// (`00b4f5c0`), the light list walk of `BSShaderLightingProperty` (first and next active
/// non-shadow light, `00b70600` and `00b70700`, `thiscall` with the address of an iterator
/// word; their count `00b707d0`), and three pass builders nobody has translated yet
/// (`00bd9540`, `00bd9840`, `00bd9bc0`, `thiscall`).
const KIND_TEST_00653290: u32 = 0x0065_3290;
const RTTI_01200788: u32 = 0x0120_0788;
const RTTI_012024C8: u32 = 0x0120_24c8;
const RTTI_012007A0: u32 = 0x0120_07a0;
const GET_PROPERTY: u32 = 0x00a5_9d30;
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const FIRST_LIGHT: u32 = 0x00b7_0600;
const NEXT_LIGHT: u32 = 0x00b7_0700;
const ACTIVE_LIGHT_COUNT: u32 = 0x00b7_07d0;
const PASSES_00BD9540: u32 = 0x00bd_9540;
const PASSES_00BD9840: u32 = 0x00bd_9840;
const PASSES_00BD9BC0: u32 = 0x00bd_9bc0;
/// The globals `GetRenderPasses_1x` reads: a byte (`011f91a7`) and one (`011f9422`), the
/// flag word it compares its second word with (`010a91d4`) and the three fade distances
/// (`011f9468`, `011f9460`, `011f9458`, `float`s).
const SWITCH_011F91A7: u32 = 0x011f_91a7;
const SWITCH_011F9422: u32 = 0x011f_9422;
const PASS_FLAGS_010A91D4: u32 = 0x010a_91d4;
const FADE_011F9468: u32 = 0x011f_9468;
const FADE_011F9460: u32 = 0x011f_9460;
const FADE_011F9458: u32 = 0x011f_9458;

/// Whether the colour at `light`'s `+0xf8` object plus `offset` compares as different from
/// the colour at `011f4998` (`00439090`).
fn light_color_differs(e: &mut Engine, light: u32, offset: u32) -> bool {
    let color = e.mem.u32(light.wrapping_add(0xf8)).wrapping_add(offset);
    e.call(COLOR_COMPARE_00439090, &args![color, COLOR_011F4998])
        .u8()
        != 0
}

/// The fade test of `GetRenderPasses_1x`: a distance `c` of 0.0 keeps the feature (the
/// `FUCOM` with 0.0 is equal), otherwise the camera distance `d` must be below it (`FCOMPP`;
/// an unordered pair counts as below).
fn within_fade(c: f64, d: f64) -> bool {
    c == 0.0 || !(d >= c)
}

// Translated from 00bdb4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::GetRenderPasses_1x` (engine map; the body matches;
/// `thiscall`, `RET 0x18`: geometry, a flag word, the pass counter, a mode, the accumulator
/// (the slot later holds a byte), the pass count). Reads the property's flags and kinds into
/// local bytes, fades the environment map and specular by camera distance (`+0x34` against the
/// `float`s at `011f9468`/`011f9460` and `011f9458`), finds the sun light (`+0xe0` of the
/// accumulator's `+0x194`), then builds the 1.x passes: alpha passes ([`fn_00bd9f00`]); the
/// refraction pass ([`fn_00bd9da0`]) and nothing more (the exe then writes a byte at address 6,
/// a null-pointer store); the `+0x24` bit 6 pass ([`fn_00bd9e60`], the exe writing 7 at its
/// answer's `+6`); then, by the number of active non-shadow lights, the flag word, the mode
/// and the kinds, the lit passes (sun and each light: [`fn_00bd9770`], `00bd9540`, `00bd9840`,
/// [`bs_shader_pp_lighting_property_add_texture_pass_1x`], [`fn_00bd99c0`]) or the unlit ones
/// ([`fn_00bda0a0`]), environment map ([`fn_00bd9ac0`]), landscape
/// ([`bs_shader_pp_lighting_property_add_landscape_passes_1x`]), `00bd9bc0`, the texture
/// effect passes ([`fn_00bd9f90`], [`bs_shader_pp_lighting_property_add_texture_effect_pass_1x`],
/// each followed by the exe's store of 1 at the answer's `+6`) and the emittance pass
/// ([`fn_00bdb380`]). Finally every pass without an accumulation hint (`+6`) gets 10, 1 or 4
/// (the bytes the flags chose), or 0, and the render-pass list is the answer.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_get_render_passes_1x(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    pass_flags: u32,
    counter: u32,
    mode: u32,
    accumulator: u32,
    pass_count: u32,
) -> u32 {
    let base = this.addr();
    let flags0 = e.mem.u32(base + 0x20);
    let mut spec = flags0 & 1 != 0;
    let mut env = flags0 & 0x80 != 0;
    let alpha_flag = flags0 & 2 != 0;
    let f20 = flags0 & 1 != 0;
    let mut f35 = flags0 & 0x20 != 0 || mode == 3;
    let f18 = e.mem.u32(e.mem.u32(base + 0xb4)) != 0;
    let f14 = flags0 & 0x400 != 0;
    let model = e.mem.u32(geometry.wrapping_add(0xb8));
    let f1c = e.mem.u32(model.wrapping_add(0x28)) != 0 || e.mem.u8(base + 0x24) & 0x20 != 0;
    let f2c = e
        .call(KIND_TEST_00653290, &args![this, RTTI_01200788])
        .u32()
        != 0;
    let f2e = e
        .call(KIND_TEST_00653290, &args![this, RTTI_012024C8])
        .u32()
        != 0;
    let f2d = e
        .call(KIND_TEST_00653290, &args![this, RTTI_012007A0])
        .u32()
        != 0;
    let flags0 = e.mem.u32(base + 0x20);
    let f33 = flags0 & 0x4000 != 0;
    let f2f = flags0 & 0x8000 != 0;
    let f4 = flags0 & 0x10000 != 0;
    let mode_byte = flags0 & 0x200 != 0;
    let f32 = flags0 & 0x10_0000 == 0 || e.mem.u8(SWITCH_011F91A7) != 0;
    let f8 = flags0 & 0x4_0000 != 0;
    let f30 = flags0 & 0x400_0000 != 0 || flags0 & 0x800_0000 != 0;
    let skin = e.mem.u32(geometry.wrapping_add(0x9c));
    let f31 = if e.mem.u8(skin.wrapping_add(0x18)) & 1 != 0 {
        false
    } else if e.mem.u32(accumulator.wrapping_add(0x34)) == 2 {
        true
    } else {
        let flags1 = e.mem.u32(base + 0x24);
        flags1 & 4 != 0 || flags1 & 2 != 0
    };
    let alpha = e.call(GET_PROPERTY, &args![geometry, 0u32]).u32();
    let f24 = alpha != 0 && (e.mem.u16(alpha + 0x18) >> 9) & 1 != 0;
    let f10 = if alpha != 0 && e.mem.u8(alpha + 0x18) & 1 != 0 {
        true
    } else {
        1.0 > x87_m32(e.mem.u32(base + 0x28))
    };
    let mut bl = spec;
    let distance = x87_m32(e.mem.u32(base + 0x34));
    if e.mem.u32(base + 0x20) & 0x2_0000 != 0 {
        let c = x87_m32(e.mem.u32(FADE_011F9468));
        bl = false;
        env = within_fade(c, distance);
        spec = false;
    } else {
        let c = x87_m32(e.mem.u32(FADE_011F9460));
        if !within_fade(c, distance) {
            env = false;
        }
    }
    let c = x87_m32(e.mem.u32(FADE_011F9458));
    if !within_fade(c, distance) {
        bl = false;
        spec = false;
    }
    let shader_accumulator = e.call(GET_ACCUMULATOR, &args![]).u32();
    let sun = e.mem.u32(
        e.mem
            .u32(shader_accumulator.wrapping_add(0x194))
            .wrapping_add(0xe0),
    );
    let sun_specular = bl && light_color_differs(e, sun, 0xe0);
    let fc = u32::from(alpha_flag);
    let b = |v: bool| u32::from(v);
    e.with_stack(0xc, |e, slots| {
        let alpha_first = Ptr::<u8>::new(slots.addr());
        let first = Ptr::<u8>::new(slots.addr() + 4);
        let iterator = slots.addr() + 8;
        let counter_ptr = Ptr::<u16>::new(counter);
        let count = pass_count as u8;
        if f10 {
            e.mem.set_u8(alpha_first.addr(), 1);
            if mode_byte {
                fn_00bd9f00(
                    e,
                    this,
                    geometry,
                    counter_ptr,
                    count,
                    alpha_first,
                    alpha_flag,
                    f24,
                );
                spec = false;
            }
            f35 = true;
        }
        e.mem.set_u8(first.addr(), 1);
        if f2f || f4 {
            let r = fn_00bd9da0(e, this, geometry, counter_ptr, count, first, alpha_flag, f4);
            // The exe stores 6 at address 6 here (its builder answers 0, folded into a
            // constant address): a null-pointer write.
            e.mem.set_u8(6, 6);
            return r;
        }
        if e.mem.u8(base + 0x24) & 0x40 != 0 {
            let r = fn_00bd9e60(e, this, geometry, counter_ptr, count, first, alpha_flag);
            e.mem.set_u8(r.wrapping_add(6), 7);
        }
        let lights = e.call(ACTIVE_LIGHT_COUNT, &args![this]).u32() as u16;
        // Same order as the exe: the pass-flags word is read only when f32 is set and f35 is not.
        let unlit_or_lit = f32 && (f35 || pass_flags == e.mem.u32(PASS_FLAGS_010A91D4));
        if unlit_or_lit && !f35 && f33 {
            // Straight to the landscape passes.
            landscape_and_rest(
                e, this, geometry, pass_flags, counter, pass_count, first, sun, f35, alpha_flag,
                f2c, f10, f18,
            );
        } else if unlit_or_lit {
            if lights >= 1 && !f35 {
                e.mem.set_u32(iterator, 0);
                let mut light = e.call(FIRST_LIGHT, &args![this, iterator]).u32();
                if !f24 && !alpha_flag && !f18 {
                    fn_00bd9770(
                        e,
                        this,
                        geometry,
                        sun,
                        light,
                        counter_ptr,
                        count,
                        first,
                        false,
                        f2c,
                    );
                } else {
                    if pass_flags & 1 != 0 {
                        e.call(
                            PASSES_00BD9540,
                            &args![
                                this,
                                geometry,
                                sun,
                                counter,
                                pass_count,
                                first,
                                fc,
                                b(f24),
                                b(f18),
                                b(f2c)
                            ],
                        );
                    }
                    if light_color_differs(e, sun, 0xd4) {
                        e.call(
                            PASSES_00BD9840,
                            &args![
                                this,
                                geometry,
                                sun,
                                counter,
                                pass_count,
                                first,
                                fc,
                                0u32,
                                0u32,
                                b(f2c)
                            ],
                        );
                    }
                    if light_color_differs(e, light, 0xd4) {
                        e.call(
                            PASSES_00BD9840,
                            &args![
                                this,
                                geometry,
                                light,
                                counter,
                                pass_count,
                                first,
                                fc,
                                1u32,
                                0u32,
                                b(f2c)
                            ],
                        );
                    }
                }
                light = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                while light != 0 {
                    if light_color_differs(e, light, 0xd4) {
                        e.call(
                            PASSES_00BD9840,
                            &args![
                                this,
                                geometry,
                                light,
                                counter,
                                pass_count,
                                first,
                                fc,
                                1u32,
                                0u32,
                                b(f2c)
                            ],
                        );
                    }
                    light = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                }
                bs_shader_pp_lighting_property_add_texture_pass_1x(
                    e,
                    this,
                    geometry,
                    sun,
                    counter_ptr,
                    count,
                    first,
                    alpha_flag,
                    0,
                    f14,
                    f1c,
                    f2c,
                    f8,
                );
                if spec {
                    let mut light = e.call(FIRST_LIGHT, &args![this, iterator]).u32();
                    if sun_specular && light_color_differs(e, sun, 0xd4) {
                        fn_00bd99c0(
                            e,
                            this,
                            geometry,
                            sun,
                            counter_ptr,
                            count,
                            first,
                            alpha_flag,
                            false,
                            0,
                            b(f20),
                            0,
                            b(f2c),
                        );
                    } else {
                        fn_00bd99c0(
                            e,
                            this,
                            geometry,
                            light,
                            counter_ptr,
                            count,
                            first,
                            alpha_flag,
                            true,
                            0,
                            b(f20),
                            0,
                            b(f2c),
                        );
                        light = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                    }
                    while light != 0 {
                        if light_color_differs(e, light, 0xd4) {
                            fn_00bd99c0(
                                e,
                                this,
                                geometry,
                                light,
                                counter_ptr,
                                count,
                                first,
                                alpha_flag,
                                true,
                                0,
                                b(f20),
                                0,
                                b(f2c),
                            );
                        }
                        light = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                    }
                }
            } else {
                // No light (or the alpha passes were made): the unlit passes.
                let skip_unlit = lights < 1 && !f35 && sun_specular;
                if !skip_unlit && !f2e && !f2d {
                    fn_00bda0a0(
                        e,
                        this,
                        geometry,
                        sun,
                        counter_ptr,
                        count,
                        first,
                        alpha_flag,
                        f24,
                        f18,
                        f14,
                        true,
                        f1c,
                        f2c,
                    );
                    f35 = true;
                } else {
                    fn_00bda0a0(
                        e,
                        this,
                        geometry,
                        sun,
                        counter_ptr,
                        count,
                        first,
                        alpha_flag,
                        f24,
                        f18,
                        f14,
                        false,
                        f1c,
                        f2c,
                    );
                }
                if sun_specular && !f35 {
                    fn_00bd99c0(
                        e,
                        this,
                        geometry,
                        sun,
                        counter_ptr,
                        count,
                        first,
                        alpha_flag,
                        false,
                        0,
                        b(f20),
                        0,
                        b(f2c),
                    );
                }
            }
            if env {
                fn_00bd9ac0(
                    e,
                    this,
                    geometry,
                    counter_ptr,
                    count,
                    first,
                    alpha_flag,
                    f1c,
                );
            }
            if f33 {
                bs_shader_pp_lighting_property_add_landscape_passes_1x(
                    e,
                    this,
                    geometry,
                    sun,
                    counter_ptr,
                    count,
                    first,
                    false,
                );
            }
            rest_passes(
                e, this, geometry, pass_flags, counter, pass_count, first, f35, alpha_flag, f2c,
                f10, f18,
            );
        } else if f33 {
            landscape_and_rest(
                e, this, geometry, pass_flags, counter, pass_count, first, sun, f35, alpha_flag,
                f2c, f10, f18,
            );
        } else {
            e.mem.set_u8(first.addr(), 1);
            if pass_flags & 1 != 0 {
                e.call(
                    PASSES_00BD9540,
                    &args![
                        this,
                        geometry,
                        sun,
                        counter,
                        pass_count,
                        first,
                        fc,
                        b(f24),
                        b(f18),
                        b(f2c)
                    ],
                );
            }
            if pass_flags & 2 != 0 {
                if light_color_differs(e, sun, 0xd4) {
                    e.call(
                        PASSES_00BD9840,
                        &args![
                            this,
                            geometry,
                            sun,
                            counter,
                            pass_count,
                            first,
                            fc,
                            0u32,
                            0u32,
                            b(f2c)
                        ],
                    );
                }
                e.mem.set_u32(iterator, 0);
                let mut light = e.call(FIRST_LIGHT, &args![this, iterator]).u32();
                while light != 0 {
                    if light_color_differs(e, light, 0xd4) {
                        e.call(
                            PASSES_00BD9840,
                            &args![
                                this,
                                geometry,
                                light,
                                counter,
                                pass_count,
                                first,
                                fc,
                                1u32,
                                0u32,
                                b(f2c)
                            ],
                        );
                    }
                    light = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                }
            }
            if pass_flags & 4 != 0 {
                bs_shader_pp_lighting_property_add_texture_pass_1x(
                    e,
                    this,
                    geometry,
                    sun,
                    counter_ptr,
                    count,
                    first,
                    alpha_flag,
                    0,
                    f14,
                    f1c,
                    f2c,
                    f8,
                );
            }
            if spec && pass_flags & 8 != 0 {
                e.mem.set_u32(iterator, 0);
                let mut light = e.call(FIRST_LIGHT, &args![this, iterator]).u32();
                if sun_specular && light_color_differs(e, sun, 0xd4) {
                    fn_00bd99c0(
                        e,
                        this,
                        geometry,
                        sun,
                        counter_ptr,
                        count,
                        first,
                        alpha_flag,
                        false,
                        0,
                        b(f20),
                        0,
                        b(f2c),
                    );
                    e.mem.set_u8(first.addr(), 0);
                } else if light != 0 && light_color_differs(e, light, 0xd4) {
                    fn_00bd99c0(
                        e,
                        this,
                        geometry,
                        light,
                        counter_ptr,
                        count,
                        first,
                        alpha_flag,
                        true,
                        0,
                        b(f20),
                        0,
                        b(f2c),
                    );
                    light = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                }
                while light != 0 {
                    if light_color_differs(e, light, 0xd4) {
                        fn_00bd99c0(
                            e,
                            this,
                            geometry,
                            light,
                            counter_ptr,
                            count,
                            first,
                            alpha_flag,
                            true,
                            0,
                            b(f20),
                            0,
                            b(f2c),
                        );
                    }
                    light = e.call(NEXT_LIGHT, &args![this, iterator]).u32();
                }
            }
            if env && pass_flags & 8 != 0 {
                fn_00bd9ac0(
                    e,
                    this,
                    geometry,
                    counter_ptr,
                    count,
                    first,
                    alpha_flag,
                    f1c,
                );
            }
            rest_passes(
                e, this, geometry, pass_flags, counter, pass_count, first, f35, alpha_flag, f2c,
                f10, f18,
            );
        }
        let hint: u8 = if f31 {
            0xa
        } else if f30 {
            if e.mem.u32(base + 0x24) & 0x20_0000 != 0 {
                1
            } else {
                4
            }
        } else if f10 {
            1
        } else {
            0
        };
        let mut index = 0u32;
        let mut list = e.mem.u32(base + 0x3c);
        while index < e.mem.u32(list + 0x10) {
            let pass = e
                .mem
                .u32(e.mem.u32(list + 4).wrapping_add(index.wrapping_mul(4)));
            if e.mem.u8(pass.wrapping_add(6)) == 0 {
                e.mem.set_u8(pass.wrapping_add(6), hint);
            }
            list = e.mem.u32(base + 0x3c);
            index += 1;
        }
        list
    })
}

/// `GetRenderPasses_1x`'s landscape entry (its `00bdba92`): the landscape passes and the rest.
#[allow(clippy::too_many_arguments)]
fn landscape_and_rest(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    pass_flags: u32,
    counter: u32,
    pass_count: u32,
    first: Ptr<u8>,
    sun: u32,
    f35: bool,
    alpha_flag: bool,
    f2c: bool,
    f10: bool,
    f18: bool,
) {
    bs_shader_pp_lighting_property_add_landscape_passes_1x(
        e,
        this,
        geometry,
        sun,
        Ptr::new(counter),
        pass_count as u8,
        first,
        false,
    );
    rest_passes(
        e, this, geometry, pass_flags, counter, pass_count, first, f35, alpha_flag, f2c, f10, f18,
    );
}

/// `GetRenderPasses_1x`'s common tail (its `00bdbaac`): `00bd9bc0` when the flag word is the
/// one at `010a91d4` and the alpha passes were not made; the texture effect passes with
/// texture effect data (`+0xdc`; [`fn_00bd9f90`] first when its `+0x6c` is set), each answer
/// getting 1 at its `+6` as the exe stores it; and the emittance pass ([`fn_00bdb380`]) when
/// the byte at `011f9422` and bit 0 of the flag word are set.
#[allow(clippy::too_many_arguments)]
fn rest_passes(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    pass_flags: u32,
    counter: u32,
    pass_count: u32,
    first: Ptr<u8>,
    f35: bool,
    alpha_flag: bool,
    f2c: bool,
    f10: bool,
    f18: bool,
) {
    let counter_ptr = Ptr::<u16>::new(counter);
    let count = pass_count as u8;
    if pass_flags == e.mem.u32(PASS_FLAGS_010A91D4) && !f35 {
        e.call(
            PASSES_00BD9BC0,
            &args![
                this,
                geometry,
                counter,
                pass_count,
                first,
                u32::from(alpha_flag),
                u32::from(f2c),
                u32::from(f10)
            ],
        );
    }
    let data = e.mem.u32(this.addr() + 0xdc);
    if data != 0 {
        if e.mem.u32(data + 0x6c) != 0 {
            let r = fn_00bd9f90(e, this, geometry, counter_ptr, count, first, alpha_flag);
            e.mem.set_u8(r.wrapping_add(6), 1);
        }
        let r = bs_shader_pp_lighting_property_add_texture_effect_pass_1x(
            e,
            this,
            geometry,
            counter_ptr,
            count,
            first,
            alpha_flag,
        );
        e.mem.set_u8(r.wrapping_add(6), 1);
    }
    if e.mem.u8(SWITCH_011F9422) != 0 && pass_flags & 1 != 0 {
        fn_00bdb380(
            e,
            this,
            geometry,
            counter_ptr,
            count,
            u8::from(alpha_flag),
            u32::from(f18),
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00bd_9e60,
            fn_00bd9e60(Ptr<BSShaderProperty>, u32, Ptr<u16>, u8, Ptr<u8>, bool) -> u32
        ),
        entry!(
            0x00bd_9f00,
            fn_00bd9f00(Ptr<BSShaderProperty>, u32, Ptr<u16>, u8, Ptr<u8>, bool, bool) -> u32
        ),
        entry!(
            0x00bd_ac00,
            bs_shader_pp_lighting_property_add_texture_pass_1x(
                Ptr<BSShaderProperty>,
                u32,
                u32,
                Ptr<u16>,
                u8,
                Ptr<u8>,
                bool,
                u32,
                bool,
                bool,
                bool,
                bool,
            ) -> u32
        ),
        entry!(
            0x00bd_9770,
            fn_00bd9770(
                Ptr<BSShaderProperty>,
                u32,
                u32,
                u32,
                Ptr<u16>,
                u8,
                Ptr<u8>,
                bool,
                bool,
            ) -> u32
        ),
        entry!(
            0x00bd_99c0,
            fn_00bd99c0(
                Ptr<BSShaderProperty>,
                u32,
                u32,
                Ptr<u16>,
                u8,
                Ptr<u8>,
                bool,
                bool,
                u32,
                u32,
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x00bd_9ac0,
            fn_00bd9ac0(Ptr<BSShaderProperty>, u32, Ptr<u16>, u8, Ptr<u8>, bool, bool) -> u32
        ),
        entry!(
            0x00bd_9d00,
            bs_shader_pp_lighting_property_add_texture_effect_pass_1x(
                Ptr<BSShaderProperty>,
                u32,
                Ptr<u16>,
                u8,
                Ptr<u8>,
                bool,
            ) -> u32
        ),
        entry!(
            0x00bd_9da0,
            fn_00bd9da0(Ptr<BSShaderProperty>, u32, Ptr<u16>, u8, Ptr<u8>, bool, bool) -> u32
        ),
        entry!(
            0x00bd_9f90,
            fn_00bd9f90(Ptr<BSShaderProperty>, u32, Ptr<u16>, u8, Ptr<u8>, bool) -> u32
        ),
        entry!(
            0x00bd_a030,
            fn_00bda030(Ptr<BSShaderProperty>, u32, u32, u8, bool) -> u32
        ),
        entry!(
            0x00bd_a060,
            fn_00bda060(Ptr<BSShaderProperty>, u32, u8, u8) -> u32
        ),
        entry!(
            0x00bd_a0a0,
            fn_00bda0a0(
                Ptr<BSShaderProperty>,
                u32,
                u32,
                Ptr<u16>,
                u8,
                Ptr<u8>,
                bool,
                bool,
                bool,
                bool,
                bool,
                bool,
                bool,
            ) -> u32
        ),
        entry!(
            0x00bd_af10,
            bs_shader_pp_lighting_property_add_landscape_passes_1x(
                Ptr<BSShaderProperty>,
                u32,
                u32,
                Ptr<u16>,
                u8,
                Ptr<u8>,
                bool,
            )
        ),
        entry!(
            0x00bd_b380,
            fn_00bdb380(Ptr<BSShaderProperty>, u32, Ptr<u16>, u8, u8, u32) -> u32
        ),
        entry!(
            0x00bd_b4a0,
            bs_shader_pp_lighting_property_get_render_passes_1x(
                Ptr<BSShaderProperty>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST: u32 = 0x0070_0000;

    /// A property whose render-pass list is `LIST`, a counter of 5, a first-pass byte of 1,
    /// and `Add` and the formatter doubled.
    fn setup() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
        e.register(FORMAT, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x3c, LIST);
        let counter = e.mem.alloc(2);
        e.mem.set_u16(counter, 5);
        let first = e.mem.alloc(1);
        e.mem.set_u8(first, 1);
        e.call_log = Some(vec![]);
        (e, this, counter, first)
    }

    fn calls_to(e: &mut Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn fn_00bd9e60_adds_1e5_or_1e6_or_counts() {
        for (flag, count, pass) in [(0u32, 1u32, Some(0x1e5)), (1, 1, Some(0x1e6)), (1, 2, None)] {
            let (mut e, this, counter, first) = setup();
            let r = e.call(0x00bd_9e60, &[this, 0x1234, counter, count, first, flag]);
            assert_eq!(r.u32(), 0);
            assert_eq!(e.mem.u8(first), 0);
            let calls = calls_to(&mut e, RENDER_PASS_ARRAY_ADD);
            match pass {
                Some(pass) => {
                    assert_eq!(calls, vec![vec![LIST, 0x1234, pass, 1, 0, 0, 0, 0, 0]]);
                    assert_eq!(e.mem.u16(counter), 5);
                }
                None => {
                    assert!(calls.is_empty());
                    assert_eq!(e.mem.u16(counter), 6);
                }
            }
        }
    }

    #[test]
    fn fn_00bd9f00_picks_3_2_or_1() {
        for (flag, second, pass) in [(1u32, 0u32, 3u32), (0, 1, 2), (0, 0, 1), (0, 0x100, 1)] {
            let (mut e, this, counter, first) = setup();
            e.call(0x00bd_9f00, &[this, 0x10, counter, 1, first, flag, second]);
            assert_eq!(
                calls_to(&mut e, RENDER_PASS_ARRAY_ADD),
                vec![vec![LIST, 0x10, pass, 1, 0, 0, 0, 0, 0]]
            );
            assert_eq!(e.mem.u8(first), 0);
        }
        // Only the low byte of the pass count is compared.
        let (mut e, this, counter, first) = setup();
        e.call(0x00bd_9f00, &[this, 0x10, counter, 0x102, first, 0, 0]);
        assert!(calls_to(&mut e, RENDER_PASS_ARRAY_ADD).is_empty());
        assert_eq!(e.mem.u16(counter), 6);
        assert_eq!(e.mem.u8(first), 0);
    }

    /// Flags in order 6, 8, 9, 10, 11.
    fn texture_pass(
        e: &mut Engine,
        this: u32,
        counter: u32,
        first: u32,
        f: [u32; 5],
        geometry: u32,
    ) {
        let r = e.call(
            0x00bd_ac00,
            &[
                this, geometry, 0, counter, 1, first, f[0], 0, f[1], f[2], f[3], f[4],
            ],
        );
        assert_eq!(r.u32(), 0);
    }

    #[test]
    fn add_texture_pass_1x_chooses_each_pass() {
        let cases = [
            ([0u32, 0, 0, 1, 0], 0x151u32),
            ([0, 0, 1, 0, 1], 0x14c),
            ([0, 0, 1, 0, 0], 0x14e),
            ([0, 1, 0, 0, 0], 0x14b),
            ([0, 0, 0, 0, 0], 0x14a),
            ([1, 0, 0, 0, 0], 0x14d),
            ([1, 0, 1, 0, 1], 0x152),
            ([1, 0, 1, 0, 0], 0x14f),
            ([1, 1, 0, 0, 0], 0x150),
        ];
        for (flags, pass) in cases {
            let (mut e, this, counter, first) = setup();
            texture_pass(&mut e, this, counter, first, flags, 0x40);
            assert_eq!(
                calls_to(&mut e, RENDER_PASS_ARRAY_ADD),
                vec![vec![LIST, 0x40, pass, 1, 0, 0, 0, 0, 0]],
                "{flags:?}"
            );
            assert_eq!(e.mem.u8(first), 0);
        }
        // A pass count other than 1 counts.
        let (mut e, this, counter, first) = setup();
        e.call(
            0x00bd_ac00,
            &[this, 0x40, 0, counter, 2, first, 0, 0, 0, 0, 0, 0],
        );
        assert!(calls_to(&mut e, RENDER_PASS_ARRAY_ADD).is_empty());
        assert_eq!(e.mem.u16(counter), 6);
    }

    #[test]
    fn add_texture_pass_1x_reports_skinned_facegen_without_counting() {
        // No geometry.
        let (mut e, this, counter, first) = setup();
        texture_pass(&mut e, this, counter, first, [1, 1, 1, 0, 0], 0);
        let calls = calls_to(&mut e, FORMAT);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0][1..], [0x80, NO_NAME_FORMAT_010250CC]);
        assert_eq!(
            calls[1][1..],
            [0xff, SKINNED_FACEGEN_ERROR_FORMAT, calls[0][0], 0]
        );
        assert_eq!(calls[1][0], calls[0][0] + 0x80);
        assert_eq!(e.mem.u16(counter), 5);
        assert_eq!(e.mem.u8(first), 0);

        // Parent and child names, parent only, own name only, neither.
        for (parent_name, name, expected) in [
            (0x111u32, 0x222u32, vec![PARENT_CHILD_FORMAT, 0x111, 0x222]),
            (0x111, 0, vec![PARENT_FORMAT, 0x111]),
            (0, 0x222, vec![NAME_FORMAT_01019F08, 0x222]),
            (0, 0, vec![NO_NAME_FORMAT_010250CC]),
        ] {
            let (mut e, this, counter, first) = setup();
            let geometry = e.mem.alloc(0x20);
            let parent = e.mem.alloc(0x20);
            e.mem.set_u32(geometry + 0x18, parent);
            e.mem.set_u32(parent + 8, parent_name);
            e.mem.set_u32(geometry + 8, name);
            texture_pass(&mut e, this, counter, first, [1, 1, 1, 0, 0], geometry);
            let calls = calls_to(&mut e, FORMAT);
            assert_eq!(calls[0][2..], expected[..]);
            assert_eq!(calls[1][4], geometry);
            assert_eq!(e.mem.u16(counter), 5);
        }
        // A geometry without a parent uses its own name.
        let (mut e, this, counter, first) = setup();
        let geometry = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 8, 0x333);
        texture_pass(&mut e, this, counter, first, [1, 1, 1, 0, 0], geometry);
        assert_eq!(
            calls_to(&mut e, FORMAT)[0][2..],
            [NAME_FORMAT_01019F08, 0x333]
        );
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

    /// A property with a render-pass list whose `Add` double appends a 0x10-byte pass (the
    /// pass enum at `+4`, hint `+6` clear) and records the call; a counter and a first-pass
    /// byte holding 1.
    fn rig() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x010a_9000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        let this = e.mem.alloc(0x104);
        let list = e.mem.alloc(0x14);
        let passes = e.mem.alloc(0x100);
        e.mem.set_u32(list + 4, passes);
        e.mem.set_u32(this + 0x3c, list);
        e.register(RENDER_PASS_ARRAY_ADD, |e, a| {
            let list = a[0];
            let used = e.mem.u32(list + 0x10);
            let pass = e.mem.alloc(0x10);
            e.mem.set_u16(pass + 4, a[2] as u16);
            let passes = e.mem.u32(list + 4);
            e.mem.set_u32(passes + 4 * used, pass);
            e.mem.set_u32(list + 0x10, used + 1);
            Ret::default()
        });
        let counter = e.mem.alloc(4);
        let first = e.mem.alloc(4);
        e.mem.set_u8(first, 1);
        e.call_log = Some(vec![]);
        (e, this, counter, first)
    }

    fn added(e: &Engine) -> Vec<Vec<u32>> {
        calls(e, RENDER_PASS_ARRAY_ADD)
            .into_iter()
            .map(|w| w[1..].to_vec())
            .collect()
    }

    #[test]
    fn simple_builders_choose_add_or_count() {
        let (mut e, this, counter, first) = rig();
        let words = |count: u32, f: u32, g: u32| args![this, 0x60u32, counter, count, first, f, g];
        e.call(0x00bd_9ac0, &words(1, 1, 0));
        assert_eq!(e.mem.u8(first), 0);
        e.call(0x00bd_9da0, &words(1, 1, 0));
        e.call(0x00bd_9da0, &words(1, 0, 1));
        e.call(0x00bd_9ac0, &words(0, 0, 0));
        assert_eq!(e.mem.u16(counter), 1);
        e.call(
            0x00bd_9d00,
            &args![this, 0x60u32, counter, 1u32, first, 1u32],
        );
        e.call(
            0x00bd_9f90,
            &args![this, 0x60u32, counter, 1u32, first, 0u32],
        );
        let passes: Vec<u32> = added(&e).iter().map(|w| w[1]).collect();
        assert_eq!(passes, vec![0x246, 0x1e3, 0x1e4, 0x24f, 4]);
        assert_eq!(
            e.call(
                0x00bd_9f90,
                &args![this, 0x60u32, counter, 2u32, first, 1u32]
            )
            .u32(),
            0
        );
        assert_eq!(e.mem.u16(counter), 2);
    }

    #[test]
    fn lit_builders_pass_their_lights() {
        let (mut e, this, counter, first) = rig();
        e.call(
            0x00bd_9770,
            &args![this, 0x60u32, 0x11u32, 0x22u32, counter, 1u32, first, 0u32, 1u32],
        );
        e.mem.set_u8(first, 0);
        e.call(
            0x00bd_99c0,
            &args![
                this, 0x60u32, 0x33u32, counter, 1u32, first, 1u32, 0u32, 9u32, 9u32, 9u32, 9u32
            ],
        );
        assert_eq!(
            added(&e),
            vec![
                vec![0x60, 0x7e, 1, 2, 0x11, 0x22, 0, 0],
                vec![0x60, 0x156, 0, 1, 0x33, 0, 0, 0]
            ]
        );
        e.call(
            0x00bd_9770,
            &args![this, 0x60u32, 0x11u32, 0x22u32, counter, 0u32, first, 1u32, 0u32],
        );
        assert_eq!(e.mem.u16(counter), 1);
    }

    #[test]
    fn si_block_and_const_alpha_passes() {
        let (mut e, this, _, _) = rig();
        e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
        let si = e.mem.alloc(0x14);
        let alpha = e.mem.alloc(0x14);
        e.call(0x00bd_a030, &args![this, 0x60u32, 0u32, 1u32, 1u32]);
        assert!(added(&e).is_empty());
        e.call(0x00bd_a030, &args![this, 0x60u32, 0u32, 1u32, 1u32]);
        e.mem.set_u32(this + 0x4c, si);
        e.mem.set_u32(this + 0x44, alpha);
        e.call(0x00bd_a030, &args![this, 0x60u32, 0u32, 1u32, 1u32]);
        e.call(0x00bd_a060, &args![this, 0x60u32, 0u32, 1u32]);
        e.call(0x00bd_a060, &args![this, 0x60u32, 1u32, 0u32]);
        let made = calls(&e, RENDER_PASS_ARRAY_ADD);
        assert_eq!(made[0], vec![si, 0x60, 0x15, 0, 0, 0, 0, 0, 0]);
        assert_eq!(made[1], vec![alpha, 0x60, 7, 1, 0, 0, 0, 0, 0]);
        assert_eq!(made[2][2], 8);
    }

    #[test]
    fn unlit_pass_follows_the_decision_tree() {
        let (mut e, this, counter, first) = rig();
        let unlit = |e: &mut Engine, count: u32, flags: [u32; 7]| {
            let mut words = args![this, 0x60u32, 0x5eu32, counter, count, first];
            words.extend_from_slice(&flags);
            e.call(0x00bd_a0a0, &words);
        };
        unlit(&mut e, 1, [0, 0, 0, 0, 0, 0, 0]);
        unlit(&mut e, 1, [1, 1, 1, 1, 1, 0, 0]);
        unlit(&mut e, 1, [1, 1, 1, 0, 0, 0, 0]);
        unlit(&mut e, 1, [0, 0, 0, 0, 1, 1, 1]);
        unlit(&mut e, 1, [0, 0, 0, 0, 1, 1, 0]);
        unlit(&mut e, 1, [1, 1, 1, 1, 1, 1, 0]);
        let passes: Vec<u32> = added(&e).iter().map(|w| w[1]).collect();
        assert_eq!(passes, vec![0x64, 0x92, 0x76, 0x7b, 0x80]);
        assert_eq!(added(&e)[0], vec![0x60, 0x64, 1, 1, 0x5e, 0, 0, 0]);
        unlit(&mut e, 0, [0, 0, 0, 0, 0, 0, 0]);
        unlit(&mut e, 0, [1, 1, 1, 1, 1, 1, 0]);
        assert_eq!(e.mem.u16(counter), 1);
        assert_eq!(e.mem.u8(first), 0);
    }

    #[test]
    fn emittance_pass_needs_a_coloured_emittance() {
        let (mut e, this, counter, _) = rig();
        e.register(COLOR_COMPARE_00439090, |e, a| {
            u32::from(e.mem.u32(a[0]) != 0).into_ret()
        });
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(RENDER_PASS_CONSTRUCT, |_, a| a[0].into_ret());
        let geometry = e.mem.alloc(0xb0);
        assert_eq!(
            e.call(
                0x00bd_b380,
                &args![this, geometry, counter, 1u32, 0u32, 0u32]
            )
            .u32(),
            0
        );
        let material = e.mem.alloc(0x40);
        e.mem.set_u32(geometry + 0xa4, material);
        assert_eq!(
            e.call(
                0x00bd_b380,
                &args![this, geometry, counter, 1u32, 0u32, 0u32]
            )
            .u32(),
            0
        );
        e.mem.set_f32(material + 0x28, 0.5);
        let previous = e.mem.alloc(0x10);
        let list = e.mem.u32(this + 0x3c);
        e.mem.set_u32(e.mem.u32(list + 4), previous);
        e.mem.set_u32(list + 0x10, 1);
        let pass = e
            .call(
                0x00bd_b380,
                &args![this, geometry, counter, 1u32, 1u32, 0u32],
            )
            .u32();
        assert_ne!(pass, 0);
        assert_eq!(e.mem.u8(previous + 8), 1);
        assert_eq!(
            calls(&e, RENDER_PASS_CONSTRUCT),
            vec![vec![pass, geometry, 0x2d7, 0, 0, 0]]
        );
        assert_eq!(
            e.call(
                0x00bd_b380,
                &args![this, geometry, counter, 2u32, 0u32, 0u32]
            )
            .u32(),
            0
        );
        assert_eq!(e.mem.u16(counter), 1);
    }

    #[test]
    fn landscape_passes_add_one_per_layer() {
        let (mut e, this, counter, first) = rig();
        let property = e.mem.alloc(0x104);
        let base = e.mem.alloc(0x24);
        let glow = e.mem.alloc(0x24);
        e.mem.set_u32(base + 4, 0x7001);
        e.mem.set_u32(base + 8, 0x7002);
        e.mem.set_u32(glow + 8, 0x7102);
        e.mem.set_u32(property + 0xac, base);
        e.mem.set_u32(property + 0xb4, glow);
        e.register_double(GET_PROPERTY, move |_, _| property.into_ret());
        e.mem.set_u16(this + 0xc8, 5);
        e.mem.set_u32(LANDSCAPE_LAYERS_011AD828, 2);
        e.mem.set_u32(SHADER_LEVEL_011F91C0, 2);
        e.mem.set_u8(SWITCH_011F91A7, 1);
        e.call(
            0x00bd_af10,
            &args![this, 0x60u32, 0x5au32, counter, 1u32, first, 0u32],
        );
        let passes: Vec<u32> = added(&e).iter().map(|w| w[1]).collect();
        assert_eq!(passes, vec![0x93, 0x1f2, 0x1f2]);
        let list = e.mem.u32(this + 0x3c);
        let last = e.mem.u32(e.mem.u32(list + 4) + 8);
        assert_eq!(e.mem.u8(last + 0xb), 2);
        // The full path at a low shader level: one lit light.
        e.mem.set_u8(SWITCH_011F91A7, 0);
        e.mem.set_u32(SHADER_LEVEL_011F91C0, 1);
        let light = e.mem.alloc(0x100);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(light + 0xf8, data);
        e.mem.set_f32(data + 0xd8, 1.0);
        e.register_double(FIRST_LIGHT, move |e, a| {
            e.mem.set_u32(a[1], 1);
            light.into_ret()
        });
        e.register(NEXT_LIGHT, |_, _| Ret::default());
        e.mem.set_u8(first, 1);
        e.call(
            0x00bd_af10,
            &args![this, 0x60u32, 0x5au32, counter, 1u32, first, 1u32],
        );
        let passes: Vec<u32> = added(&e)[3..].iter().map(|w| w[1]).collect();
        assert_eq!(passes, vec![0x94, 0x1ef, 0x1f3, 0x1f3, 0x1f5, 0x1f4, 0x1f5]);
        assert_eq!(added(&e)[3], vec![0x60, 0x94, 1, 1, 0x5a, 0, 0, 0]);
        assert_eq!(added(&e)[4], vec![0x60, 0x1ef, 0, 1, light, 0, 0, 0]);
    }

    /// The doubles `GetRenderPasses_1x` needs: kind tests answering no, no alpha property,
    /// an accumulator with a sun, the given active light count and colour compare.
    fn render_rig(e: &mut Engine, lights: u32, sun: u32) {
        e.register(KIND_TEST_00653290, |_, _| Ret::default());
        e.register(GET_PROPERTY, |_, _| Ret::default());
        let accumulator = e.mem.alloc(0x200);
        let holder = e.mem.alloc(0x100);
        e.mem.set_u32(holder + 0xe0, sun);
        e.mem.set_u32(accumulator + 0x194, holder);
        e.register_double(GET_ACCUMULATOR, move |_, _| accumulator.into_ret());
        e.register_double(ACTIVE_LIGHT_COUNT, move |_, _| lights.into_ret());
        e.register(COLOR_COMPARE_00439090, |e, a| {
            u32::from(e.mem.u32(a[0]) != 0).into_ret()
        });
        e.register(PASSES_00BD9BC0, |_, _| Ret::default());
        e.mem.set_u32(PASS_FLAGS_010A91D4, 0x55);
    }

    fn geometry(e: &mut Engine) -> u32 {
        let geometry = e.mem.alloc(0xc0);
        let model = e.mem.alloc(0x40);
        let skin = e.mem.alloc(0x20);
        e.mem.set_u8(skin + 0x18, 1);
        e.mem.set_u32(geometry + 0xb8, model);
        e.mem.set_u32(geometry + 0x9c, skin);
        geometry
    }

    #[test]
    fn render_passes_1x_without_lights_make_the_unlit_pass() {
        let (mut e, this, counter, _) = rig();
        let sun = e.mem.alloc(0x100);
        let sun_data = e.mem.alloc(0x100);
        e.mem.set_u32(sun + 0xf8, sun_data);
        render_rig(&mut e, 0, sun);
        let glow = e.mem.alloc(8);
        e.mem.set_u32(this + 0xb4, glow);
        e.mem.set_f32(this + 0x28, 1.0);
        let geometry = geometry(&mut e);
        let accumulator = e.mem.alloc(0x40);
        e.mem.set_u32(accumulator + 0x34, 2);
        let list = e
            .call(
                0x00bd_b4a0,
                &args![this, geometry, 0x55u32, counter, 0u32, accumulator, 1u32],
            )
            .u32();
        assert_eq!(list, e.mem.u32(this + 0x3c));
        assert_eq!(added(&e), vec![vec![geometry, 0x7f, 1, 1, sun, 0, 0, 0]]);
        // The skin byte was set: no hint 10; nothing else asked for one either.
        let pass = e.mem.u32(e.mem.u32(list + 4));
        assert_eq!(e.mem.u8(pass + 6), 0);
        assert!(calls(&e, PASSES_00BD9BC0).is_empty());
    }

    #[test]
    fn render_passes_1x_with_a_light_make_the_lit_passes() {
        let (mut e, this, counter, _) = rig();
        let sun = e.mem.alloc(0x100);
        let sun_data = e.mem.alloc(0x100);
        e.mem.set_u32(sun + 0xf8, sun_data);
        render_rig(&mut e, 1, sun);
        let light = e.mem.alloc(0x100);
        e.register_double(FIRST_LIGHT, move |_, _| light.into_ret());
        e.register(NEXT_LIGHT, |_, _| Ret::default());
        let glow = e.mem.alloc(8);
        e.mem.set_u32(this + 0xb4, glow);
        e.mem.set_f32(this + 0x28, 1.0);
        let geometry = geometry(&mut e);
        e.mem.set_u8(e.mem.u32(geometry + 0x9c) + 0x18, 0);
        let accumulator = e.mem.alloc(0x40);
        e.mem.set_u32(accumulator + 0x34, 2);
        e.call(
            0x00bd_b4a0,
            &args![this, geometry, 0x55u32, counter, 0u32, accumulator, 1u32],
        );
        assert_eq!(
            added(&e),
            vec![
                vec![geometry, 0x7c, 1, 2, sun, light, 0, 0],
                vec![geometry, 0x14a, 0, 0, 0, 0, 0, 0]
            ]
        );
        assert_eq!(calls(&e, PASSES_00BD9BC0).len(), 1);
        // f31 (the accumulator's +0x34 is 2): every pass gets the hint 10.
        let list = e.mem.u32(this + 0x3c);
        for i in 0..2 {
            let pass = e.mem.u32(e.mem.u32(list + 4) + 4 * i);
            assert_eq!(e.mem.u8(pass + 6), 0xa);
        }
    }
}
