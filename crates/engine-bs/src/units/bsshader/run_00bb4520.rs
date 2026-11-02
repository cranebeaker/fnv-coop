//! `BSShader/run_00bb4520` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsshader::bsshaderproperty::RenderPass;

/// `00aa13e0` (`cdecl`, one word): the object allocation; `00aa1460`
/// (`cdecl`, `(block, size)`): the sized free.
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
/// The memory manager (`011f6238`) and its `Allocate` (`00aa3e40`,
/// `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The vtable of this unit's class (`010b9910`, no RTTI; built on
/// `BSShaderPPLightingProperty`).
const VTABLE_010B9910: u32 = 0x010b_9910;
/// `BSShaderPPLightingProperty`'s constructor (`00b66f50`), destructor
/// (`00b67380`), `SetupGeometry` (`00b67a40`, one word) and the base member
/// copy (`00b665d0`, `thiscall`, `(clone, process)`).
const PP_LIGHTING_CONSTRUCTOR: u32 = 0x00b6_6f50;
const PP_LIGHTING_DESTRUCTOR: u32 = 0x00b6_7380;
const PP_LIGHTING_SETUP_GEOMETRY: u32 = 0x00b6_7a40;
const PP_LIGHTING_COPY_MEMBERS: u32 = 0x00b6_65d0;
/// `BSShaderProperty::RenderPass::RenderPass` (`00ba8ec0`, `cdecl`, the
/// pass, geometry, pass enum, first-pass flag, light count and the lights).
const RENDER_PASS_CONSTRUCTOR: u32 = 0x00ba_8ec0;
/// `NiAVObject::GetProperty` (`00a59d30`, `thiscall`, one word) and
/// `00654640` (unnamed, `thiscall`, one word: the property).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const USE_PROPERTY_00654640: u32 = 0x0065_4640;
/// The thread block's word at +0x2B4 (the allocation context), 0x28 while
/// the clone is made.
const TLS_ALLOCATION_CONTEXT: u32 = 0x2b4;

/// `004a2020` (unnamed, `thiscall`, one word: a flag number): whether bit
/// `n % 32` of the flag word `n / 32` at +0x20 is set (a byte).
const HAS_FLAG: u32 = 0x004a_2020;
/// `BSShaderProperty::SetFlag` (`00441130`, `thiscall`, `(flag, value)`).
const SET_FLAG: u32 = 0x0044_1130;
/// `BSShaderProperty`'s pass-array makers: `CheckCreateRenderPassArray`
/// (`00ba95e0`), `CheckCreateDepthMapRenderPassArray` (`00ba9660`),
/// `CheckCreateConstAlphaRenderPassArray` (`00ba9740`) and the two unnamed
/// ones of the local-map (`00ba96d0`) and +0x54 (`00ba9820`) arrays; each
/// `thiscall` with one word.
const CHECK_CREATE_RENDER_PASSES: u32 = 0x00ba_95e0;
const CHECK_CREATE_DEPTH_MAP_PASSES: u32 = 0x00ba_9660;
const CHECK_CREATE_CONST_ALPHA_PASSES: u32 = 0x00ba_9740;
const CHECK_CREATE_LOCAL_MAP_PASSES: u32 = 0x00ba_96d0;
const CHECK_CREATE_PASSES_54: u32 = 0x00ba_9820;
/// `BSShaderProperty::RenderPassArray::Add` (`00ba9ee0`, `thiscall`, eight
/// words: geometry, pass enum, first-pass flag, light count, four lights).
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// The two light-array helpers of a pass (`00ba8c30` frees it, `00ba8c00`
/// allocates one for a count).
const PASS_FREE_LIGHTS: u32 = 0x00ba_8c30;
const PASS_ALLOCATE_LIGHTS: u32 = 0x00ba_8c00;
/// `BSShaderManager::GetAccumulator` (`00b4f5c0`, no words).
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
/// `BSShaderProperty::SetAlpha` (`00ba8ab0`, one `float`).
const SET_ALPHA: u32 = 0x00ba_8ab0;
/// `BSShaderLightingProperty`'s light walk (each `thiscall` with the
/// iterator's address): `ResortLights` (`00b70390`, with a bound),
/// `GetFirstActiveLight` (`00b70590`), `GetNextActiveLight` (`00b70680`),
/// `GetFirstActiveNonShadowLight` (`00b70600`),
/// `GetNextActiveNonShadowLight` (`00b70700`).
const RESORT_LIGHTS: u32 = 0x00b7_0390;
const FIRST_ACTIVE_LIGHT: u32 = 0x00b7_0590;
const NEXT_ACTIVE_LIGHT: u32 = 0x00b7_0680;
const FIRST_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0600;
const NEXT_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0700;
/// `NiPoint3::Length` (`00457990`, `thiscall`, the length in ST0).
const VECTOR_LENGTH: u32 = 0x0045_7990;
/// `004b3ab0` (unnamed, `cdecl`, five `float`s `(a, b, c, d, x)`): `a + (b -
/// a) * (x - c) / (d - c)` in ST0.
const INTERPOLATE: u32 = 0x004b_3ab0;
/// `005bf7b0`, `00b60330` (no words, a byte) and `00b60170` (no words, a
/// word): unnamed tests of the render settings.
const DEPTH_PASSES_WANTED: u32 = 0x005b_f7b0;
const SETTING_TEST_00B60330: u32 = 0x00b6_0330;
const SETTING_VALUE_00B60170: u32 = 0x00b6_0170;

/// The renderer singleton word (`011f4748`): its +8 object's word +0x1A0 is
/// the light the depth and constant-alpha passes get.
const RENDERER: u32 = 0x011f_4748;
/// The default bound (`011f4288`) used when the geometry has none at +0x20.
const DEFAULT_BOUND: u32 = 0x011f_4288;
/// The fade distances the passes compare the camera distance with.
const FADE_FAR_011F9458: u32 = 0x011f_9458;
const FADE_FAR_011F9460: u32 = 0x011f_9460;
const FADE_FAR_011F9468: u32 = 0x011f_9468;
const FADE_NEAR_011F946C: u32 = 0x011f_946c;
const FADE_FAR_011F9470: u32 = 0x011f_9470;
/// A byte that keeps the passes on when bit 20 of the flags is set
/// (`011f91a7`).
const PASSES_FORCED_011F91A7: u32 = 0x011f_91a7;

/// `BSShaderProperty` fields (Xbox PDB): `ulFlags[0]` +0x20, `ulFlags[1]`
/// +0x24, `fAlpha` +0x28, `fFadeAlpha` +0x2C, `fCameraDistance` +0x34,
/// `iLastRenderPassState` +0x38, `pRenderPassList` +0x3C,
/// `pDepthMapRenderPassList` +0x40, `pConstAlphaRenderPassList` +0x44,
/// `pLocalMapRenderPassList` +0x48, the +0x54 list, `iShader` +0x58,
/// `fDepthBias` +0x5C; the byte at +0x74 that asks for the lights to be
/// resorted; `iShaderPropertyType` +0x1C.
const FLAGS0: u32 = 0x20;
const FLAGS1: u32 = 0x24;
const ALPHA: u32 = 0x28;
const FADE_ALPHA: u32 = 0x2c;
const CAMERA_DISTANCE: u32 = 0x34;
const LAST_STATE: u32 = 0x38;
const PASSES: u32 = 0x3c;
const DEPTH_PASSES: u32 = 0x40;
const CONST_ALPHA_PASSES: u32 = 0x44;
const LOCAL_MAP_PASSES: u32 = 0x48;
const PASSES_54: u32 = 0x54;
const SHADER: u32 = 0x58;
const DEPTH_BIAS: u32 = 0x5c;
const LIGHTS_DIRTY: u32 = 0x74;
const SHADER_PROPERTY_TYPE: u32 = 0x1c;
/// The pass at +0x100 `00bb5dd0` makes, and the words at +0xB4 and +0xDC.
const OWN_PASS: u32 = 0x100;
const WORD_B4: u32 = 0xb4;
const OBJECT_DC: u32 = 0xdc;

/// The flag test `004a2020` on `this`.
fn has_flag(e: &mut Engine, this: u32, flag: u32) -> bool {
    e.call(HAS_FLAG, &args![this, flag]).bool()
}

/// The renderer's light word (`[[011f4748] + 8] + 0x1A0`).
fn renderer_light(e: &mut Engine) -> u32 {
    let renderer = e.global::<u32>(RENDERER);
    let object = e.mem.u32(renderer + 8);
    e.mem.u32(object + 0x1a0)
}

/// The accumulator's +0x194 object.
fn accumulator_object(e: &mut Engine) -> u32 {
    let accumulator = e.call(GET_ACCUMULATOR, &[]).u32();
    e.mem.u32(accumulator + 0x194)
}

/// `RenderPassArray::Add` on `list` with the eight words.
fn add_pass(e: &mut Engine, list: u32, words: [u32; 8]) {
    let mut call = vec![list];
    call.extend_from_slice(&words);
    e.call(RENDER_PASS_ARRAY_ADD, &call);
}

/// The last pass of the main list (`[list+4][iSize - 1]`).
fn last_pass(e: &mut Engine, this: u32) -> Ptr<RenderPass> {
    let list = e.mem.u32(this + PASSES);
    let size = e.mem.u32(list + 0x10);
    let items = e.mem.u32(list + 4);
    Ptr::new(
        e.mem
            .u32(items.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4)),
    )
}

/// A float field or global as the x87 stack holds it after `FLD dword`.
fn load(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// The pass array's first pass gets the renderer's light at its first light
/// slot when it has a light (`ucNumLights` above 0).
fn refresh_first_light(e: &mut Engine, list: u32) {
    let pass: Ptr<RenderPass> = Ptr::new(e.mem.u32(e.mem.u32(list + 4)));
    if e.get(pass, RenderPass::ucNumLights) > 0 {
        let light = renderer_light(e);
        let lights = e.get(pass, RenderPass::ppSceneLights);
        e.mem.set_u32(lights, light);
    }
}

/// The threshold test the update repeats for each fade distance (two `FCOM`
/// pairs on `limit` against the new and the previous camera distance, every
/// compare ordered): true when the distance crossed the limit either way.
fn crossed(new: f64, old: f64, limit: f64) -> bool {
    (limit > new && limit <= old) || (new >= limit && limit > old)
}

// Translated from 00bb4540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the constructor of the `010b9910` class: the
/// `BSShaderPPLightingProperty` constructor, the vtable, `object`'s vtable
/// slot +0xCC with `this`, then `iShader` 0x1D and `iShaderPropertyType`
/// 0xC. Returns `this`.
// Translation incomplete: 00bb4540 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b858, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb4540(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    e.call(PP_LIGHTING_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_010B9910);
    e.vcall(object, 0xcc, &args![this]);
    e.mem.set_u32(this.addr() + SHADER, 0x1d);
    e.mem.set_u32(this.addr() + SHADER_PROPERTY_TYPE, 0xc);
    this
}

// Translated from 00bb4650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010b9910` class's scalar deleting destructor: its vtable,
/// the `BSShaderPPLightingProperty` destructor, then the 0x108-byte free when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_00bb4650(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_010B9910);
    e.call(PP_LIGHTING_DESTRUCTOR, &args![this]);
    if flags as u8 & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x108u32]);
    }
    this
}

// Translated from 00bb4680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `010b9910` class's `CreateClone`): with the thread's
/// allocation context at 0x28 (put back at the end), a 0x108-byte object
/// from `00aa13e0` through the `BSShaderPPLightingProperty` constructor,
/// this class's vtable and the pass at +0x100 cleared (0 when the
/// allocation fails), then the base member copy into it (which still runs
/// for a null clone). Returns the clone.
// Translation incomplete: 00bb4680 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b8ca, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb4680(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 0x28);
    let block = e.call(NI_NEW, &args![0x108u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        e.call(PP_LIGHTING_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, VTABLE_010B9910);
        e.mem.set_u32(block + OWN_PASS, 0);
        Ptr::new(block)
    };
    e.call(PP_LIGHTING_COPY_MEMBERS, &args![this, clone, process]);
    e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, saved);
    clone
}

// Translated from 00bb5dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the class's single pass for `geometry`, made once and kept at
/// +0x100: none (0) when bit 26 or 27 of the first flag word is set;
/// otherwise the kept pass, or a new 0x10-byte `RenderPass` from the memory
/// manager for `geometry` (pass 0x1DA with bit 1 of the flags and vertex
/// colours on the geometry data (+0xB8, +0x28), 0x1D9 with bit 1 only,
/// 0x1D8 with colours only, else 0x1D7; first pass, no lights), stored and
/// answered (a failed allocation stores and answers 0).
// Translation incomplete: 00bb5dd0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b91c, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb5dd0(e: &mut Engine, this: Ptr, geometry: u32) -> u32 {
    let flags = e.mem.u32(this.addr() + FLAGS0);
    if flags & 0x0400_0000 != 0 || flags & 0x0800_0000 != 0 {
        return 0;
    }
    let data = e.mem.u32(geometry + 0xb8);
    let colours = e.mem.u32(data + 0x28) != 0;
    if e.mem.u32(this.addr() + OWN_PASS) != 0 {
        return e.mem.u32(this.addr() + OWN_PASS);
    }
    let pass_enum: u32 = match (flags as u8 & 2 != 0, colours) {
        (true, true) => 0x1da,
        (true, false) => 0x1d9,
        (false, true) => 0x1d8,
        (false, false) => 0x1d7,
    };
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
        .u32();
    if block == 0 {
        e.mem.set_u32(this.addr() + OWN_PASS, 0);
        return e.mem.u32(this.addr() + OWN_PASS);
    }
    let pass = e
        .call(
            RENDER_PASS_CONSTRUCTOR,
            &args![block, geometry, pass_enum, 1u32, 0u32, 0u32],
        )
        .u32();
    e.mem.set_u32(this.addr() + OWN_PASS, pass);
    pass
}

// Translated from 00bb5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `BSShaderPPLightingProperty::SetupGeometry` on `geometry`; with
/// bit 19 of the first flag word set the depth bias becomes 1.0; then the
/// geometry's property 10 (`NiAVObject::GetProperty`), when it has one, goes
/// to `00654640` on the geometry. Answers what `SetupGeometry` answered.
pub fn fn_00bb5ff0(e: &mut Engine, this: Ptr, geometry: u32) -> bool {
    let set_up = e
        .call(PP_LIGHTING_SETUP_GEOMETRY, &args![this, geometry])
        .u8();
    if e.mem.u32(this.addr() + FLAGS0) & 0x0008_0000 != 0 {
        e.mem.set_u32(this.addr() + DEPTH_BIAS, 1.0f32.to_bits());
    }
    let property = e.call(GET_PROPERTY, &args![geometry, 0xau32]).u32();
    if property != 0 {
        e.call(USE_PROPERTY_00654640, &args![geometry, property]);
    }
    set_up != 0
}

/// The light refresh of `00bb4740` for an unchanged pass state: each pass
/// of the main list gets its lights again from the current active lights
/// (`word` is the accumulator's light word, `count_word` the stack word whose
/// low byte holds the count the exe passes to `00ba8c00`). Clears the state
/// and stops when a shadow pass finds no usable shadow light.
fn refresh_lights(e: &mut Engine, this: u32, frame: u32, word: u32, count_word: &mut u32) {
    let first_iterator = frame + 0xc;
    let second_iterator = frame + 0x1c;
    let third_iterator = frame + 0x20;
    let list = e.mem.u32(this + PASSES);
    if e.mem.u32(list + 0x10) == 0 {
        return;
    }
    let mut index: u32 = 0;
    loop {
        let list = e.mem.u32(this + PASSES);
        let items = e.mem.u32(list + 4);
        let pass: Ptr<RenderPass> = Ptr::new(e.mem.u32(items + index * 4));
        let pass_enum = e.get(pass, RenderPass::usPassEnum);
        if (0x17f..=0x1ba).contains(&pass_enum) {
            let mut count: u8 = 1;
            *count_word = (*count_word & 0xffff_ff00) | 1;
            let mut light = e
                .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![this, first_iterator])
                .u32();
            let mut skip = false;
            if light != 0 {
                while light != 0 {
                    count = count.wrapping_add(1);
                    light = e
                        .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![this, first_iterator])
                        .u32();
                }
                *count_word = (*count_word & 0xffff_ff00) | u32::from(count);
                if count == 0 {
                    e.set(pass, RenderPass::ucNumLights, 0);
                    skip = true;
                }
            }
            if !skip {
                if e.get(pass, RenderPass::ucMaxNumLights) < count {
                    e.call(PASS_FREE_LIGHTS, &args![pass]);
                    e.call(PASS_ALLOCATE_LIGHTS, &args![pass, *count_word]);
                    e.set(pass, RenderPass::ucMaxNumLights, count);
                }
                let mut light = e
                    .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![this, first_iterator])
                    .u32();
                let lights = e.get(pass, RenderPass::ppSceneLights);
                e.mem.set_u32(lights, word);
                let mut stored: u32 = 1;
                if light != 0 {
                    let limit = *count_word & 0xff;
                    while (stored as i32) < limit as i32 {
                        let lights = e.get(pass, RenderPass::ppSceneLights);
                        e.mem.set_u32(lights + stored * 4, light);
                        stored += 1;
                        light = e
                            .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![this, first_iterator])
                            .u32();
                        if light == 0 {
                            break;
                        }
                    }
                }
                e.set(pass, RenderPass::ucNumLights, stored as u8);
            }
        } else if (0x1bd..=0x1c3).contains(&pass_enum) {
            if e.get(pass, RenderPass::ucNumLights) == 1 {
                let light = e
                    .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![this, second_iterator])
                    .u32();
                let lights = e.get(pass, RenderPass::ppSceneLights);
                e.mem.set_u32(lights, if light != 0 { light } else { word });
            }
        } else if (0x1c4..=0x1c7).contains(&pass_enum) && e.get(pass, RenderPass::ucNumLights) == 1
        {
            let light = e
                .call(FIRST_ACTIVE_LIGHT, &args![this, third_iterator])
                .u32();
            if light == 0
                || e.mem.u8(light + 0xec) == 0
                || e.mem.u8(e.mem.u32(light + 0xf8) + 0x30) & 1 != 0
            {
                e.mem.set_u32(this + LAST_STATE, 0);
                return;
            }
            let lights = e.get(pass, RenderPass::ppSceneLights);
            e.mem.set_u32(lights, light);
        }
        index += 1;
        let list = e.mem.u32(this + PASSES);
        if index >= e.mem.u32(list + 0x10) {
            return;
        }
    }
}

/// The accumulation hint the main and shadow passes get from flags 0x1A,
/// 0x1B and 0x35: `Some(1 or 4)` with 0x1A, `Some(1 or 5)` with 0x1B (1 when
/// 0x35 is set), `None` with neither (the flags read in the exe's order).
fn blend_hint(e: &mut Engine, this: u32) -> Option<u8> {
    if has_flag(e, this, 0x1a) {
        Some(if has_flag(e, this, 0x35) { 1 } else { 4 })
    } else if has_flag(e, this, 0x1b) {
        Some(if has_flag(e, this, 0x35) { 1 } else { 5 })
    } else {
        None
    }
}

/// The main pass enum of `00bb4740`'s rebuild (`None`: the pass keeps the
/// 0 it was added with). The arguments are the flags the exe tests, in the
/// order of its decision tree.
#[allow(clippy::too_many_arguments)]
fn main_pass_enum(
    flag_12: bool,
    specular: bool,
    flag_1: bool,
    flag_0b: bool,
    flag_1c: bool,
    flag_0a: bool,
    has_b4: bool,
    colours: bool,
    shadowed: bool,
) -> Option<u16> {
    let pick = |yes: u16, no: u16| if shadowed { yes } else { no };
    if flag_12 {
        return Some(if specular {
            pick(0x1ae, 0x190)
        } else {
            pick(0x19f, 0x181)
        });
    }
    let value = match (specular, flag_1) {
        (true, false) => {
            if flag_0b {
                match (colours, flag_1c) {
                    (true, true) => pick(0x1b8, 0x19a),
                    (true, false) => pick(0x1b7, 0x199),
                    (false, true) => pick(0x1b0, 0x192),
                    (false, false) => pick(0x1af, 0x191),
                }
            } else if flag_0a {
                pick(0x1b1, 0x193)
            } else if has_b4 {
                if colours {
                    pick(0x1b9, 0x19b)
                } else {
                    pick(0x1b3, 0x195)
                }
            } else if colours {
                pick(0x1b5, 0x197)
            } else {
                pick(0x1ac, 0x18e)
            }
        }
        (true, true) => {
            if flag_0b {
                return None;
            } else if flag_0a {
                pick(0x1b2, 0x194)
            } else if has_b4 {
                if colours {
                    pick(0x1ba, 0x19c)
                } else {
                    pick(0x1b4, 0x196)
                }
            } else if colours {
                pick(0x1b6, 0x198)
            } else {
                pick(0x1ad, 0x18f)
            }
        }
        (false, false) => {
            if flag_0b {
                match (colours, flag_1c) {
                    (true, true) => pick(0x1a9, 0x18b),
                    (true, false) => pick(0x1a8, 0x18a),
                    (false, true) => pick(0x1a1, 0x183),
                    (false, false) => pick(0x1a0, 0x182),
                }
            } else if flag_0a {
                pick(0x1a2, 0x184)
            } else if has_b4 {
                if colours {
                    pick(0x1aa, 0x18c)
                } else {
                    pick(0x1a4, 0x186)
                }
            } else if colours {
                pick(0x1a6, 0x188)
            } else {
                pick(0x19d, 0x17f)
            }
        }
        (false, true) => {
            if flag_0b {
                return None;
            } else if flag_0a {
                pick(0x1a3, 0x185)
            } else if has_b4 {
                if colours {
                    pick(0x1ab, 0x18d)
                } else {
                    pick(0x1a5, 0x187)
                }
            } else if colours {
                pick(0x1a7, 0x189)
            } else {
                pick(0x19e, 0x180)
            }
        }
    };
    Some(value)
}

// Translated from 00bb4740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `010b9910` class's vtable slot 40, where `GetRenderPasses`
/// sits; `thiscall`, `RET 0x18`): the render passes for `geometry`.
///
/// Kinds 7 (depth map, +0x40), 8 (constant alpha, +0x44), 9 (local map,
/// +0x48) and 0xF (+0x54) make their array and add their one pass when it
/// is empty (7 and 8 refresh the first pass's light instead when it has
/// one; 7 answers 0, 8 its array, when bit 15 or 16 of the flags is set).
/// Any other kind uses the main list (+0x3C) under the state `kind << 8 |
/// render_mode`: with one of the fades on, the camera distance is measured
/// again (the bound centre's distance to the accumulator's camera at
/// +0x194 +0x1F0, less the radius, scaled by the accumulator's +8 object's
/// +0x110) and the state cleared when a fade distance was crossed; the
/// alpha is set (the geometry's material alpha times the fade alpha, or
/// the `004b3ab0` fade between `011f946c` and `011f9470`); the lights are
/// resorted when asked; an unchanged state with resorted lights refreshes
/// the passes' lights; a changed one rebuilds the list (the main pass with
/// its lights and its enum from the flags, shadow-light passes, the
/// translucency, +0xDC, and the 0x1BD.. passes) and stores the state.
/// `_unused_3` and `_unused_6` are not read. The 32-bit stack slot of
/// `render_mode` is reused for byte flags and the alpha; its low byte is
/// what the exe passes as the light count to `00ba8c00`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bb4740(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    render_mode: u32,
    _unused_3: u32,
    kind: u32,
    accumulator: u32,
    _unused_6: u32,
) -> u32 {
    let base = this.addr();
    let state = (kind << 8) | render_mode;
    let flags1 = e.mem.u32(base + FLAGS1);
    let wide = flags1 & 0x0080_0000 != 0;
    let properties = geometry.wrapping_add(0x9c);

    if kind == 7 {
        let flags = e.mem.u32(base + FLAGS0);
        if flags & 0x8000 != 0 || flags & 0x0001_0000 != 0 {
            return 0;
        }
        e.call(CHECK_CREATE_DEPTH_MAP_PASSES, &args![base, 1u32]);
        let list = e.mem.u32(base + DEPTH_PASSES);
        if e.mem.u32(list + 0x10) != 0 {
            refresh_first_light(e, list);
            return e.mem.u32(base + DEPTH_PASSES);
        }
        let data = e.mem.u32(geometry + 0xb8);
        let colours = e.mem.u32(data + 0x28) != 0;
        let flag_1 = has_flag(e, base, 1);
        let light = renderer_light(e);
        let pass_enum = match (colours, flag_1) {
            (true, true) => 0x1cb,
            (true, false) => 0x1c9,
            (false, true) => 0x1ca,
            (false, false) => 0x1c8,
        };
        add_pass(e, list, [geometry, pass_enum, 1, 1, light, 0, 0, 0]);
        return e.mem.u32(base + DEPTH_PASSES);
    }
    if kind == 8 {
        let flags = e.mem.u32(base + FLAGS0);
        if flags & 0x8000 != 0 || flags & 0x0001_0000 != 0 || flags1 as u8 & 0x10 != 0 {
            return e.mem.u32(base + CONST_ALPHA_PASSES);
        }
        e.call(CHECK_CREATE_CONST_ALPHA_PASSES, &args![base, 1u32]);
        let list = e.mem.u32(base + CONST_ALPHA_PASSES);
        if e.mem.u32(list + 0x10) != 0 {
            refresh_first_light(e, list);
            return e.mem.u32(base + CONST_ALPHA_PASSES);
        }
        let data = e.mem.u32(geometry + 0xb8);
        let colours = e.mem.u32(data + 0x28) != 0;
        let flag_1 = has_flag(e, base, 1);
        let light = renderer_light(e);
        let pass_enum = match (flag_1, colours) {
            (false, true) => 0x1d4,
            (false, false) => 0x1d3,
            (true, true) => 0x1d6,
            (true, false) => 0x1d5,
        };
        add_pass(e, list, [geometry, pass_enum, 1, 1, light, 0, 0, 0]);
        return e.mem.u32(base + CONST_ALPHA_PASSES);
    }
    if kind == 9 {
        e.call(CHECK_CREATE_LOCAL_MAP_PASSES, &args![base, 1u32]);
        let list = e.mem.u32(base + LOCAL_MAP_PASSES);
        if e.mem.u32(list + 0x10) == 0 {
            let pass_enum = if e.mem.u32(base + FLAGS0) & 0x0010_0000 != 0 {
                0x1df
            } else if has_flag(e, base, 1) {
                0x1de
            } else {
                0x1dd
            };
            add_pass(e, list, [geometry, pass_enum, 1, 0, 0, 0, 0, 0]);
        }
        return e.mem.u32(base + LOCAL_MAP_PASSES);
    }
    if kind == 0xf {
        e.call(CHECK_CREATE_PASSES_54, &args![base, 1u32]);
        let list = e.mem.u32(base + PASSES_54);
        if e.mem.u32(list + 0x10) == 0 && e.mem.u32(geometry + 0xbc) == 0 {
            let pass_enum = if has_flag(e, base, 0x25) {
                0x1bc
            } else {
                0x1bb
            };
            add_pass(e, list, [geometry, pass_enum, 1, 0, 0, 0, 0, 0]);
        }
        return e.mem.u32(base + PASSES_54);
    }

    e.with_stack(0x4c, |e, frame| {
        let frame = frame.addr();
        // The `render_mode` argument's stack slot, reused by the exe.
        let mut slot = render_mode;
        let mut keep_specular = true;
        let mut fade_allowed = true;
        let mut translucent = false;
        let old = x87_load32(x87_quiet_bits(e.mem.u32(base + CAMERA_DISTANCE)));
        let flags = e.mem.u32(base + FLAGS0);
        let fade_80 = flags & 0x80 != 0 && 0.0 < load(e, FADE_FAR_011F9460);
        let fade_20000 = flags & 0x0002_0000 != 0 && 0.0 < load(e, FADE_FAR_011F9468);
        slot = (slot & 0xffff_ff00) | u32::from(fade_20000);
        let lod_on = flags & 0x0400_0000 != 0 || e.call(HAS_FLAG, &args![base, 0x1bu32]).u8() == 1;
        let lod_fade = lod_on && 0.0 < load(e, FADE_FAR_011F9470) && !has_flag(e, base, 0x23);
        let main_fade = flags & 1 != 0 && 0.0 < load(e, FADE_FAR_011F9458);
        let mut below_main = main_fade;

        if fade_20000 || fade_80 || lod_fade || main_fade {
            let camera = e.mem.u32(accumulator + 0x194);
            let mut bound = e.mem.u32(geometry + 0x20);
            if bound == 0 {
                bound = DEFAULT_BOUND;
            }
            // The bound copy at `frame + 0x3c`, the camera at `+0x24`, the
            // difference at `+0x30`.
            for word in 0..3 {
                let value = e.mem.u32(camera + 0x1f0 + 4 * word);
                e.mem.set_u32(frame + 0x24 + 4 * word, value);
            }
            for word in 0..4 {
                let value = e.mem.u32(bound + 4 * word);
                e.mem.set_u32(frame + 0x3c + 4 * word, value);
            }
            for word in 0..3 {
                let difference = x87_sub(
                    load(e, frame + 0x3c + 4 * word),
                    x87_m32(e.mem.u32(frame + 0x24 + 4 * word)),
                );
                e.mem
                    .set_u32(frame + 0x30 + 4 * word, x87_store32(difference));
            }
            let length = e.call(VECTOR_LENGTH, &args![frame + 0x30]).f64();
            let distance = x87_sub(length, x87_m32(e.mem.u32(frame + 0x48)));
            e.mem.set_u32(base + CAMERA_DISTANCE, x87_store32(distance));
            let accumulator_now = e.call(GET_ACCUMULATOR, &[]).u32();
            if e.mem.u32(accumulator_now + 8) != 0 {
                let accumulator_now = e.call(GET_ACCUMULATOR, &[]).u32();
                let object = e.mem.u32(accumulator_now + 8);
                let scaled = x87_mul(
                    load(e, object + 0x110),
                    x87_m32(e.mem.u32(base + CAMERA_DISTANCE)),
                );
                e.mem.set_u32(base + CAMERA_DISTANCE, x87_store32(scaled));
            }
            let new = load(e, base + CAMERA_DISTANCE);
            // `FUCOMP; TEST AH,0x44; JNP`: on unless the distance is unchanged.
            if !(old == new) {
                if fade_20000 {
                    if crossed(new, old, load(e, FADE_FAR_011F9468)) {
                        e.mem.set_u32(base + LAST_STATE, 0);
                    }
                } else if fade_80 && crossed(new, old, load(e, FADE_FAR_011F9460)) {
                    e.mem.set_u32(base + LAST_STATE, 0);
                }
                if lod_fade && crossed(new, old, load(e, FADE_FAR_011F9470)) {
                    e.mem.set_u32(base + LAST_STATE, 0);
                }
                if main_fade && !wide {
                    let limit = load(e, FADE_FAR_011F9458);
                    if crossed(new, old, limit) {
                        e.mem.set_u32(base + LAST_STATE, 0);
                    }
                    if load(e, base + CAMERA_DISTANCE) >= limit {
                        below_main = false;
                    }
                }
            }
        }
        if !wide {
            let limit = load(e, FADE_FAR_011F9460);
            if !(limit == 0.0) && load(e, base + CAMERA_DISTANCE) >= limit {
                keep_specular = false;
            }
        }

        if properties != 0 {
            let material = e.mem.u32(geometry + 0xa4);
            slot = 1.0f32.to_bits();
            if material == 0 {
                slot = x87_quiet_bits(e.mem.u32(base + FADE_ALPHA));
            } else if !lod_fade {
                let alpha = x87_mul(
                    load(e, material + 0x3c),
                    x87_m32(e.mem.u32(base + FADE_ALPHA)),
                );
                slot = x87_store32(alpha);
            } else {
                let far = load(e, FADE_FAR_011F9470);
                let distance = load(e, base + CAMERA_DISTANCE);
                if !(0.0 == far) && distance >= far {
                    fade_allowed = false;
                } else {
                    let near = load(e, FADE_NEAR_011F946C);
                    if !(near == 0.0) && !(0.0 == far) && distance >= near && distance < far {
                        let faded = e
                            .call(
                                INTERPOLATE,
                                &args![
                                    0u32,
                                    1.0f32.to_bits(),
                                    x87_store32(far),
                                    x87_store32(near),
                                    x87_store32(distance)
                                ],
                            )
                            .f64();
                        slot = x87_store32(faded);
                    } else {
                        let alpha = x87_mul(
                            load(e, material + 0x3c),
                            x87_m32(e.mem.u32(base + FADE_ALPHA)),
                        );
                        slot = x87_store32(alpha);
                    }
                }
            }
            e.call(SET_ALPHA, &args![base, x87_quiet_bits(slot)]);
            if 1.0 > x87_m32(slot) {
                translucent = true;
            }
        }

        if e.mem.u8(base + LIGHTS_DIRTY) != 0 {
            let mut bound = e.mem.u32(geometry + 0x20);
            if bound == 0 {
                bound = DEFAULT_BOUND;
            }
            for word in 0..4 {
                let value = e.mem.u32(bound + 4 * word);
                e.mem.set_u32(frame + 0x3c + 4 * word, value);
            }
            e.call(RESORT_LIGHTS, &args![base, frame + 0x3c]);
        }

        if e.mem.u32(base + LAST_STATE) == state && e.mem.u8(base + LIGHTS_DIRTY) != 0 {
            let object = accumulator_object(e);
            let word = e.mem.u32(object + 0xe0);
            refresh_lights(e, base, frame, word, &mut slot);
            e.mem.set_u8(base + LIGHTS_DIRTY, 0);
        }

        if e.mem.u32(base + LAST_STATE) == state {
            return;
        }
        e.call(CHECK_CREATE_RENDER_PASSES, &args![base, u32::MAX]);
        let list = e.mem.u32(base + PASSES);
        e.mem.set_u32(list + 0x10, 0);
        e.mem.set_u8(base + LIGHTS_DIRTY, 0);
        let enabled = e.mem.u32(base + FLAGS0) & 0x0010_0000 == 0
            || e.global::<u8>(PASSES_FORCED_011F91A7) != 0;
        let fade_flags = fade_allowed && (has_flag(e, base, 0x1a) || has_flag(e, base, 0x1b));
        // `FLDZ; FCOMP [alpha]; TEST AH,0x44; JNP`: stop on a zero alpha.
        if 0.0 == x87_m32(e.mem.u32(base + ALPHA)) || !enabled || (lod_fade && !fade_flags) {
            return;
        }
        let flag_1 = has_flag(e, base, 1);
        let flag_7 = keep_specular && has_flag(e, base, 7);
        let flag_11 = has_flag(e, base, 0x11);
        let flag_15 = has_flag(e, base, 0x15);
        let flag_0f = has_flag(e, base, 0xf);
        let flag_10 = has_flag(e, base, 0x10);
        let flag_0b = has_flag(e, base, 0xb);
        let flag_1c = has_flag(e, base, 0x1c);
        let flag_0a = has_flag(e, base, 0xa);
        let has_b4 = e.mem.u32(e.mem.u32(base + WORD_B4)) != 0;
        let specular = has_flag(e, base, 0) && state & 8 != 0 && below_main;
        let alpha_blend = translucent || {
            let property_state = e.mem.u32(geometry + 0x9c);
            e.mem.u8(property_state + 0x18) & 1 != 0
        };
        let mut shadowed = e.call(SETTING_TEST_00B60330, &[]).bool()
            && !has_flag(e, base, 0xd)
            && e.call(SETTING_VALUE_00B60170, &[]).u32() != 0
            && {
                let object = accumulator_object(e);
                e.mem.u8(object + 0x120) != 1
            }
            && {
                let object = accumulator_object(e);
                e.mem.u8(object + 0x120) != 3
            };
        let data = e.mem.u32(geometry + 0xb8);
        let colours = e.mem.u32(data + 0x28) != 0 || has_flag(e, base, 0x25);
        if e.mem.u32(base + OBJECT_DC) != 0 {
            shadowed = false;
        }
        let one = u32::from(flag_1);
        let colour = u32::from(colours);

        if flag_0f {
            let pass_enum = if flag_10 {
                0x1d0
            } else if colours {
                0x1cd + 2 * one
            } else {
                0x1cc + 2 * one
            };
            let list = e.mem.u32(base + PASSES);
            add_pass(e, list, [geometry, pass_enum, 1, 0, 0, 0, 0, 0]);
            let pass = last_pass(e, base);
            e.set(pass, RenderPass::eAccumulationHint, 6);
            e.mem.set_u32(base + LAST_STATE, state);
            return;
        }
        if e.mem.u8(base + FLAGS1) & 0x40 != 0 {
            let list = e.mem.u32(base + PASSES);
            add_pass(e, list, [geometry, 0x1d1 + one, 1, 0, 0, 0, 0, 0]);
            let pass = last_pass(e, base);
            e.set(pass, RenderPass::eAccumulationHint, 7);
        }
        let word = {
            let object = accumulator_object(e);
            e.mem.u32(object + 0xe0)
        };
        let list = e.mem.u32(base + PASSES);
        add_pass(e, list, [geometry, 0, 1, 0, 0, 0, 0, 0]);
        let pass = last_pass(e, base);
        let iterator = frame + 0xc;
        e.set(pass, RenderPass::ucNumLights, 1);
        let mut light = e.call(FIRST_ACTIVE_LIGHT, &args![base, iterator]).u32();
        while light != 0 {
            if e.mem.u8(light + 0xec) == 0 {
                let count = e.get(pass, RenderPass::ucNumLights);
                e.set(pass, RenderPass::ucNumLights, count.wrapping_add(1));
            }
            light = e.call(NEXT_ACTIVE_LIGHT, &args![base, iterator]).u32();
        }
        let count = e.get(pass, RenderPass::ucNumLights);
        if count == 0 {
            e.set(pass, RenderPass::ppSceneLights, 0);
        } else {
            if e.get(pass, RenderPass::ucMaxNumLights) < count {
                e.call(PASS_FREE_LIGHTS, &args![pass]);
                let count = u32::from(e.get(pass, RenderPass::ucNumLights));
                e.call(PASS_ALLOCATE_LIGHTS, &args![pass, count]);
                let count = e.get(pass, RenderPass::ucNumLights);
                e.set(pass, RenderPass::ucMaxNumLights, count);
            }
            let mut light = e.call(FIRST_ACTIVE_LIGHT, &args![base, iterator]).u32();
            let lights = e.get(pass, RenderPass::ppSceneLights);
            e.mem.set_u32(lights, word);
            let mut stored: u8 = 1;
            while light != 0 {
                if e.mem.u8(light + 0xec) == 0 {
                    let lights = e.get(pass, RenderPass::ppSceneLights);
                    e.mem.set_u32(lights + u32::from(stored) * 4, light);
                    stored = stored.wrapping_add(1);
                }
                light = e.call(NEXT_ACTIVE_LIGHT, &args![base, iterator]).u32();
            }
            e.set(pass, RenderPass::ucNumLights, stored);
        }
        e.call(SET_FLAG, &args![base, 0x1eu32, 0u32]);
        let flag_12 = has_flag(e, base, 0x12);
        if let Some(pass_enum) = main_pass_enum(
            flag_12, specular, flag_1, flag_0b, flag_1c, flag_0a, has_b4, colours, shadowed,
        ) {
            e.set(pass, RenderPass::usPassEnum, pass_enum);
        }
        if let Some(hint) = blend_hint(e, base) {
            e.set(pass, RenderPass::eAccumulationHint, hint);
        } else if alpha_blend {
            // `FLD1; FCOMP [fade]; TEST AH,0x41; JZ`: 0xE only when 1 is above.
            let hint = if 1.0 > x87_m32(e.mem.u32(base + FADE_ALPHA)) {
                0xe
            } else {
                1
            };
            e.set(pass, RenderPass::eAccumulationHint, hint);
        } else if e.mem.u32(accumulator + 0x34) == 2
            || has_flag(e, base, 0x22)
            || has_flag(e, base, 0x21)
        {
            e.set(pass, RenderPass::eAccumulationHint, 0xa);
        }

        if e.call(DEPTH_PASSES_WANTED, &[]).bool() {
            let mut light = e.call(FIRST_ACTIVE_LIGHT, &args![base, iterator]).u32();
            while light != 0 {
                if e.mem.u8(light + 0xec) != 0 {
                    let list = e.mem.u32(base + PASSES);
                    add_pass(e, list, [geometry, 0, 0, 1, light, 0, 0, 0]);
                    let shadow = last_pass(e, base);
                    let pass_enum = (if colours { 0x1c5 } else { 0x1c4 }) + 2 * one as u16;
                    e.set(shadow, RenderPass::usPassEnum, pass_enum);
                    if let Some(hint) = blend_hint(e, base) {
                        e.set(shadow, RenderPass::eAccumulationHint, hint);
                    }
                }
                light = e.call(NEXT_ACTIVE_LIGHT, &args![base, iterator]).u32();
            }
        }

        let extra = e.mem.u32(base + OBJECT_DC);
        if extra != 0 && e.mem.u32(extra + 0x6c) != 0 {
            let list = e.mem.u32(base + PASSES);
            add_pass(e, list, [geometry, 0xe + one, 1, 0, 0, 0, 0, 0]);
            let pass = last_pass(e, base);
            e.set(pass, RenderPass::eAccumulationHint, 0xe);
        } else if (has_flag(e, base, 9) || 1.0 > x87_m32(e.mem.u32(base + FADE_ALPHA))) && {
            let property_state = e.mem.u32(geometry + 0x9c);
            property_state == 0 || e.mem.u8(property_state + 0x18) & 1 == 0
        } {
            let pass_enum = if colours { 0xc + one } else { 0xa + one };
            let list = e.mem.u32(base + PASSES);
            add_pass(e, list, [geometry, pass_enum, 1, 0, 0, 0, 0, 0]);
            let pass = last_pass(e, base);
            let hint = if has_flag(e, base, 9) { 1 } else { 0xe };
            e.set(pass, RenderPass::eAccumulationHint, hint);
            if 1.0 > x87_m32(e.mem.u32(base + FADE_ALPHA)) || 1.0 > x87_m32(e.mem.u32(base + ALPHA))
            {
                e.set(pass, RenderPass::eAccumulationHint, 0xe);
            }
        }
        if e.mem.u32(base + OBJECT_DC) != 0 {
            let list = e.mem.u32(base + PASSES);
            add_pass(e, list, [geometry, 0x1e0 + one, 1, 0, 0, 0, 0, 0]);
            let pass = last_pass(e, base);
            e.set(pass, RenderPass::eAccumulationHint, 1);
        }
        if flag_7 {
            let pass = if !flag_15 && flag_11 {
                let light = e.call(FIRST_ACTIVE_LIGHT, &args![base, iterator]).u32();
                let light = if light != 0 { light } else { word };
                let list = e.mem.u32(base + PASSES);
                add_pass(e, list, [geometry, 0, 0, 1, light, 0, 0, 0]);
                let pass = last_pass(e, base);
                e.set(pass, RenderPass::usPassEnum, 0x1c3);
                pass
            } else {
                let pass_enum = if flag_15 {
                    0x1be + 2 * colour
                } else if flag_1 {
                    0x1c1 + colour
                } else {
                    0x1bd + 2 * colour
                };
                let list = e.mem.u32(base + PASSES);
                add_pass(e, list, [geometry, 0, 0, 0, 0, 0, 0, 0]);
                let pass = last_pass(e, base);
                e.set(pass, RenderPass::usPassEnum, pass_enum as u16);
                pass
            };
            if alpha_blend {
                e.set(pass, RenderPass::eAccumulationHint, 1);
            }
        }
        e.mem.set_u32(base + LAST_STATE, state);
    });
    e.mem.u32(base + PASSES)
}

// Translated from 00bb4520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the default constructor of the `010b9910` class (`thiscall`, no stack words):
/// the `BSShaderPPLightingProperty` constructor, the vtable, then the pass at +0x100
/// cleared. Returns `this`.
pub fn fn_00bb4520(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PP_LIGHTING_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_010B9910);
    e.mem.set_u32(this.addr() + OWN_PASS, 0);
    this
}

/// The `BSShaderPPLightingProperty` methods the jumps of this class reach: `00b68dc0`
/// (unnamed, one word), `00b667e0` (unnamed, one word, answers a bool) and
/// `BSShaderPPLightingProperty::PostLinkObject` (`00b68fe0`, one word), and
/// `BSShaderPPLightingProperty::GetViewerStrings` (`00b690d0`, one word).
const PP_LIGHTING_00B68DC0: u32 = 0x00b6_8dc0;
const PP_LIGHTING_00B667E0: u32 = 0x00b6_67e0;
const PP_LIGHTING_POST_LINK_OBJECT: u32 = 0x00b6_8fe0;
const PP_LIGHTING_VIEWER_STRINGS: u32 = 0x00b6_90d0;

// Translated from 00bb44f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entries at `010b837c`, `010b995c`,
/// `010bac44`, `010bcbac`): a jump to `00b68dc0` with the same `this` and word.
pub fn fn_00bb44f0(e: &mut Engine, this: Ptr, word: u32) {
    e.call(PP_LIGHTING_00B68DC0, &args![this, word]);
}

// Translated from 00bb4500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entries at `010b8384`, `010b9964`,
/// `010bac4c`, `010bcbb4`): a jump to `00b667e0` with the same `this` and word; answers its
/// bool.
pub fn fn_00bb4500(e: &mut Engine, this: Ptr, word: u32) -> bool {
    e.call(PP_LIGHTING_00B667E0, &args![this, word]).bool()
}

// Translated from 00bb4510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entries at `010b997c`,
/// `010bcbcc`): a jump to `BSShaderPPLightingProperty::PostLinkObject` with the same `this`
/// and word.
pub fn fn_00bb4510(e: &mut Engine, this: Ptr, word: u32) {
    e.call(PP_LIGHTING_POST_LINK_OBJECT, &args![this, word]);
}

// Translated from 00bb45b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `010b9910` class's destructor, `thiscall`. Stores the vtable
/// and jumps to the `BSShaderPPLightingProperty` destructor (`00b67380`).
pub fn fn_00bb45b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_010B9910);
    e.call(PP_LIGHTING_DESTRUCTOR, &args![this]);
}

/// The class's RTTI record (`012007b0`), answered by `00bb45c0`.
const RTTI_012007B0: u32 = 0x0120_07b0;

// Translated from 00bb45c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entries at `010b9918`,
/// `010bcb68`; `ECX` unread): answers the address `012007b0`.
pub fn fn_00bb45c0(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(RTTI_012007B0)
}

// Translated from 00bb45d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bb45d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
/// Unnamed in the engine map: the creator `RegisterStreamables` registers for this class
/// (`Lighting30ShaderProperty` and `NoFaderShaderProperty`), no arguments. Allocates 0x108 bytes
/// (`00aa13e0`); when that succeeds, the `00bb4520` constructor's body (inlined in the exe)
/// runs on them. Answers the block or null.
pub fn fn_00bb45d0(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x108u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00bb4520(e, Ptr::new(block));
    block
}

/// `BSShaderProperty::CheckCreateRenderPassArray`-like `00ba97b0` (unnamed, `thiscall`, one
/// word) and the pass list at +0x50 it makes.
const CHECK_CREATE_PASSES_50: u32 = 0x00ba_97b0;
const PASSES_50: u32 = 0x50;

// Translated from 00bb5c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entries at `010b99c8`, `010bcc18`), `thiscall` with one
/// word, a geometry. Answers 0 when 0.0 is below the `float` +0xe0 (`FCOMP`, `TEST AH,5`, `JP`;
/// a NaN goes on). Else makes the pass list +0x50 (`00ba97b0(1)`); when it is empty (+0x10),
/// adds one pass for the geometry (`Add(geometry, pass, 1, 0, 0, 0, 0, 0)`): by whether the
/// geometry's +0xb8 block has +0x28 set, bit 9 of the `u16` +0x18 of its +0x9c block (false
/// without one) and bit 1 of the byte +0x20: 0x2ec/0x2ed, 0x2e6/0x2e7 with +0x28; 0x2e0/0x2e1,
/// 0x2da/0x2db without (the second of each pair with bit 1, the first pair of each with bit 9).
/// Answers the pass list.
pub fn fn_00bb5c90(e: &mut Engine, this: Ptr, geometry: u32) -> u32 {
    let base = this.addr();
    let value = x87_m32(e.mem.u32(base + 0xe0));
    if 0.0 < value {
        return 0;
    }
    let block = e.mem.u32(geometry + 0x9c);
    let mut bit9 = false;
    if block != 0 {
        bit9 = (e.mem.u16(block + 0x18) >> 9) & 1 != 0;
    }
    let data = e.mem.u32(geometry + 0xb8);
    let set = e.mem.u32(data + 0x28) != 0;
    e.call(CHECK_CREATE_PASSES_50, &args![base, 1u32]);
    let list = e.mem.u32(base + PASSES_50);
    if e.mem.u32(list + 0x10) != 0 {
        return e.mem.u32(base + PASSES_50);
    }
    let bit1 = e.mem.u8(base + FLAGS0) & 2 != 0;
    let pass: u32 = match (set, bit9, bit1) {
        (true, true, true) => 0x2ed,
        (true, true, false) => 0x2ec,
        (true, false, true) => 0x2e7,
        (true, false, false) => 0x2e6,
        (false, true, true) => 0x2e1,
        (false, true, false) => 0x2e0,
        (false, false, true) => 0x2db,
        (false, false, false) => 0x2da,
    };
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    e.mem.u32(base + PASSES_50)
}

use crate::units::bsshader::bsshaderproperty::{viewer_strings_add, VIEWER_STRING};

// Translated from 00bb5fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entries at `010b9970`, `010bcbc0`): the class's
/// `GetViewerStrings`, `thiscall` with one word, the string array (`this` only passed on in
/// `ECX`). Runs `BSShaderPPLightingProperty::GetViewerStrings` and adds the class name
/// (`00a7d330` of `012007b0`; the inlined `NiTArray` add).
pub fn fn_00bb5fa0(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(PP_LIGHTING_VIEWER_STRINGS, &args![this, strings]);
    let rtti = e.global::<u32>(RTTI_012007B0);
    let string = e.call(VIEWER_STRING, &args![rtti]).u32();
    viewer_strings_add(e, strings, string);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_44f0, fn_00bb44f0(Ptr, u32)),
        entry!(0x00bb_4500, fn_00bb4500(Ptr, u32) -> bool),
        entry!(0x00bb_4510, fn_00bb4510(Ptr, u32)),
        entry!(0x00bb_45b0, fn_00bb45b0(Ptr)),
        entry!(0x00bb_45c0, fn_00bb45c0(Ptr) -> Ptr),
        entry!(0x00bb_45d0, fn_00bb45d0() -> u32),
        entry!(0x00bb_5c90, fn_00bb5c90(Ptr, u32) -> u32),
        entry!(0x00bb_5fa0, fn_00bb5fa0(Ptr, u32)),
        entry!(0x00bb_4520, fn_00bb4520(Ptr) -> Ptr),
        entry!(0x00bb_4540, fn_00bb4540(Ptr, u32) -> Ptr),
        entry!(0x00bb_4650, fn_00bb4650(Ptr, u32) -> Ptr),
        entry!(0x00bb_4680, fn_00bb4680(Ptr, Ptr) -> Ptr),
        entry!(
            0x00bb_4740,
            fn_00bb4740(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00bb_5dd0, fn_00bb5dd0(Ptr, u32) -> u32),
        entry!(0x00bb_5ff0, fn_00bb5ff0(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A light walk: `first` resets the iterator word and answers the first
    /// light, `next` steps it (0 past the end).
    fn light_walk(e: &mut Engine, first: u32, next: u32, lights: Vec<u32>) {
        let head = lights.first().copied().unwrap_or(0);
        e.register_double(first, move |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret {
                eax: head,
                ..Ret::default()
            }
        });
        e.register_double(next, move |e, a| {
            let i = e.mem.u32(a[1]) + 1;
            e.mem.set_u32(a[1], i);
            Ret {
                eax: lights.get(i as usize).copied().unwrap_or(0),
                ..Ret::default()
            }
        });
    }

    fn new_list(e: &mut Engine) -> u32 {
        let list = e.mem.alloc(0x14);
        let items = e.mem.alloc(0x100);
        e.mem.set_u32(list + 4, items);
        list
    }

    fn pass_at(e: &Engine, list: u32, index: u32) -> Ptr<RenderPass> {
        Ptr::new(e.mem.u32(e.mem.u32(list + 4) + 4 * index))
    }

    /// The rig of `00bb4740`: the flag test reads the real flag words, the
    /// pass adder appends a pass (enum, first flag, light count and the four
    /// light words), the accumulator's +0x194 object has the light word
    /// `0xacc0` at +0xE0. Returns (engine, this, geometry, accumulator).
    fn passes_rig() -> (Engine, Ptr, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.register(HAS_FLAG, |e, a| {
            let word = e.mem.u32(a[0] + 0x20 + (a[1] / 32) * 4);
            Ret {
                eax: (word >> (a[1] % 32)) & 1,
                ..Ret::default()
            }
        });
        for check in [
            CHECK_CREATE_RENDER_PASSES,
            CHECK_CREATE_DEPTH_MAP_PASSES,
            CHECK_CREATE_CONST_ALPHA_PASSES,
            CHECK_CREATE_LOCAL_MAP_PASSES,
            CHECK_CREATE_PASSES_54,
            SET_FLAG,
            SET_ALPHA,
            RESORT_LIGHTS,
            PASS_FREE_LIGHTS,
        ] {
            e.register(check, |_, _| Ret::default());
        }
        e.register(PASS_ALLOCATE_LIGHTS, |e, a| {
            let lights = e.mem.alloc(4 * (a[1] & 0xff).max(1));
            e.mem.set_u32(a[0] + 0xc, lights);
            Ret::default()
        });
        e.register(RENDER_PASS_ARRAY_ADD, |e, a| {
            let list = a[0];
            let pass = e.mem.alloc(0x10);
            e.mem.set_u32(pass, a[1]);
            e.mem.set_u16(pass + 4, a[2] as u16);
            e.mem.set_u8(pass + 7, a[3] as u8);
            e.mem.set_u8(pass + 9, a[4] as u8);
            e.mem.set_u8(pass + 0xa, a[4] as u8);
            let lights = e.mem.alloc(0x10);
            for i in 0..4 {
                e.mem.set_u32(lights + 4 * i, a[5 + i as usize]);
            }
            e.mem.set_u32(pass + 0xc, lights);
            let size = e.mem.u32(list + 0x10);
            let items = e.mem.u32(list + 4);
            e.mem.set_u32(items + 4 * size, pass);
            e.mem.set_u32(list + 0x10, size + 1);
            Ret::default()
        });
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object + 0xe0, 0xacc0);
        let accumulator_now = e.mem.alloc(0x200);
        e.mem.set_u32(accumulator_now + 0x194, object);
        e.register_double(GET_ACCUMULATOR, move |_, _| Ret {
            eax: accumulator_now,
            ..Ret::default()
        });
        e.register(VECTOR_LENGTH, |e, a| {
            let (x, y, z) = (
                f64::from(e.mem.f32(a[0])),
                f64::from(e.mem.f32(a[0] + 4)),
                f64::from(e.mem.f32(a[0] + 8)),
            );
            Ret {
                st0: (x * x + y * y + z * z).sqrt(),
                ..Ret::default()
            }
        });
        e.register(DEPTH_PASSES_WANTED, |_, _| Ret::default());
        e.register(SETTING_TEST_00B60330, |_, _| Ret::default());
        e.register(SETTING_VALUE_00B60170, |_, _| Ret::default());
        light_walk(&mut e, FIRST_ACTIVE_LIGHT, NEXT_ACTIVE_LIGHT, vec![]);
        light_walk(
            &mut e,
            FIRST_ACTIVE_NON_SHADOW_LIGHT,
            NEXT_ACTIVE_NON_SHADOW_LIGHT,
            vec![],
        );
        // The renderer's light.
        let renderer = e.mem.alloc(0x10);
        let renderer_object = e.mem.alloc(0x200);
        e.mem.set_u32(renderer + 8, renderer_object);
        e.mem.set_u32(renderer_object + 0x1a0, 0x1a01);
        e.set_global(RENDERER, renderer);

        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        for list in [
            PASSES,
            DEPTH_PASSES,
            CONST_ALPHA_PASSES,
            LOCAL_MAP_PASSES,
            PASSES_54,
        ] {
            let made = new_list(&mut e);
            e.mem.set_u32(this.addr() + list, made);
        }
        let b4 = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + WORD_B4, b4);
        e.mem.set_f32(this.addr() + ALPHA, 1.0);
        e.mem.set_f32(this.addr() + FADE_ALPHA, 1.0);
        let geometry = e.mem.alloc(0xc0);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(geometry + 0xb8, data);
        let property_state = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0x9c, property_state);
        let accumulator = e.mem.alloc(0x200);
        let camera = e.mem.alloc(0x200);
        e.mem.set_u32(accumulator + 0x194, camera);
        e.call_log = Some(vec![]);
        (e, this, geometry, accumulator)
    }

    fn passes(
        e: &mut Engine,
        this: Ptr,
        geometry: u32,
        mode: u32,
        kind: u32,
        accumulator: u32,
    ) -> u32 {
        e.call(
            0x00bb_4740,
            &args![this, geometry, mode, 0u32, kind, accumulator, 0u32],
        )
        .u32()
    }

    #[test]
    fn depth_map_kind_adds_one_pass_or_refreshes_its_light() {
        let (mut e, this, geometry, accumulator) = passes_rig();
        let data = e.mem.u32(geometry + 0xb8);
        e.mem.set_u32(data + 0x28, 0x5000);
        e.mem.set_u32(this.addr() + FLAGS0, 2);
        let list = e.mem.u32(this.addr() + DEPTH_PASSES);
        assert_eq!(passes(&mut e, this, geometry, 1, 7, accumulator), list);
        assert_eq!(
            calls_to(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![list, geometry, 0x1cb, 1, 1, 0x1a01, 0, 0, 0]]
        );
        // Now the list has a pass with one light: it gets the renderer's
        // light again.
        let pass = pass_at(&e, list, 0);
        let lights = e.get(pass, RenderPass::ppSceneLights);
        e.mem.set_u32(lights, 0);
        passes(&mut e, this, geometry, 1, 7, accumulator);
        assert_eq!(e.mem.u32(lights), 0x1a01);
        assert_eq!(calls_to(&e, RENDER_PASS_ARRAY_ADD).len(), 1);
        // Bit 15 of the flags: 0.
        e.mem.set_u32(this.addr() + FLAGS0, 0x8000);
        assert_eq!(passes(&mut e, this, geometry, 1, 7, accumulator), 0);
    }

    #[test]
    fn constant_alpha_kind_answers_its_list() {
        let (mut e, this, geometry, accumulator) = passes_rig();
        let list = e.mem.u32(this.addr() + CONST_ALPHA_PASSES);
        e.mem.set_u32(this.addr() + FLAGS1, 0x10);
        assert_eq!(passes(&mut e, this, geometry, 1, 8, accumulator), list);
        assert!(calls_to(&e, RENDER_PASS_ARRAY_ADD).is_empty());
        e.mem.set_u32(this.addr() + FLAGS1, 0);
        passes(&mut e, this, geometry, 1, 8, accumulator);
        assert_eq!(
            calls_to(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![list, geometry, 0x1d3, 1, 1, 0x1a01, 0, 0, 0]]
        );
    }

    #[test]
    fn local_map_and_kind_15_lists() {
        let (mut e, this, geometry, accumulator) = passes_rig();
        let local = e.mem.u32(this.addr() + LOCAL_MAP_PASSES);
        e.mem.set_u32(this.addr() + FLAGS0, 0x0010_0002);
        assert_eq!(passes(&mut e, this, geometry, 1, 9, accumulator), local);
        assert_eq!(
            calls_to(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![local, geometry, 0x1df, 1, 0, 0, 0, 0, 0]]
        );
        let list = e.mem.u32(this.addr() + PASSES_54);
        e.mem.set_u32(geometry + 0xbc, 1);
        assert_eq!(passes(&mut e, this, geometry, 1, 0xf, accumulator), list);
        assert_eq!(calls_to(&e, RENDER_PASS_ARRAY_ADD).len(), 1);
        e.mem.set_u32(geometry + 0xbc, 0);
        passes(&mut e, this, geometry, 1, 0xf, accumulator);
        assert_eq!(
            calls_to(&e, RENDER_PASS_ARRAY_ADD)[1],
            vec![list, geometry, 0x1bb, 1, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn rebuild_makes_the_main_pass_with_its_lights() {
        let (mut e, this, geometry, accumulator) = passes_rig();
        let light = e.mem.alloc(0x100);
        light_walk(&mut e, FIRST_ACTIVE_LIGHT, NEXT_ACTIVE_LIGHT, vec![light]);
        let list = e.mem.u32(this.addr() + PASSES);
        assert_eq!(passes(&mut e, this, geometry, 2, 0, accumulator), list);
        assert_eq!(e.mem.u32(this.addr() + LAST_STATE), 2);
        assert_eq!(
            calls_to(&e, SET_ALPHA),
            vec![vec![this.addr(), 1.0f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(list + 0x10), 1);
        let pass = pass_at(&e, list, 0);
        assert_eq!(e.get(pass, RenderPass::usPassEnum), 0x17f);
        assert_eq!(e.get(pass, RenderPass::ucNumLights), 2);
        let lights = e.get(pass, RenderPass::ppSceneLights);
        assert_eq!([e.mem.u32(lights), e.mem.u32(lights + 4)], [0xacc0, light]);
        assert_eq!(calls_to(&e, SET_FLAG), vec![vec![this.addr(), 0x1e, 0]]);
        // The same state again: no rebuild.
        passes(&mut e, this, geometry, 2, 0, accumulator);
        assert_eq!(e.mem.u32(list + 0x10), 1);
    }

    #[test]
    fn rebuild_with_flag_15_adds_the_single_pass() {
        let (mut e, this, geometry, accumulator) = passes_rig();
        e.mem
            .set_u32(this.addr() + FLAGS0, (1 << 0xf) | (1 << 0x10));
        let list = e.mem.u32(this.addr() + PASSES);
        passes(&mut e, this, geometry, 3, 0, accumulator);
        assert_eq!(
            calls_to(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![list, geometry, 0x1d0, 1, 0, 0, 0, 0, 0]]
        );
        let pass = pass_at(&e, list, 0);
        assert_eq!(e.get(pass, RenderPass::eAccumulationHint), 6);
        assert_eq!(e.mem.u32(this.addr() + LAST_STATE), 3);
    }

    #[test]
    fn rebuild_stops_on_a_zero_alpha() {
        let (mut e, this, geometry, accumulator) = passes_rig();
        e.mem.set_f32(this.addr() + ALPHA, 0.0);
        passes(&mut e, this, geometry, 2, 0, accumulator);
        assert!(calls_to(&e, RENDER_PASS_ARRAY_ADD).is_empty());
        assert_eq!(e.mem.u32(this.addr() + LAST_STATE), 0);
    }

    #[test]
    fn crossing_a_fade_distance_rebuilds_and_drops_the_specular_pass() {
        let (mut e, this, geometry, accumulator) = passes_rig();
        e.set_global(FADE_FAR_011F9460, 100.0f32);
        e.mem.set_u32(this.addr() + FLAGS0, 0x80);
        e.mem.set_f32(this.addr() + CAMERA_DISTANCE, 50.0);
        e.mem.set_u32(this.addr() + LAST_STATE, 2);
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound, 200.0);
        e.mem.set_u32(geometry + 0x20, bound);
        passes(&mut e, this, geometry, 2, 0, accumulator);
        assert_eq!(e.mem.f32(this.addr() + CAMERA_DISTANCE), 200.0);
        // Rebuilt (the crossing cleared the state), without the flag-7
        // pass: the distance is beyond the second fade.
        assert_eq!(calls_to(&e, RENDER_PASS_ARRAY_ADD).len(), 1);
        assert_eq!(e.mem.u32(this.addr() + LAST_STATE), 2);

        // Within the distance the flag-7 pass is added (no flag 0x15 or
        // 0x11, no flag 1, no colours: 0x1BD).
        let (mut e, this, geometry, accumulator) = passes_rig();
        e.set_global(FADE_FAR_011F9460, 100.0f32);
        e.mem.set_u32(this.addr() + FLAGS0, 1 << 7);
        passes(&mut e, this, geometry, 2, 0, accumulator);
        let list = e.mem.u32(this.addr() + PASSES);
        assert_eq!(e.mem.u32(list + 0x10), 2);
        assert_eq!(e.get(pass_at(&e, list, 1), RenderPass::usPassEnum), 0x1bd);
    }

    #[test]
    fn unchanged_state_refreshes_the_pass_lights() {
        let (mut e, this, geometry, accumulator) = passes_rig();
        light_walk(
            &mut e,
            FIRST_ACTIVE_NON_SHADOW_LIGHT,
            NEXT_ACTIVE_NON_SHADOW_LIGHT,
            vec![0x0002_2000, 0x0003_3000],
        );
        let list = e.mem.u32(this.addr() + PASSES);
        // One lit pass (enum 0x180, room for one light).
        let pass = e.mem.alloc(0x10);
        e.mem.set_u16(pass + 4, 0x180);
        e.mem.set_u8(pass + 9, 1);
        e.mem.set_u8(pass + 0xa, 1);
        let lights = e.mem.alloc(4);
        e.mem.set_u32(pass + 0xc, lights);
        e.mem.set_u32(e.mem.u32(list + 4), pass);
        e.mem.set_u32(list + 0x10, 1);
        e.mem.set_u32(this.addr() + LAST_STATE, 2);
        e.mem.set_u8(this.addr() + LIGHTS_DIRTY, 1);
        passes(&mut e, this, geometry, 2, 0, accumulator);
        let pass: Ptr<RenderPass> = Ptr::new(pass);
        assert_eq!(e.get(pass, RenderPass::ucNumLights), 3);
        assert_eq!(e.get(pass, RenderPass::ucMaxNumLights), 3);
        let lights = e.get(pass, RenderPass::ppSceneLights);
        assert_eq!(
            [
                e.mem.u32(lights),
                e.mem.u32(lights + 4),
                e.mem.u32(lights + 8)
            ],
            [0xacc0, 0x0002_2000, 0x0003_3000]
        );
        // The light count's word: the low byte 3 over the alpha stored there.
        assert_eq!(calls_to(&e, PASS_ALLOCATE_LIGHTS)[0][1] & 0xff, 3);
        assert_eq!(e.mem.u8(this.addr() + LIGHTS_DIRTY), 0);
        assert!(calls_to(&e, RENDER_PASS_ARRAY_ADD).is_empty());
        assert_eq!(calls_to(&e, RESORT_LIGHTS).len(), 1);
    }

    #[test]
    fn a_shadow_pass_without_its_light_clears_the_state() {
        let (mut e, this, geometry, accumulator) = passes_rig();
        let list = e.mem.u32(this.addr() + PASSES);
        let pass = e.mem.alloc(0x10);
        e.mem.set_u16(pass + 4, 0x1c5);
        e.mem.set_u8(pass + 9, 1);
        e.mem.set_u32(e.mem.u32(list + 4), pass);
        e.mem.set_u32(list + 0x10, 1);
        e.mem.set_u32(this.addr() + LAST_STATE, 2);
        e.mem.set_u8(this.addr() + LIGHTS_DIRTY, 1);
        passes(&mut e, this, geometry, 2, 0, accumulator);
        // The state was cleared, so the list was rebuilt.
        assert_eq!(calls_to(&e, RENDER_PASS_ARRAY_ADD).len(), 1);
        assert_eq!(e.mem.u32(this.addr() + LAST_STATE), 2);
    }

    #[test]
    fn main_pass_enum_follows_the_decision_tree() {
        // flag 0x12 wins.
        assert_eq!(
            main_pass_enum(true, true, false, false, false, false, false, false, true),
            Some(0x1ae)
        );
        assert_eq!(
            main_pass_enum(true, false, false, false, false, false, false, false, false),
            Some(0x181)
        );
        // Specular with flag 0xB and flag 1: unchanged.
        assert_eq!(
            main_pass_enum(false, true, true, true, false, false, false, false, false),
            None
        );
        assert_eq!(
            main_pass_enum(false, true, false, true, true, false, false, true, true),
            Some(0x1b8)
        );
        assert_eq!(
            main_pass_enum(false, false, true, false, false, false, true, true, false),
            Some(0x18d)
        );
        assert_eq!(
            main_pass_enum(false, false, false, false, false, true, false, false, true),
            Some(0x1a2)
        );
    }

    #[test]
    fn own_pass_is_made_once() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, _| Ret {
            eax: e.mem.alloc(0x10),
            ..Ret::default()
        });
        e.register(RENDER_PASS_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        let geometry = e.mem.alloc(0xc0);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(data + 0x28, 1);
        e.mem.set_u32(this.addr() + FLAGS0, 2);
        e.call_log = Some(vec![]);
        let pass = e.call(0x00bb_5dd0, &args![this, geometry]).u32();
        assert_eq!(e.mem.u32(this.addr() + OWN_PASS), pass);
        assert_eq!(
            calls_to(&e, RENDER_PASS_CONSTRUCTOR),
            vec![vec![pass, geometry, 0x1da, 1, 0, 0]]
        );
        assert_eq!(e.call(0x00bb_5dd0, &args![this, geometry]).u32(), pass);
        assert_eq!(calls_to(&e, RENDER_PASS_CONSTRUCTOR).len(), 1);
        // Without bit 1 and colours: 0x1D7; bit 26: none.
        e.mem.set_u32(this.addr() + OWN_PASS, 0);
        e.mem.set_u32(this.addr() + FLAGS0, 0);
        e.mem.set_u32(data + 0x28, 0);
        e.call(0x00bb_5dd0, &args![this, geometry]);
        assert_eq!(calls_to(&e, RENDER_PASS_CONSTRUCTOR)[1][2], 0x1d7);
        e.mem.set_u32(this.addr() + FLAGS0, 0x0400_0000);
        assert_eq!(e.call(0x00bb_5dd0, &args![this, geometry]).u32(), 0);
        // A failed allocation stores and answers 0.
        e.mem.set_u32(this.addr() + FLAGS0, 0);
        e.mem.set_u32(this.addr() + OWN_PASS, 0);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00bb_5dd0, &args![this, geometry]).u32(), 0);
    }

    #[test]
    fn setup_geometry_sets_the_depth_bias_and_uses_property_10() {
        let mut e = Engine::new();
        e.register(PP_LIGHTING_SETUP_GEOMETRY, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(GET_PROPERTY, |_, a| Ret {
            eax: if a[1] == 0xa { 0x0077_0000 } else { 0 },
            ..Ret::default()
        });
        e.register(USE_PROPERTY_00654640, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        e.mem.set_u32(this.addr() + FLAGS0, 0x0008_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00bb_5ff0, &args![this, 0x4000u32]).bool());
        assert_eq!(e.mem.f32(this.addr() + DEPTH_BIAS), 1.0);
        assert_eq!(
            calls_to(&e, USE_PROPERTY_00654640),
            vec![vec![0x4000, 0x0077_0000]]
        );
        // No property, no bit 19.
        e.register(GET_PROPERTY, |_, _| Ret::default());
        e.register(PP_LIGHTING_SETUP_GEOMETRY, |_, _| Ret::default());
        e.mem.set_u32(this.addr() + FLAGS0, 0);
        e.mem.set_u32(this.addr() + DEPTH_BIAS, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00bb_5ff0, &args![this, 0x4000u32]).bool());
        assert_eq!(e.mem.u32(this.addr() + DEPTH_BIAS), 0);
        assert!(calls_to(&e, USE_PROPERTY_00654640).is_empty());
    }

    #[test]
    fn constructor_destructor_and_clone() {
        let mut e = Engine::new();
        e.register(PP_LIGHTING_CONSTRUCTOR, |_, _| Ret::default());
        e.register(PP_LIGHTING_DESTRUCTOR, |_, _| Ret::default());
        e.register(PP_LIGHTING_COPY_MEMBERS, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        e.register(0x0040_10cc, |_, _| Ret::default());
        e.put_vtable(0x0090_0000, &[0; 0xd0 / 4]);
        e.mem.set_u32(0x0090_0000 + 0xcc, 0x0040_10cc);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0090_0000);
        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        e.call_log = Some(vec![]);
        e.call(0x00bb_4540, &args![this, object]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_010B9910);
        assert_eq!(e.mem.u32(this.addr() + SHADER), 0x1d);
        assert_eq!(e.mem.u32(this.addr() + SHADER_PROPERTY_TYPE), 0xc);
        assert_eq!(calls_to(&e, 0x0040_10cc), vec![vec![object, this.addr()]]);
        e.call(0x00bb_4650, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x108]]);

        let tls = e.tls();
        e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 7);
        e.register(NI_NEW, |e, a| {
            let tls = e.tls();
            let context = e.mem.u32(tls + TLS_ALLOCATION_CONTEXT);
            assert_eq!(context, 0x28);
            Ret {
                eax: e.mem.alloc(a[0]),
                ..Ret::default()
            }
        });
        let clone = e.call(0x00bb_4680, &args![this, 0x6000u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE_010B9910);
        assert_eq!(e.mem.u32(tls + TLS_ALLOCATION_CONTEXT), 7);
        assert_eq!(
            calls_to(&e, PP_LIGHTING_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x6000]]
        );
    }

    #[test]
    fn default_constructor_sets_the_vtable_and_clears_the_pass() {
        let mut e = Engine::new();
        e.register(PP_LIGHTING_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x108);
        e.mem.set_u32(this + OWN_PASS, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bb_4520, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, PP_LIGHTING_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_010B9910);
        assert_eq!(e.mem.u32(this + OWN_PASS), 0);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::bsshaderproperty::{STRINGS_SET_AT, STRINGS_SET_SIZE};
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::rig;
    use crate::units::bsshader::shadowlightshader::n071_rig::calls;

    #[test]
    fn the_jumps_pass_this_and_the_word_on() {
        let mut e = rig();
        e.register(PP_LIGHTING_00B68DC0, |_, _| Ret::default());
        e.register(PP_LIGHTING_00B667E0, |_, a| (a[1] != 0).into_ret());
        e.register(PP_LIGHTING_POST_LINK_OBJECT, |_, _| Ret::default());
        e.call(0x00bb_44f0, &args![1u32, 2u32]);
        assert!(e.call(0x00bb_4500, &args![3u32, 4u32]).bool());
        assert!(!e.call(0x00bb_4500, &args![3u32, 0u32]).bool());
        e.call(0x00bb_4510, &args![5u32, 6u32]);
        assert_eq!(calls(&e, PP_LIGHTING_00B68DC0), vec![vec![1, 2]]);
        assert_eq!(
            calls(&e, PP_LIGHTING_00B667E0),
            vec![vec![3, 4], vec![3, 0]]
        );
        assert_eq!(calls(&e, PP_LIGHTING_POST_LINK_OBJECT), vec![vec![5, 6]]);
        assert_eq!(e.call(0x00bb_45c0, &args![0u32]).u32(), RTTI_012007B0);
    }

    #[test]
    fn fn_00bb45b0_and_00bb45d0_destroy_and_create() {
        let mut e = rig();
        e.register(PP_LIGHTING_DESTRUCTOR, |_, _| Ret::default());
        e.register(PP_LIGHTING_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let made = e.call(0x00bb_45d0, &args![]).u32();
        assert_eq!(calls(&e, NI_NEW), vec![vec![0x108]]);
        assert_eq!(calls(&e, PP_LIGHTING_CONSTRUCTOR), vec![vec![made]]);
        assert_eq!(e.mem.u32(made), VTABLE_010B9910);
        assert_eq!(e.mem.u32(made + OWN_PASS), 0);
        e.mem.set_u32(made, 0);
        e.call(0x00bb_45b0, &args![made]);
        assert_eq!(e.mem.u32(made), VTABLE_010B9910);
        assert_eq!(calls(&e, PP_LIGHTING_DESTRUCTOR), vec![vec![made]]);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00bb_45d0, &args![]).u32(), 0);
    }

    #[test]
    fn fn_00bb5c90_picks_the_pass() {
        let mut e = rig();
        e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
        e.register(CHECK_CREATE_PASSES_50, |_, _| Ret::default());
        let this = e.mem.alloc(0x108);
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(this + PASSES_50, list);
        let geometry = e.mem.alloc(0xc0);
        let data = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0xb8, data);
        // A positive +0xe0 answers 0.
        e.mem.set_f32(this + 0xe0, 1.0);
        assert_eq!(e.call(0x00bb_5c90, &args![this, geometry]).u32(), 0);
        assert!(calls(&e, CHECK_CREATE_PASSES_50).is_empty());
        e.mem.set_f32(this + 0xe0, 0.0);
        assert_eq!(e.call(0x00bb_5c90, &args![this, geometry]).u32(), list);
        assert_eq!(calls(&e, CHECK_CREATE_PASSES_50), vec![vec![this, 1]]);
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0x9c, block);
        e.mem.set_u16(block + 0x18, 0x200);
        e.mem.set_u32(data + 0x28, 1);
        e.mem.set_u8(this + FLAGS0, 2);
        e.call(0x00bb_5c90, &args![this, geometry]);
        e.mem.set_u16(block + 0x18, 0);
        e.mem.set_u8(this + FLAGS0, 0);
        e.call(0x00bb_5c90, &args![this, geometry]);
        e.mem.set_u32(this + 0xe0, 0x7fc0_0000);
        e.mem.set_u32(data + 0x28, 0);
        e.mem.set_u16(block + 0x18, 0x200);
        e.call(0x00bb_5c90, &args![this, geometry]);
        let passes: Vec<u32> = calls(&e, RENDER_PASS_ARRAY_ADD)
            .iter()
            .map(|a| a[2])
            .collect();
        assert_eq!(passes, vec![0x2da, 0x2ed, 0x2e6, 0x2e0]);
        assert_eq!(
            calls(&e, RENDER_PASS_ARRAY_ADD)[0],
            vec![list, geometry, 0x2da, 1, 0, 0, 0, 0, 0]
        );
        // A list with passes is answered as it is.
        e.mem.set_u32(list + 0x10, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bb_5c90, &args![this, geometry]).u32(), list);
        assert!(calls(&e, RENDER_PASS_ARRAY_ADD).is_empty());
    }

    #[test]
    fn fn_00bb5fa0_adds_the_class_name() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.set_global(RTTI_012007B0, 0x7b0u32);
        e.register(PP_LIGHTING_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, _| 0x5150u32.into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let word = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x20, word);
            Ret::default()
        });
        let strings = e.mem.alloc(0x40);
        e.call(0x00bb_5fa0, &args![0x1234u32, strings]);
        assert_eq!(
            calls(&e, PP_LIGHTING_VIEWER_STRINGS),
            vec![vec![0x1234, strings]]
        );
        assert_eq!(calls(&e, VIEWER_STRING), vec![vec![0x7b0]]);
        assert_eq!(calls(&e, STRINGS_SET_SIZE), vec![vec![strings, 0]]);
        assert_eq!(e.mem.u32(strings + 0x20), 0x5150);
    }
}
