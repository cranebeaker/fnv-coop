//! `BSShader/bsshaderpplightingproperty_passes_2x.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSShaderProperty::RenderPassArray::Add` (`00ba9ee0`, `thiscall` on a
/// pass list, eight words: geometry, pass enum, first-pass flag, light
/// count, four lights).
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// `BSShaderProperty::pRenderPassList` (Xbox PDB) +0x3C and
/// `pDepthMapRenderPassList` +0x40.
const PASSES: u32 = 0x3c;
const DEPTH_PASSES: u32 = 0x40;
/// `BSShaderPPLightingProperty::AddTexturePass_1x` (`00bdac00`, `thiscall`,
/// eleven words).
const ADD_TEXTURE_PASS_1X: u32 = 0x00bd_ac00;
/// `004a2020` (unnamed, `thiscall` on a shader property, one word): whether
/// its flag `n` is set (a byte).
const HAS_FLAG: u32 = 0x004a_2020;
/// The two colour compares of `fallout shared/multiboundmarkerdata.cpp`
/// (`004390c0` and `00439090`, `thiscall` on a colour, one word: the other
/// colour; a byte).
const COLOUR_COMPARE_004390C0: u32 = 0x0043_90c0;
const COLOUR_COMPARE_00439090: u32 = 0x0043_9090;
/// The black colour (`011f4998`: three `float`s) the light colours are
/// compared with, and the byte at `011ad80d` that enables the material
/// variants.
const BLACK: u32 = 0x011f_4998;
const MATERIAL_VARIANTS_011AD80D: u32 = 0x011a_d80d;
/// The byte at `011f94ab` that enables the landscape's extra pass.
const LANDSCAPE_EXTRA_011F94AB: u32 = 0x011f_94ab;
/// `BSShaderLightingProperty`'s `GetNumberofActiveNonShadowLights`
/// (`00b707d0`, no words, a byte), `GetFirstActiveNonShadowLight`
/// (`00b70600`) and `GetNextActiveNonShadowLight` (`00b70700`), with the
/// iterator's address.
const ACTIVE_NON_SHADOW_LIGHT_COUNT: u32 = 0x00b7_07d0;
const FIRST_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0600;
const NEXT_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0700;
/// The pass's light-array helpers (`00ba8c30` frees, `00ba8c00` allocates
/// for a count).
const PASS_FREE_LIGHTS: u32 = 0x00ba_8c30;
const PASS_ALLOCATE_LIGHTS: u32 = 0x00ba_8c00;

/// What a pass table chose: a pass enum to add, the skipped-pass count to
/// raise, or neither (the exe's jump past the count).
enum Pick {
    Pass(u32),
    Count,
    Skip,
}

/// A table leaf: the pass when the pass word's low byte is 1, else counted.
fn leaf(one: u32, pass: u32) -> Pick {
    if one as u8 == 1 {
        Pick::Pass(pass)
    } else {
        Pick::Count
    }
}

/// `RenderPassArray::Add` on the property's main list.
fn add_pass(e: &mut Engine, this: Ptr, words: [u32; 8]) {
    let list = e.mem.u32(this.addr() + PASSES);
    let mut call = vec![list];
    call.extend_from_slice(&words);
    e.call(RENDER_PASS_ARRAY_ADD, &call);
}

/// The tail the table functions share: the chosen pass added (with the
/// words `words` makes of its enum), or the `u16` at `count` raised; then
/// the byte at `flag` cleared. Answers 0.
fn finish(
    e: &mut Engine,
    this: Ptr,
    pick: Pick,
    words: impl FnOnce(u32) -> [u32; 8],
    count: u32,
    flag: u32,
) -> u32 {
    match pick {
        Pick::Pass(pass) => add_pass(e, this, words(pass)),
        Pick::Count => {
            let value = e.mem.u16(count);
            e.mem.set_u16(count, value.wrapping_add(1));
        }
        Pick::Skip => {}
    }
    e.mem.set_u8(flag, 0);
    0
}

/// The byte argument as the exe tests it (`CMP byte ptr [..], 0`).
fn on(word: u32) -> bool {
    word as u8 != 0
}

/// The last pass of the main list.
fn last_pass(e: &Engine, this: Ptr) -> u32 {
    let list = e.mem.u32(this.addr() + PASSES);
    let size = e.mem.u32(list + 0x10);
    let items = e.mem.u32(list + 4);
    e.mem
        .u32(items.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4))
}

/// Whether a light's colour (+0xF8 object, +0xD4) is the black at
/// `011f4998` (`FUCOMPP` on each component, `TEST AH,0x44`: equal).
fn is_black(e: &Engine, light: u32) -> bool {
    let colour = e.mem.u32(light + 0xf8) + 0xd4;
    (0..3).all(|i| {
        x87_load32(e.global::<u32>(BLACK + 4 * i)) == x87_load32(e.mem.u32(colour + 4 * i))
    })
}

/// The material test the variant passes share: the variants enabled
/// (`011ad80d`), the geometry's material (+0xA4) absent or its colour (+0x34,
/// or the material's own colour at +0x28) equal to black (`004390c0`).
fn material_is_black(e: &mut Engine, geometry: u32) -> bool {
    if e.global::<u8>(MATERIAL_VARIANTS_011AD80D) == 0 {
        return false;
    }
    let material = e.mem.u32(geometry + 0xa4);
    if material == 0 {
        return true;
    }
    let mut colour = e.mem.u32(material + 0x34);
    if colour == 0 {
        colour = material + 0x28;
    }
    e.call(COLOUR_COMPARE_004390C0, &args![colour, BLACK])
        .bool()
}

/// `1.0 > value` as `FLD1; FCOMP [value]; TEST AH,0x41; JZ` tests it.
fn below_one(e: &Engine, addr: u32) -> bool {
    1.0 > x87_m32(e.mem.u32(addr))
}

// Translated from 00bdbf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddEnvmapPass_2x` (Xbox PDB): `thiscall`,
/// nine words. With the pass word `one` at 1, adds pass 0x249 (`toggle`
/// set), 0x24B (`second` set), 0x24A (`third` set: no light) or 0x248, with
/// the byte at `first_pass` as the first-pass flag and one light
/// (`light_override`, or `light` when that is 0); otherwise raises the `u16`
/// at `count`. Answers 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_envmap_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light: u32,
    light_override: u32,
    count: u32,
    one: u32,
    first_pass: u32,
    toggle: u32,
    second: u32,
    third: u32,
) -> u32 {
    let chosen = if light_override != 0 {
        light_override
    } else {
        light
    };
    if one as u8 != 1 {
        let value = e.mem.u16(count);
        e.mem.set_u16(count, value.wrapping_add(1));
        return 0;
    }
    let first = u32::from(e.mem.u8(first_pass));
    let words = if on(toggle) {
        [geometry, 0x249, first, 1, chosen, 0, 0, 0]
    } else if on(second) {
        [geometry, 0x24b, first, 1, chosen, 0, 0, 0]
    } else if on(third) {
        [geometry, 0x24a, first, 0, 0, 0, 0, 0]
    } else {
        [geometry, 0x248, first, 1, chosen, 0, 0, 0]
    };
    add_pass(e, this, words);
    0
}

// Translated from 00bdc530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map unit `bsshaderpplightingproperty_passes_2x.cpp`):
/// `thiscall`, fourteen words. A pass table of three lights (passes 0xE9 to
/// 0xFA, first pass, `(light0, light1, light2)`) chosen by the byte flags
/// `f8` to `f14` (`f11` is not read); with `one` not 1, or a combination
/// without a pass, the `u16` at `count` is raised (some combinations skip
/// both). Clears the byte at `flag`; answers 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bdc530(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light0: u32,
    light1: u32,
    light2: u32,
    count: u32,
    one: u32,
    flag: u32,
    f8: u32,
    f9: u32,
    f10: u32,
    _unused_11: u32,
    f12: u32,
    f13: u32,
    f14: u32,
) -> u32 {
    let l = |pass| leaf(one, pass);
    let pick = if !on(f8) {
        if !on(f9) {
            if !on(f10) {
                if !on(f13) {
                    if !on(f12) {
                        if !on(f14) {
                            l(0xe9)
                        } else {
                            l(0xf1)
                        }
                    } else if !on(f14) {
                        l(0xf2)
                    } else {
                        l(0xfa)
                    }
                } else if !on(f12) {
                    l(0xeb)
                } else {
                    l(0xf4)
                }
            } else if !on(f12) {
                l(0xed)
            } else {
                l(0xf6)
            }
        } else if on(f10) {
            Pick::Skip
        } else if !on(f13) {
            if !on(f12) {
                l(0xea)
            } else {
                l(0xf3)
            }
        } else if !on(f12) {
            l(0xec)
        } else {
            l(0xf5)
        }
    } else if !on(f9) {
        if !on(f10) {
            if on(f13) {
                Pick::Skip
            } else if !on(f12) {
                l(0xee)
            } else {
                l(0xf7)
            }
        } else if !on(f12) {
            l(0xf0)
        } else {
            l(0xf9)
        }
    } else if on(f10) || on(f13) {
        Pick::Skip
    } else if !on(f12) {
        l(0xef)
    } else {
        l(0xf8)
    };
    finish(
        e,
        this,
        pick,
        |pass| [geometry, pass, 1, 3, light0, light1, light2, 0],
        count,
        flag,
    )
}

// Translated from 00bdca60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddAmbDiffTex2Pass_2x` (Xbox PDB):
/// `thiscall`, fourteen words. A pass table of two lights (passes 0x116 to
/// 0x12B, first pass, `(light0, light1)`) chosen by the byte flags `f7`,
/// `f8`, `f9`, `f11` to `f14` (`f10` is not read); otherwise the `u16` at
/// `count` is raised (some combinations skip both). Clears the byte at
/// `flag`; answers 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_amb_diff_tex2_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light0: u32,
    light1: u32,
    count: u32,
    one: u32,
    flag: u32,
    f7: u32,
    f8: u32,
    f9: u32,
    _unused_10: u32,
    f11: u32,
    f12: u32,
    f13: u32,
    f14: u32,
) -> u32 {
    let l = |pass| leaf(one, pass);
    let pick = if !on(f7) {
        if !on(f8) {
            if !on(f9) {
                if !on(f12) {
                    if !on(f11) {
                        if !on(f13) {
                            if !on(f14) {
                                l(0x116)
                            } else {
                                l(0x11b)
                            }
                        } else {
                            l(0x11f)
                        }
                    } else if !on(f13) {
                        if !on(f14) {
                            l(0x121)
                        } else {
                            l(0x126)
                        }
                    } else {
                        l(0x12a)
                    }
                } else if !on(f11) {
                    l(0x119)
                } else {
                    l(0x124)
                }
            } else if !on(f11) {
                l(0x118)
            } else {
                l(0x123)
            }
        } else if on(f9) {
            Pick::Skip
        } else if !on(f12) {
            if !on(f11) {
                l(0x117)
            } else {
                l(0x122)
            }
        } else if !on(f11) {
            l(0x11a)
        } else {
            l(0x125)
        }
    } else if !on(f8) {
        if !on(f9) {
            if on(f12) {
                Pick::Skip
            } else if !on(f11) {
                if !on(f14) {
                    l(0x11c)
                } else {
                    l(0x120)
                }
            } else if !on(f14) {
                l(0x127)
            } else {
                l(0x12b)
            }
        } else if !on(f11) {
            l(0x11e)
        } else {
            l(0x129)
        }
    } else if on(f9) || on(f12) {
        Pick::Skip
    } else if !on(f11) {
        l(0x11d)
    } else {
        l(0x128)
    };
    finish(
        e,
        this,
        pick,
        |pass| [geometry, pass, 1, 2, light0, light1, 0, 0],
        count,
        flag,
    )
}

// Translated from 00bdd050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddAmbDiffTexSpecularPass_2x` (Xbox PDB):
/// `thiscall`, thirteen words. A pass table of one light (passes 0x95 to
/// 0xAA, first pass) chosen by the byte flags `f6`, `f7`, `f8`, `f10` to
/// `f13` (`f9` is not read); otherwise the `u16` at `count` is raised (some
/// combinations skip both). Clears the byte at `flag`; answers 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_amb_diff_tex_specular_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light: u32,
    count: u32,
    one: u32,
    flag: u32,
    f6: u32,
    f7: u32,
    f8: u32,
    _unused_9: u32,
    f10: u32,
    f11: u32,
    f12: u32,
    f13: u32,
) -> u32 {
    let l = |pass| leaf(one, pass);
    let pick = if !on(f6) {
        if on(f8) {
            Pick::Skip
        } else if !on(f7) {
            if !on(f11) {
                if !on(f10) {
                    if !on(f12) {
                        if !on(f13) {
                            l(0x95)
                        } else {
                            l(0x97)
                        }
                    } else {
                        l(0x9f)
                    }
                } else if !on(f12) {
                    if !on(f13) {
                        l(0xa0)
                    } else {
                        l(0xa2)
                    }
                } else {
                    l(0xaa)
                }
            } else if !on(f10) {
                l(0x99)
            } else {
                l(0xa4)
            }
        } else if !on(f11) {
            if !on(f10) {
                l(0x96)
            } else {
                l(0xa1)
            }
        } else if !on(f10) {
            l(0x9a)
        } else {
            l(0xa5)
        }
    } else if on(f8) || on(f11) {
        Pick::Skip
    } else if !on(f7) {
        if !on(f10) {
            if !on(f13) {
                l(0x9b)
            } else {
                l(0x9d)
            }
        } else if !on(f13) {
            l(0xa6)
        } else {
            l(0xa8)
        }
    } else if !on(f10) {
        l(0x9c)
    } else {
        l(0xa7)
    };
    finish(
        e,
        this,
        pick,
        |pass| [geometry, pass, 1, 1, light, 0, 0, 0],
        count,
        flag,
    )
}

// Translated from 00bdd520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddAmbDiffTexSpecular2Pass_2x` (Xbox PDB):
/// `thiscall`, fourteen words. A pass table (passes 0xAB to 0xC0, first
/// pass; two lights `(light0, light1)`, one light for 0xB0 and 0xBB) chosen
/// by the byte flags `f7`, `f8`, `f9`, `f11` to `f14` (`f10` is not read);
/// otherwise the `u16` at `count` is raised (some combinations skip both).
/// Clears the byte at `flag`; answers 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_amb_diff_tex_specular2_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light0: u32,
    light1: u32,
    count: u32,
    one: u32,
    flag: u32,
    f7: u32,
    f8: u32,
    f9: u32,
    _unused_10: u32,
    f11: u32,
    f12: u32,
    f13: u32,
    f14: u32,
) -> u32 {
    let l = |pass| leaf(one, pass);
    let pick = if !on(f7) {
        if on(f9) {
            Pick::Skip
        } else if !on(f8) {
            if !on(f12) {
                if !on(f11) {
                    if !on(f13) {
                        if !on(f14) {
                            l(0xab)
                        } else {
                            l(0xad)
                        }
                    } else {
                        l(0xb5)
                    }
                } else if !on(f13) {
                    if !on(f14) {
                        l(0xb6)
                    } else {
                        l(0xb8)
                    }
                } else {
                    l(0xc0)
                }
            } else if !on(f11) {
                l(0xaf)
            } else {
                l(0xba)
            }
        } else if !on(f12) {
            if !on(f11) {
                l(0xac)
            } else {
                l(0xb7)
            }
        } else if !on(f11) {
            l(0xb0)
        } else {
            l(0xbb)
        }
    } else if on(f9) || on(f12) {
        Pick::Skip
    } else if !on(f8) {
        if !on(f11) {
            if !on(f14) {
                l(0xb1)
            } else {
                l(0xb3)
            }
        } else if !on(f14) {
            l(0xbc)
        } else {
            l(0xbe)
        }
    } else if !on(f11) {
        l(0xb2)
    } else {
        l(0xbd)
    };
    finish(
        e,
        this,
        pick,
        |pass| {
            if pass == 0xb0 || pass == 0xbb {
                [geometry, pass, 1, 1, light0, 0, 0, 0]
            } else {
                [geometry, pass, 1, 2, light0, light1, 0, 0]
            }
        },
        count,
        flag,
    )
}

// Translated from 00bdda20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddDiffusePt2Pass_2x` (Xbox PDB): `thiscall`,
/// eleven words. A pass table of two lights (passes 0x132 to 0x137, the byte
/// at `flag` as the first-pass flag) chosen by the byte flags `f7`, `f8`,
/// `f10`, `f11` (`f9` is not read); otherwise the `u16` at `count` is raised
/// (one combination skips both). Clears the byte at `flag`; answers 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_diffuse_pt2_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light0: u32,
    light1: u32,
    count: u32,
    one: u32,
    flag: u32,
    f7: u32,
    f8: u32,
    _unused_9: u32,
    f10: u32,
    f11: u32,
) -> u32 {
    let l = |pass| leaf(one, pass);
    let pick = if !on(f7) {
        if !on(f8) {
            if !on(f10) {
                if !on(f11) {
                    l(0x132)
                } else {
                    l(0x137)
                }
            } else {
                l(0x134)
            }
        } else {
            l(0x133)
        }
    } else if !on(f8) {
        if on(f10) {
            Pick::Skip
        } else {
            l(0x135)
        }
    } else {
        l(0x136)
    };
    let first = u32::from(e.mem.u8(flag));
    finish(
        e,
        this,
        pick,
        |pass| [geometry, pass, first, 2, light0, light1, 0, 0],
        count,
        flag,
    )
}

// Translated from 00bddbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddDiffusePt3Pass_2x` (Xbox PDB): `thiscall`,
/// twelve words. A pass table of three lights (passes 0x13E to 0x143, the
/// byte at `flag` as the first-pass flag) chosen by the byte flags `f8`,
/// `f9`, `f11`, `f12` (`f10` is not read); otherwise the `u16` at `count` is
/// raised (one combination skips both). Clears the byte at `flag`; answers
/// 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_diffuse_pt3_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light0: u32,
    light1: u32,
    light2: u32,
    count: u32,
    one: u32,
    flag: u32,
    f8: u32,
    f9: u32,
    _unused_10: u32,
    f11: u32,
    f12: u32,
) -> u32 {
    let l = |pass| leaf(one, pass);
    let pick = if !on(f8) {
        if !on(f9) {
            if !on(f11) {
                if !on(f12) {
                    l(0x13e)
                } else {
                    l(0x143)
                }
            } else {
                l(0x140)
            }
        } else {
            l(0x13f)
        }
    } else if !on(f9) {
        if on(f11) {
            Pick::Skip
        } else {
            l(0x141)
        }
    } else {
        l(0x142)
    };
    let first = u32::from(e.mem.u8(flag));
    finish(
        e,
        this,
        pick,
        |pass| [geometry, pass, first, 3, light0, light1, light2, 0],
        count,
        flag,
    )
}

// Translated from 00bddd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddTexturePass_2x` (Xbox PDB): `thiscall`,
/// thirteen words. Without `use_2x` the 1x version
/// (`AddTexturePass_1x`, `00bdac00`) gets the first ten words and the
/// thirteenth and answers. Otherwise, unless `skip` is set: with the pass
/// word `one` at 1 pass 0x153 is added (the byte at `first_pass` as the
/// first-pass flag, no lights), else the `u16` at `count` is raised.
/// Answers 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_texture_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    word2: u32,
    count: u32,
    one: u32,
    first_pass: u32,
    skip: u32,
    word7: u32,
    word8: u32,
    word9: u32,
    word10: u32,
    _unused_11: u32,
    use_2x: u32,
    word13: u32,
) -> u32 {
    if !on(use_2x) {
        return e
            .call(
                ADD_TEXTURE_PASS_1X,
                &args![
                    this, geometry, word2, count, one, first_pass, skip, word7, word8, word9,
                    word10, word13
                ],
            )
            .u32();
    }
    if on(skip) {
        return 0;
    }
    if one as u8 != 1 {
        let value = e.mem.u16(count);
        e.mem.set_u16(count, value.wrapping_add(1));
        return 0;
    }
    let first = u32::from(e.mem.u8(first_pass));
    add_pass(e, this, [geometry, 0x153, first, 0, 0, 0, 0, 0]);
    0
}

// Translated from 00bdde10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddSpecularPt2Pass_2x` (Xbox PDB):
/// `thiscall`, ten words. A pass table of two lights (passes 0x16C to
/// 0x171, the byte at `flag` as the first-pass flag) chosen by the byte
/// flags `f7` to `f10`; otherwise the `u16` at `count` is raised (one
/// combination skips both). Clears the byte at `flag`; answers 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_specular_pt2_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light0: u32,
    light1: u32,
    count: u32,
    one: u32,
    flag: u32,
    f7: u32,
    f8: u32,
    f9: u32,
    f10: u32,
) -> u32 {
    let l = |pass| leaf(one, pass);
    let pick = if !on(f7) {
        if !on(f8) {
            if !on(f9) {
                if !on(f10) {
                    l(0x16c)
                } else {
                    l(0x16d)
                }
            } else {
                l(0x171)
            }
        } else {
            l(0x16e)
        }
    } else if on(f8) {
        Pick::Skip
    } else if !on(f10) {
        l(0x16f)
    } else {
        l(0x170)
    };
    let first = u32::from(e.mem.u8(flag));
    finish(
        e,
        this,
        pick,
        |pass| [geometry, pass, first, 2, light0, light1, 0, 0],
        count,
        flag,
    )
}

// Translated from 00bddfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddSpecularPt3Pass_2x` (Xbox PDB):
/// `thiscall`, eleven words. A pass table of three lights (passes 0x172 to
/// 0x177, the byte at `flag` as the first-pass flag) chosen by the byte
/// flags `f8` to `f11`; otherwise the `u16` at `count` is raised (one
/// combination skips both). Clears the byte at `flag`; answers 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_specular_pt3_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light0: u32,
    light1: u32,
    light2: u32,
    count: u32,
    one: u32,
    flag: u32,
    f8: u32,
    f9: u32,
    f10: u32,
    f11: u32,
) -> u32 {
    let l = |pass| leaf(one, pass);
    let pick = if !on(f8) {
        if !on(f9) {
            if !on(f10) {
                if !on(f11) {
                    l(0x172)
                } else {
                    l(0x173)
                }
            } else {
                l(0x177)
            }
        } else {
            l(0x174)
        }
    } else if on(f9) {
        Pick::Skip
    } else if !on(f11) {
        l(0x175)
    } else {
        l(0x176)
    };
    let first = u32::from(e.mem.u8(flag));
    finish(
        e,
        this,
        pick,
        |pass| [geometry, pass, first, 3, light0, light1, light2, 0],
        count,
        flag,
    )
}

// Translated from 00bde1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddAmbDiffTexPass_2x` (Xbox PDB):
/// `thiscall`, sixteen words. A pass table of one light (passes 0xFB to
/// 0x115 and 0x235, first pass) chosen by the byte flags `f6`, `f7`, `f8`,
/// `f10` to `f15` (`f9` is not read). Two leaves depend on the geometry's
/// material and shader (+0xA4, +0xA8): 0xFC/0xFD (by the shader's flag
/// 0x22) instead of 0xFB when the material is black (`011ad80d` set), the
/// shader's alpha (+0x28) is not below 1 and its flag 0x25 is set; 0x102
/// instead of 0x101 when the material is black, flag 0x25 is set and its
/// fade alpha (+0x2C) is not below 1. Otherwise the `u16` at `count` is
/// raised (some combinations skip both). Clears the byte at `flag`;
/// answers 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_amb_diff_tex_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light: u32,
    count: u32,
    one: u32,
    flag: u32,
    f6: u32,
    f7: u32,
    f8: u32,
    _unused_9: u32,
    f10: u32,
    f11: u32,
    f12: u32,
    f13: u32,
    f14: u32,
    f15: u32,
    _unused_16: u32,
) -> u32 {
    let l = |pass| leaf(one, pass);
    let first_variant = |e: &mut Engine| -> Pick {
        if one as u8 != 1 {
            return Pick::Count;
        }
        let shader = e.mem.u32(geometry + 0xa8);
        if material_is_black(e, geometry)
            && !below_one(e, shader + 0x28)
            && e.call(HAS_FLAG, &args![shader, 0x25u32]).bool()
        {
            let lit = e.call(HAS_FLAG, &args![shader, 0x22u32]).bool();
            return Pick::Pass(0xfc + u32::from(lit));
        }
        Pick::Pass(0xfb)
    };
    let second_variant = |e: &mut Engine| -> Pick {
        if one as u8 != 1 {
            return Pick::Count;
        }
        let shader = e.mem.u32(geometry + 0xa8);
        if material_is_black(e, geometry)
            && e.call(HAS_FLAG, &args![shader, 0x25u32]).bool()
            && !below_one(e, shader + 0x2c)
        {
            return Pick::Pass(0x102);
        }
        Pick::Pass(0x101)
    };
    let pick = if !on(f6) {
        if !on(f7) {
            if !on(f8) {
                if !on(f11) {
                    if !on(f12) {
                        if !on(f10) {
                            if !on(f13) {
                                if !on(f14) {
                                    if !on(f15) {
                                        first_variant(e)
                                    } else {
                                        l(0x104)
                                    }
                                } else {
                                    l(0x235)
                                }
                            } else if on(f14) {
                                l(0x235)
                            } else {
                                l(0x108)
                            }
                        } else if !on(f13) {
                            if !on(f15) {
                                l(0x10a)
                            } else {
                                l(0x110)
                            }
                        } else {
                            l(0x114)
                        }
                    } else if !on(f10) {
                        l(0xfe)
                    } else {
                        l(0x10b)
                    }
                } else if !on(f10) {
                    second_variant(e)
                } else {
                    l(0x10e)
                }
            } else if !on(f10) {
                l(0x100)
            } else {
                l(0x10d)
            }
        } else if on(f8) {
            Pick::Skip
        } else if !on(f11) {
            if !on(f10) {
                l(0xff)
            } else {
                l(0x10c)
            }
        } else if !on(f10) {
            l(0x103)
        } else {
            l(0x10f)
        }
    } else if !on(f7) {
        if !on(f8) {
            if on(f11) {
                Pick::Skip
            } else if !on(f10) {
                if !on(f15) {
                    l(0x105)
                } else {
                    l(0x109)
                }
            } else if !on(f15) {
                l(0x111)
            } else {
                l(0x115)
            }
        } else if !on(f10) {
            l(0x107)
        } else {
            l(0x113)
        }
    } else if on(f8) || on(f11) {
        Pick::Skip
    } else if !on(f10) {
        l(0x106)
    } else {
        l(0x112)
    };
    finish(
        e,
        this,
        pick,
        |pass| [geometry, pass, 1, 1, light, 0, 0, 0],
        count,
        flag,
    )
}

/// One of the variant tests of `AddAmbDiffTexSpecular10Pass_2x`: the
/// material black, the first light's colour (+0xF8 +0xD4) not black by
/// `00439090`, the shader's flag 0x25, `alpha_ok`, and none of the
/// property's flags 7, 0x15, 0x11 nor the shader's 0x1A, 0x1B (the tests in
/// the exe's order, each stopping the chain).
fn specular10_variant(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light: u32,
    alpha_ok: impl FnOnce(&mut Engine, u32) -> bool,
) -> bool {
    let shader = e.mem.u32(geometry + 0xa8);
    if !material_is_black(e, geometry) {
        return false;
    }
    let colour = e.mem.u32(light + 0xf8) + 0xd4;
    if e.call(COLOUR_COMPARE_00439090, &args![colour, BLACK])
        .bool()
    {
        return false;
    }
    if !e.call(HAS_FLAG, &args![shader, 0x25u32]).bool() || !alpha_ok(e, shader) {
        return false;
    }
    for flag in [7u32, 0x15, 0x11] {
        if e.call(HAS_FLAG, &args![this, flag]).bool() {
            return false;
        }
    }
    for flag in [0x1au32, 0x1b] {
        if e.call(HAS_FLAG, &args![shader, flag]).bool() {
            return false;
        }
    }
    true
}

// Translated from 00bde9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddAmbDiffTexSpecular10Pass_2x` (Xbox PDB):
/// `thiscall`, ten words. With the pass word `one` set (its low byte), a
/// pass 0 is added (first pass, no lights) and gets the active non-shadow
/// lights: `light` first, then as many as the count allows (the count is
/// the number of such lights plus one, at most 6/5/3 by `f8`, `f9` and
/// `f10`, unless both `f8` and `f9` are set). Then the pass enum (0xC3 to
/// 0xD6) comes from the byte flags `f6` to `f9` and whether fewer than five
/// lights were set, with the material variants 0xCA (over 0xC9) and 0xD1
/// (over 0xD0); with `one` not 1 the `u16` at `count` is raised. The byte at
/// `flag` is cleared. Answers the pass. With `one` clear no pass is added
/// and the exe reads the light count at address 9 (the null pass's +9).
/// The light iterator lives in the `geometry` argument's stack slot.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_amb_diff_tex_specular10_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light: u32,
    count: u32,
    one: u32,
    flag: u32,
    f6: u32,
    f7: u32,
    f8: u32,
    f9: u32,
    f10: u32,
) -> u32 {
    let mut pass = 0u32;
    if on(one) {
        add_pass(e, this, [geometry, 0, 1, 0, 0, 0, 0, 0]);
        pass = last_pass(e, this);
        let lights = e
            .call(ACTIVE_NON_SHADOW_LIGHT_COUNT, &args![this])
            .u8()
            .wrapping_add(1);
        e.mem.set_u8(pass + 9, lights);
        e.mem.set_u8(pass + 0xa, lights);
        e.call(PASS_FREE_LIGHTS, &args![pass]);
        let lights = u32::from(e.mem.u8(pass + 9));
        e.call(PASS_ALLOCATE_LIGHTS, &args![pass, lights]);
        let array = e.mem.u32(pass + 0xc);
        e.mem.set_u32(array, light);
        let current = i32::from(e.mem.u8(pass + 9));
        let extra = i32::from(on(f10)) + 1;
        let limit = match (on(f8), on(f9)) {
            (false, false) => Some(extra + 5),
            (false, true) => Some(extra + 4),
            (true, false) => Some(extra + 2),
            (true, true) => None,
        };
        let clamped = match limit {
            Some(limit) if current >= limit => limit,
            _ => current,
        };
        e.mem.set_u8(pass + 9, clamped as u8);
        e.with_stack(4, |e, iterator| {
            let mut next = e
                .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![this, iterator])
                .u32();
            let mut index: i32 = 1;
            while next != 0 {
                if index >= i32::from(e.mem.u8(pass + 9)) {
                    break;
                }
                let array = e.mem.u32(pass + 0xc);
                e.mem.set_u32(array + 4 * index as u32, next);
                index += 1;
                next = e
                    .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![this, iterator])
                    .u32();
            }
        });
    }
    let few = i32::from(e.mem.u8(pass + 9)) < 5;
    let leaf = |pass_enum: u16| -> Option<u16> {
        if one as u8 == 1 {
            Some(pass_enum)
        } else {
            None
        }
    };
    let pick = if !on(f6) {
        if on(f7) {
            if on(f8) {
                leaf(0xd2)
            } else if few {
                leaf(0xcb)
            } else {
                leaf(0xc4)
            }
        } else if on(f8) {
            if on(f9) {
                leaf(0xd3)
            } else if one as u8 == 1 {
                let property_state = e.mem.u32(geometry + 0x9c);
                let variant = specular10_variant(e, this, geometry, light, |e, shader| {
                    property_state == 0
                        || (e.mem.u16(property_state + 0x18) >> 9) & 1 == 0
                        || !below_one(e, shader + 0x2c)
                });
                Some(if variant { 0xd1 } else { 0xd0 })
            } else {
                None
            }
        } else if on(f9) {
            if few {
                leaf(0xcc)
            } else {
                leaf(0xc5)
            }
        } else if few {
            if one as u8 == 1 {
                let variant = specular10_variant(e, this, geometry, light, |e, shader| {
                    !below_one(e, shader + 0x2c)
                });
                Some(if variant { 0xca } else { 0xc9 })
            } else {
                None
            }
        } else {
            leaf(0xc3)
        }
    } else if on(f7) {
        if on(f8) {
            leaf(0xd5)
        } else if few {
            leaf(0xce)
        } else {
            leaf(0xc7)
        }
    } else if on(f8) {
        if on(f9) {
            leaf(0xd6)
        } else {
            leaf(0xd4)
        }
    } else if on(f9) {
        if few {
            leaf(0xcf)
        } else {
            leaf(0xc8)
        }
    } else if few {
        leaf(0xcd)
    } else {
        leaf(0xc6)
    };
    match pick {
        Some(pass_enum) => e.mem.set_u16(pass + 4, pass_enum),
        None => {
            let value = e.mem.u16(count);
            e.mem.set_u16(count, value.wrapping_add(1));
        }
    }
    e.mem.set_u8(flag, 0);
    pass
}

// Translated from 00bdef40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddSpecularPass_2x` (Xbox PDB): `thiscall`,
/// eleven words. Nothing (answers 0) for a black light (its +0xF8 object's
/// colour at +0xD4). Otherwise a pass table of one light (passes 0x15A to
/// 0x16B, the byte at `flag` as the first-pass flag) chosen by the byte flag
/// `f6`, the light's byte +0xF4 and `f8` to `f11`; otherwise the `u16` at
/// `count` is raised (some combinations skip both). Clears the byte at
/// `flag`; answers 0.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_specular_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light: u32,
    count: u32,
    one: u32,
    flag: u32,
    f6: u32,
    _unused_7: u32,
    f8: u32,
    f9: u32,
    f10: u32,
    f11: u32,
) -> u32 {
    let own = e.mem.u8(light + 0xf4) != 0;
    if is_black(e, light) {
        return 0;
    }
    let l = |pass| leaf(one, pass);
    let pick = if !on(f6) {
        if !own {
            if !on(f9) {
                if !on(f8) {
                    if !on(f10) {
                        if !on(f11) {
                            l(0x15a)
                        } else {
                            l(0x15b)
                        }
                    } else {
                        l(0x15f)
                    }
                } else if !on(f10) {
                    if !on(f11) {
                        l(0x160)
                    } else {
                        l(0x161)
                    }
                } else {
                    l(0x165)
                }
            } else if !on(f8) {
                l(0x15c)
            } else {
                l(0x162)
            }
        } else if !on(f9) {
            if !on(f10) {
                if !on(f11) {
                    l(0x166)
                } else {
                    l(0x167)
                }
            } else {
                l(0x16b)
            }
        } else {
            l(0x168)
        }
    } else if !own {
        if on(f9) {
            Pick::Skip
        } else if !on(f8) {
            if !on(f11) {
                l(0x15d)
            } else {
                l(0x15e)
            }
        } else if !on(f11) {
            l(0x163)
        } else {
            l(0x164)
        }
    } else if on(f9) {
        Pick::Skip
    } else if !on(f11) {
        l(0x169)
    } else {
        l(0x16a)
    };
    let first = u32::from(e.mem.u8(flag));
    finish(
        e,
        this,
        pick,
        |pass| [geometry, pass, first, 1, light, 0, 0, 0],
        count,
        flag,
    )
}

// Translated from 00bdf3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddLandscapePasses_2x` (Xbox PDB):
/// `thiscall`, seven words. The first three active non-shadow lights that
/// are not black are taken; the texture count (vtable slot +0xEC, at most 7)
/// picks the pass: `0x1F7 + 8 * count` without a light, `0x1FB + 8 * count`
/// with one (with all three lights), plus 1 with `f7` and 2 with bit 20 of
/// the second flag word; added with `light` first when the pass word `one`
/// is 1, else the `u16` at `count` is raised. The byte at `flag` is
/// cleared. Then, with `one` at 1, the byte at `011f94ab` set and bit 14 of
/// the second flag word clear, pass 0x230 (not first, `light`) is added
/// with its extra parameter 9; otherwise the count is raised.
#[allow(clippy::too_many_arguments)]
pub fn bs_shader_pp_lighting_property_add_landscape_passes_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    light: u32,
    count: u32,
    one: u32,
    flag: u32,
    _unused_6: u32,
    f7: u32,
) {
    let base = this.addr();
    let [first, second, third] = e.with_stack(4, |e, iterator| {
        let mut found = [0u32; 3];
        let mut next = e
            .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![this, iterator])
            .u32();
        for (index, slot) in found.iter_mut().enumerate() {
            if index > 0 {
                next = e
                    .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![this, iterator])
                    .u32();
            }
            while next != 0 && is_black(e, next) {
                next = e
                    .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![this, iterator])
                    .u32();
            }
            *slot = next;
        }
        found
    });
    let textures = e.vcall(base, 0xec, &[]).u16();
    let textures = u32::from(if textures >= 7 { 7 } else { textures });
    let wide = e.mem.u32(base + 0x24) & 0x0010_0000 != 0;
    let mut pass = if first == 0 {
        textures * 8 + 0x1f7
    } else {
        textures * 8 + 0x1fb
    };
    if on(f7) {
        pass += 1;
    }
    if wide {
        pass += 2;
    }
    let raise = |e: &mut Engine| {
        let value = e.mem.u16(count);
        e.mem.set_u16(count, value.wrapping_add(1));
    };
    if one as u8 == 1 {
        if first == 0 {
            add_pass(e, this, [geometry, pass, 1, 1, light, 0, 0, 0]);
        } else {
            add_pass(e, this, [geometry, pass, 1, 4, light, first, second, third]);
        }
    } else {
        raise(e);
    }
    e.mem.set_u8(flag, 0);
    if one as u8 != 1
        || e.global::<u8>(LANDSCAPE_EXTRA_011F94AB) == 0
        || e.mem.u32(base + 0x24) & 0x4000 != 0
    {
        raise(e);
        return;
    }
    add_pass(e, this, [geometry, 0x230, 0, 1, light, 0, 0, 0]);
    let added = last_pass(e, this);
    e.mem.set_u8(added + 0xb, 9);
}

// Translated from 00bdf650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddDepthMapPass_2x` (Xbox PDB): `thiscall`,
/// four words. Adds to the depth-map list (+0x40) pass 0x10 to 0x13 (by the
/// byte flags `skinned` and `second`; first pass) with the accumulator's
/// light (+0x1A0).
pub fn bs_shader_pp_lighting_property_add_depth_map_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    skinned: u32,
    second: u32,
    accumulator: u32,
) {
    let light = e.mem.u32(accumulator + 0x1a0);
    let list = e.mem.u32(this.addr() + DEPTH_PASSES);
    let pass: u32 = match (on(skinned), on(second)) {
        (false, false) => 0x10,
        (false, true) => 0x11,
        (true, false) => 0x12,
        (true, true) => 0x13,
    };
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, 1u32, 1u32, light, 0u32, 0u32, 0u32],
    );
}

/// The pass helpers `GetRenderPasses_2x` calls besides this unit's
/// (`thiscall`; engine map unit `bsshaderpplightingproperty_passes_1x.cpp`
/// unless said otherwise): `00bd9f00` (six words), `00bd9f90` (five),
/// `00bd9e60` (five), `00bdc0d0` (thirteen, this unit's file has no
/// translation of it), `00bd9540` (nine), `00bd9840` (nine), `00bd9bc0`
/// (seven), `00bdc030` (five), `00bdb380` (five), `00bda030` (four),
/// `00bd9da0` (six).
const PASS_00BD9F00: u32 = 0x00bd_9f00;
const PASS_00BD9F90: u32 = 0x00bd_9f90;
const PASS_00BD9E60: u32 = 0x00bd_9e60;
const PASS_00BDC0D0: u32 = 0x00bd_c0d0;
const PASS_00BD9540: u32 = 0x00bd_9540;
const PASS_00BD9840: u32 = 0x00bd_9840;
const PASS_00BD9BC0: u32 = 0x00bd_9bc0;
const PASS_00BDC030: u32 = 0x00bd_c030;
const PASS_00BDB380: u32 = 0x00bd_b380;
const PASS_00BDA030: u32 = 0x00bd_a030;
const PASS_00BD9DA0: u32 = 0x00bd_9da0;
/// `00653290` (unnamed, `thiscall`, one word: an `NiRTTI` record): `this`
/// when the object is of that kind, else 0; with the records `01200788`,
/// `012024c8` and `012007a0`.
const KIND_OF: u32 = 0x0065_3290;
const RTTI_01200788: u32 = 0x0120_0788;
const RTTI_012024C8: u32 = 0x0120_24c8;
const RTTI_012007A0: u32 = 0x0120_07a0;
/// `NiAVObject::GetProperty` (`00a59d30`, one word),
/// `BSShaderManager::GetAccumulator` (`00b4f5c0`), `005bf7b0` (no words, a
/// byte), `GetFirstActiveLight` (`00b70590`) and `GetNextActiveLight`
/// (`00b70680`) with the iterator's address.
const GET_PROPERTY: u32 = 0x00a5_9d30;
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const DEPTH_PASSES_WANTED: u32 = 0x005b_f7b0;
const FIRST_ACTIVE_LIGHT: u32 = 0x00b7_0590;
const NEXT_ACTIVE_LIGHT: u32 = 0x00b7_0680;
/// The render settings `GetRenderPasses_2x` reads: bytes `011f9427`,
/// `011f91a7` and `011f9422`, the word `011f95e4`, the byte `011f91d8`, the
/// shader level `011f91c0`, the `float` `011fd890`, the fade distances
/// `011f9458`/`011f9460`/`011f9468`, and the pass mask `010a91d4`.
const SETTING_011F9427: u32 = 0x011f_9427;
const SETTING_011F95E4: u32 = 0x011f_95e4;
const SETTING_011F91D8: u32 = 0x011f_91d8;
const SHADER_LEVEL_011F91C0: u32 = 0x011f_91c0;
const SETTING_011FD890: u32 = 0x011f_d890;
const PASSES_FORCED_011F91A7: u32 = 0x011f_91a7;
const SETTING_011F9422: u32 = 0x011f_9422;
const FADE_FAR_011F9458: u32 = 0x011f_9458;
const FADE_FAR_011F9460: u32 = 0x011f_9460;
const FADE_FAR_011F9468: u32 = 0x011f_9468;
const PASS_MASK_010A91D4: u32 = 0x010a_91d4;

/// The call words of the pass helpers, built as the exe pushes them (each a
/// 32-bit read of the frame; the helpers read the low byte).
struct Frame {
    base: u32,
}

impl Frame {
    fn b(&self, e: &Engine, k: u32) -> bool {
        e.mem.u8(self.base + k) != 0
    }
    fn set(&self, e: &mut Engine, k: u32, value: bool) {
        e.mem.set_u8(self.base + k, u8::from(value));
    }
    fn w(&self, e: &Engine, k: u32) -> u32 {
        e.mem.u32(self.base + k)
    }
    fn set_w(&self, e: &mut Engine, k: u32, value: u32) {
        e.mem.set_u32(self.base + k, value);
    }
    fn at(&self, k: u32) -> u32 {
        self.base + k
    }
}

/// The last pass of the main list gets the accumulation hint `hint`.
fn hint_last(e: &mut Engine, this: Ptr, hint: u8) {
    let pass = last_pass(e, this);
    e.mem.set_u8(pass + 6, hint);
}

// Translated from 00bdf790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::GetRenderPasses_2x` (Xbox PDB): `thiscall`,
/// six words: the geometry, the pass mask word (`mask`: bits 0 to 3), the
/// address of the `u16` pass count, a kind word (3 counts as bit 5 of the
/// flags), the accumulator and the pass word. Builds the main pass list
/// from the flags of the property (+0x20, +0x24), the geometry's property 0
/// (or its property state), the alpha and fade alpha, the fade distances,
/// the active lights and the render settings: the translucent pass
/// (`00bd9f00`), the +0xDC pass (`00bd9f90`), the +0x24 bit-6 pass
/// (`00bd9e60`), the shadow passes 0x231 to 0x234 (last pass), then one of
/// the ambient-diffuse tables (`AddAmbDiffTex*Pass_2x`, `00bdc0d0`,
/// `00bdc530`, `00bd9540`), the point-light passes (`00bd9840`,
/// `AddDiffusePt2Pass_2x`, `AddDiffusePt3Pass_2x`), the texture pass, the
/// specular passes (`AddSpecularPass_2x`, `AddSpecularPt2Pass_2x`,
/// `AddSpecularPt3Pass_2x`), the envmap or landscape passes, the last-pass
/// mark or `00bd9bc0`, `00bdc030`, `00bdb380` and `00bda030`; finally every
/// pass without an accumulation hint gets 0xA, 4, 0xE or 1. Two flag
/// combinations hand over to vtable slot +0x114 or to `00bd9da0` instead.
/// The exe keeps its many flags as bytes of its stack frame and passes them
/// as 32-bit reads of it; the translation keeps the same frame (the frame's
/// other bytes are zero where the exe has whatever its stack held; the
/// helpers read only the low bytes).
pub fn bs_shader_pp_lighting_property_get_render_passes_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    mask: u32,
    count: u32,
    kind: u32,
    accumulator: u32,
    one: u32,
) {
    let base = this.addr();
    e.with_stack(0x90, |e, frame| {
        let f = Frame { base: frame.addr() };
        let flags0 = e.mem.u32(base + 0x20);
        f.set(e, 0x59, flags0 & 1 != 0);
        f.set(e, 0x13, flags0 & 0x80 != 0);
        f.set(e, 0x7c, flags0 & 0x0002_0000 != 0);
        let wide_bit = flags0 & 0x0020_0000 != 0;
        f.set(e, 0x14, flags0 & 2 != 0);
        f.set(e, 0x80, wide_bit);
        f.set(e, 0x18, flags0 as u8 & 0x20 != 0 || kind == 3);
        let b4 = e.mem.u32(base + 0xb4);
        f.set(e, 0x47, flags0 & 0x100 != 0);
        let bit10 = flags0 & 0x400 != 0;
        f.set(e, 0x1c, bit10);
        f.set(e, 0x3c, e.mem.u32(b4) != 0 && !bit10);
        let data = e.mem.u32(geometry + 0xb8);
        let colours = e.mem.u32(data + 0x28) != 0 || e.mem.u8(base + 0x24) & 0x20 != 0;
        f.set(e, 0x74, colours);
        let mut shadows = false;
        if e.global::<u8>(SETTING_011F9427) == 0
            && e.global::<u32>(SETTING_011F95E4) != 0
            && e.global::<u8>(SETTING_011F91D8) & 0x20 != 0
            && e.global::<i32>(SHADER_LEVEL_011F91C0) >= 2
            && flags0 & 0x2000 == 0
        {
            // `FNSTCW`, the word with rounding toward zero, `FISTP qword`.
            let control = crate::x87::control_word();
            e.mem.set_u16(f.at(0x54), control);
            f.set_w(e, 0x6c, u32::from(control) | 0xc00);
            let value = X87Ext::load32(e.global::<u32>(SETTING_011FD890));
            let whole = x87_fistp64_chop(value) as u64;
            e.mem.set_u64(f.at(0x84), whole);
            if whole as u32 != 0 {
                let accumulator_now = e.call(GET_ACCUMULATOR, &[]).u32();
                let object = e.mem.u32(accumulator_now + 0x194);
                if e.mem.u8(object + 0x120) != 1 {
                    let accumulator_now = e.call(GET_ACCUMULATOR, &[]).u32();
                    let object = e.mem.u32(accumulator_now + 0x194);
                    shadows = e.mem.u8(object + 0x120) != 3;
                }
            }
        }
        f.set(e, 0x33, shadows);
        let kind_a = e.call(KIND_OF, &args![base, RTTI_01200788]).u32();
        f.set(e, 0x64, kind_a != 0);
        let kind_b = e.call(KIND_OF, &args![base, RTTI_012024C8]).u32();
        f.set(e, 0x5d, kind_b != 0);
        let kind_c = e.call(KIND_OF, &args![base, RTTI_012007A0]).u32();
        f.set(e, 0x5e, kind_c != 0);
        let flags0 = e.mem.u32(base + 0x20);
        f.set(e, 0x70, flags0 & 0x0040_0000 != 0);
        f.set(e, 0x20, flags0 & 0x800 != 0);
        f.set(e, 0x60, flags0 & 0x1000 != 0);
        f.set(e, 0x19, flags0 & 0x4000 != 0);
        f.set(e, 0x5f, flags0 & 0x8000 != 0);
        f.set(e, 0x4c, flags0 & 0x0001_0000 != 0);
        f.set(e, 0x1b, flags0 & 0x200 != 0);
        f.set(
            e,
            0x46,
            flags0 & 0x0010_0000 == 0 || e.global::<u8>(PASSES_FORCED_011F91A7) != 0,
        );
        f.set(e, 0x38, flags0 & 0x0004_0000 != 0);
        f.set(
            e,
            0x45,
            flags0 & 0x0400_0000 != 0 || flags0 & 0x0800_0000 != 0,
        );
        let state_slot = geometry.wrapping_add(0x9c);
        let property_state = e.mem.u32(geometry + 0x9c);
        let flags1 = e.mem.u32(base + 0x24);
        let blend_hint = e.mem.u8(property_state + 0x18) & 1 == 0
            && (e.mem.u32(accumulator + 0x34) == 2
                || flags1 as u8 & 4 != 0
                || flags1 as u8 & 2 != 0);
        f.set(e, 0x30, blend_hint);

        let mut property = e.call(GET_PROPERTY, &args![geometry, 0u32]).u32();
        if property == 0 {
            property = if state_slot == 0 {
                0
            } else {
                e.mem.u32(state_slot)
            };
        }
        f.set_w(e, 0x78, property);
        let blended = (property != 0 && e.mem.u8(property + 0x18) & 1 != 0)
            || 1.0 > x87_m32(e.mem.u32(base + 0x28))
            || 1.0 > x87_m32(e.mem.u32(base + 0x2c));
        f.set(e, 0x68, blended);
        if property != 0 && (e.mem.u16(property + 0x18) >> 9) & 1 != 0 {
            f.set(e, 0x40, true);
            f.set(e, 0x47, false);
        } else {
            f.set(e, 0x40, false);
        }
        if e.mem.u32(base + 0x24) & 0x0080_0000 == 0 {
            let distance = x87_load32(e.mem.u32(base + 0x34));
            if f.b(e, 0x7c) {
                let limit = x87_load32(e.global::<u32>(FADE_FAR_011F9468));
                let near = limit == 0.0 || !(distance >= limit);
                f.set(e, 0x13, near);
                f.set(e, 0x59, false);
            } else {
                let limit = x87_load32(e.global::<u32>(FADE_FAR_011F9460));
                if !(limit == 0.0) && distance >= limit {
                    f.set(e, 0x13, false);
                }
            }
            let limit = x87_load32(e.global::<u32>(FADE_FAR_011F9458));
            if !(limit == 0.0) && distance >= limit {
                f.set(e, 0x59, false);
            }
        }
        let accumulator_now = e.call(GET_ACCUMULATOR, &[]).u32();
        let object = e.mem.u32(accumulator_now + 0x194);
        let word = e.mem.u32(object + 0xe0);
        f.set_w(e, 0x2c, word);
        let lit = f.b(e, 0x59) && {
            let colour = e.mem.u32(word + 0xf8) + 0xe0;
            e.call(COLOUR_COMPARE_00439090, &args![colour, BLACK])
                .bool()
        };
        f.set(e, 0x31, lit);
        let pass_flag = f.at(0x12);
        if f.b(e, 0x68) {
            f.set(e, 0x1a, true);
            let go = (f.b(e, 0x1b) || below_one(e, base + 0x2c))
                && !(property != 0 && e.mem.u8(property + 0x18) & 1 != 0)
                && e.mem.u32(base + 0x20) & 0x8000 == 0
                && e.mem.u32(base + 0x20) & 0x0001_0000 == 0;
            if go {
                let w14 = f.w(e, 0x14);
                let w40 = f.w(e, 0x40);
                e.call(
                    PASS_00BD9F00,
                    &args![base, geometry, count, one, f.at(0x1a), w14, w40],
                );
                if below_one(e, base + 0x2c) || below_one(e, base + 0x28) {
                    hint_last(e, this, 0xe);
                }
                f.set(e, 0x59, false);
            }
            f.set(e, 0x18, true);
        }
        f.set(e, 0x12, true);
        let extra = e.mem.u32(base + 0xdc);
        if extra != 0 && e.mem.u32(extra + 0x6c) != 0 {
            let w14 = f.w(e, 0x14);
            e.call(
                PASS_00BD9F90,
                &args![base, geometry, count, one, pass_flag, w14],
            );
            hint_last(e, this, 0xe);
        }
        let lights = u32::from(e.call(ACTIVE_NON_SHADOW_LIGHT_COUNT, &args![base]).u16());
        f.set(e, 0x54, mask & 1 != 0);
        f.set(e, 0x32, mask & 2 != 0);
        f.set_w(e, 0x84, mask & 1);
        f.set(e, 0x37, mask & 4 != 0);
        f.set_w(e, 0x50, lights);
        let bit3 = mask & 8 != 0;
        let lit_specular = bit3 && f.b(e, 0x59);
        f.set(e, 0x28, lit_specular);
        let envmap = f.b(e, 0x13) && bit3;
        f.set(e, 0x1b, envmap);
        let wanted = e.global::<u32>(PASS_MASK_010A91D4);
        f.set(e, 0x13, wanted & mask == wanted);
        f.set(e, 0x1a, true);
        f.set_w(e, 0x6c, lights);
        f.set_w(e, 0x48, lights);
        if f.b(e, 0x5f) || f.b(e, 0x4c) {
            let w4c = f.w(e, 0x4c);
            let w14 = f.w(e, 0x14);
            e.call(
                PASS_00BD9DA0,
                &args![base, geometry, count, one, pass_flag, w14, w4c],
            );
            hint_last(e, this, 6);
            return;
        }
        if e.mem.u8(base + 0x24) & 0x40 != 0 {
            let w14 = f.w(e, 0x14);
            e.call(
                PASS_00BD9E60,
                &args![base, geometry, count, one, pass_flag, w14],
            );
            hint_last(e, this, 7);
        }
        if f.b(e, 0x5d) || f.b(e, 0x5e) {
            let lights_word = e.mem.u32(accumulator + 0x19c);
            e.vcall(
                base,
                0x114,
                &args![geometry, mask, count, lights_word, accumulator, one],
            );
            return;
        }
        let iterator = f.at(0x24);
        f.set_w(e, 0x24, 0);
        if e.call(DEPTH_PASSES_WANTED, &[]).bool() && !f.b(e, 0x30) && !f.b(e, 0x60) {
            let shadow_iterator = f.at(0x4c);
            f.set_w(e, 0x4c, 0);
            let mut light = e
                .call(FIRST_ACTIVE_LIGHT, &args![base, shadow_iterator])
                .u32();
            while light != 0 {
                if e.mem.u8(light + 0xec) != 0 {
                    add_pass(e, this, [geometry, 0, 1, 1, light, 0, 0, 0]);
                    let pass = last_pass(e, this);
                    let pass_enum: u16 = if f.b(e, 0x14) {
                        0x232
                    } else if f.b(e, 0x19) {
                        0x233
                    } else if f.b(e, 0x64) {
                        0x234
                    } else {
                        0x231
                    };
                    e.mem.set_u16(pass + 4, pass_enum);
                    e.mem.set_u8(pass + 8, 1);
                }
                light = e
                    .call(NEXT_ACTIVE_LIGHT, &args![base, shadow_iterator])
                    .u32();
            }
        }
        let w64 = f.w(e, 0x64);
        let main_tables = |e: &mut Engine| -> Option<bool> {
            // `None`: on to the specular part (`00be017b`); `Some(lit)`:
            // done (`00be061d`), `lit` asking for the specular passes.
            let n16 = e.mem.u16(f.at(0x50));
            let shortcut = n16 <= 1
                && f.b(e, 0x31)
                && (n16 == 0 || !f.b(e, 0x33))
                && f.b(e, 0x28)
                && !f.b(e, 0x1c);
            let ambient = shortcut
                || (f.b(e, 0x18)
                    && (f.b(e, 0x20)
                        || f.b(e, 0x38)
                        || f.b(e, 0x33)
                        || f.b(e, 0x60)
                        || f.b(e, 0x64)
                        || f.b(e, 0x3c)));
            let words = |e: &Engine| {
                [
                    f.w(e, 0x14),
                    f.w(e, 0x3c),
                    f.w(e, 0x1c),
                    f.w(e, 0x33),
                    f.w(e, 0x20),
                    f.w(e, 0x38),
                ]
            };
            if ambient {
                let specular = (shortcut || !f.b(e, 0x1c)) && f.b(e, 0x28);
                let [w14, w3c, w1c, w33, w20, w38] = words(e);
                if specular {
                    if n16 != 0 {
                        let light = e
                            .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                            .u32();
                        bs_shader_pp_lighting_property_add_amb_diff_tex_specular2_pass_2x(
                            e, this, geometry, word, light, count, one, pass_flag, w14, w3c, w1c,
                            0u32, w33, w20, w64, w38,
                        );
                    } else {
                        bs_shader_pp_lighting_property_add_amb_diff_tex_specular_pass_2x(
                            e, this, geometry, word, count, one, pass_flag, w14, w3c, w1c, 0u32,
                            w33, w20, w64, w38,
                        );
                    }
                } else if n16 == 0 {
                    let w60 = f.w(e, 0x60);
                    let w70 = f.w(e, 0x70);
                    let w40 = f.w(e, 0x40);
                    bs_shader_pp_lighting_property_add_amb_diff_tex_pass_2x(
                        e, this, geometry, word, count, one, pass_flag, w14, w3c, w1c, 0u32, w33,
                        w20, w60, w64, w70, w38, w40,
                    );
                } else {
                    let light = e
                        .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                        .u32();
                    bs_shader_pp_lighting_property_add_amb_diff_tex2_pass_2x(
                        e, this, geometry, word, light, count, one, pass_flag, w14, w3c, w1c, 0u32,
                        w33, w20, w64, w38,
                    );
                }
                f.set(e, 0x13, false);
                return Some(false);
            }
            // `00bdffb8`: the specular-10 pass.
            f.set_w(e, 0x54, 1);
            let material_black = material_is_black(e, geometry);
            let variant = material_black
                && {
                    let colour = e.mem.u32(word + 0xf8) + 0xd4;
                    !e.call(COLOUR_COMPARE_00439090, &args![colour, BLACK])
                        .bool()
                }
                && e.call(HAS_FLAG, &args![base, 0x25u32]).bool()
                && {
                    let property = f.w(e, 0x78);
                    property == 0
                        || (e.mem.u16(property + 0x18) >> 9) & 1 == 0
                        || !below_one(e, base + 0x2c)
                }
                && !e.call(HAS_FLAG, &args![base, 0x1au32]).bool()
                && !e.call(HAS_FLAG, &args![base, 0x1bu32]).bool()
                && !e.call(HAS_FLAG, &args![base, 7u32]).bool()
                && !e.call(HAS_FLAG, &args![base, 0x15u32]).bool()
                && !e.call(HAS_FLAG, &args![base, 0x11u32]).bool()
                && e.mem.u32(geometry + 0xbc) == 0;
            if !variant {
                f.set_w(e, 0x54, 0);
            }
            let mut take = f.b(e, 0x18);
            if !take {
                let v = f.w(e, 0x54) as i32;
                let n = f.w(e, 0x6c) as i32;
                let (s28, s1c) = (f.b(e, 0x28), f.b(e, 0x1c));
                let middle = (n <= v + 5 && !s28 && !s1c)
                    || (n <= v + 4 && !s28 && s1c)
                    || (n <= v + 2 && s28 && !s1c);
                take = middle
                    && (e.mem.u16(f.at(0x50)) > 1 || f.b(e, 0x59))
                    && !f.b(e, 0x20)
                    && !f.b(e, 0x38)
                    && !f.b(e, 0x33)
                    && !f.b(e, 0x60)
                    && !f.b(e, 0x64)
                    && !f.b(e, 0x3c);
            }
            if take {
                let ten = u32::from(f.w(e, 0x54) != 0);
                let [w14, w3c, w1c, _, _, _] = words(e);
                let w28 = f.w(e, 0x28);
                bs_shader_pp_lighting_property_add_amb_diff_tex_specular10_pass_2x(
                    e, this, geometry, word, count, one, pass_flag, w14, w3c, w28, w1c, ten,
                );
                f.set(e, 0x13, false);
                return Some(false);
            }
            None
        };
        // The flow from `00bdfd87`.
        let mut to_specular_tail = false;
        let mut done = false;
        if f.b(e, 0x19) {
            // `00be0660`: the landscape passes.
            let w33 = f.w(e, 0x33);
            let w59 = f.w(e, 0x59);
            bs_shader_pp_lighting_property_add_landscape_passes_2x(
                e, this, geometry, word, count, one, pass_flag, w59, w33,
            );
            done = true;
        } else if !f.b(e, 0x46) {
            done = true;
        }
        let mut stage = if done { 3 } else { 0 };
        if stage == 0 {
            if !f.b(e, 0x54) {
                stage = 2;
            } else if e.global::<u8>(SETTING_011F9422) != 0 {
                stage = 1;
            } else if !f.b(e, 0x18) && !f.b(e, 0x68) {
                if !f.b(e, 0x32) {
                    stage = 4;
                } else if !f.b(e, 0x37) || f.b(e, 0x47) || f.b(e, 0x70) {
                    stage = 1;
                }
            }
        }
        if stage == 0 {
            match main_tables(e) {
                Some(_) => stage = 6,
                None => stage = 1,
            }
        }
        if stage == 1 {
            // `00be017b`.
            if !f.b(e, 0x32) {
                stage = 4;
            } else {
                let byte_9427 = e.global::<u8>(SETTING_011F9427) != 0;
                let ambient = f.b(e, 0x37)
                    && !(f.b(e, 0x1c) && byte_9427)
                    && (e.mem.u16(f.at(0x50)) <= 1 || f.b(e, 0x18));
                if ambient {
                    let n16 = e.mem.u16(f.at(0x50));
                    let w14 = f.w(e, 0x14);
                    let w3c = f.w(e, 0x3c);
                    let w1c = f.w(e, 0x1c);
                    let w33 = f.w(e, 0x33);
                    let w20 = f.w(e, 0x20);
                    let w38 = f.w(e, 0x38);
                    if n16 == 0 {
                        let w60 = f.w(e, 0x60);
                        let w70 = f.w(e, 0x70);
                        let w40 = f.w(e, 0x40);
                        bs_shader_pp_lighting_property_add_amb_diff_tex_pass_2x(
                            e, this, geometry, word, count, one, pass_flag, w14, w3c, w1c, 0u32,
                            w33, w20, w60, w64, w70, w38, w40,
                        );
                    } else {
                        let light = e
                            .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                            .u32();
                        bs_shader_pp_lighting_property_add_amb_diff_tex2_pass_2x(
                            e, this, geometry, word, light, count, one, pass_flag, w14, w3c, w1c,
                            0u32, w33, w20, w64, w38,
                        );
                    }
                    if f.b(e, 0x28) {
                        to_specular_tail = true;
                        stage = 5;
                    } else {
                        f.set(e, 0x13, false);
                        stage = 6;
                    }
                } else if f.b(e, 0x1c) && byte_9427 {
                    stage = 4;
                } else {
                    // `00be0279`.
                    let light = e
                        .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                        .u32();
                    let w14 = f.w(e, 0x14);
                    let w3c = f.w(e, 0x3c);
                    let w1c = f.w(e, 0x1c);
                    let w33 = f.w(e, 0x33);
                    let w20 = f.w(e, 0x20);
                    if e.mem.u16(f.at(0x50)) <= 1 {
                        e.call(
                            PASS_00BDC0D0,
                            &args![
                                base, geometry, word, light, count, one, pass_flag, w14, w3c, w1c,
                                0u32, w33, w20, w64
                            ],
                        );
                        f.set_w(e, 0x48, 0);
                    } else {
                        f.set_w(e, 0x4c, light);
                        let second = e
                            .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                            .u32();
                        fn_00bdc530(
                            e, this, geometry, word, light, second, count, one, pass_flag, w14,
                            w3c, w1c, 0u32, w33, w20, w64,
                        );
                        let left = f.w(e, 0x48).wrapping_sub(2);
                        f.set_w(e, 0x48, left);
                    }
                    f.set(e, 0x1a, false);
                    stage = 2;
                }
            }
        }
        if stage == 4 {
            // `00be0330`.
            let w3c = f.w(e, 0x3c);
            let w40 = f.w(e, 0x40);
            let w14 = f.w(e, 0x14);
            e.call(
                PASS_00BD9540,
                &args![base, geometry, word, count, one, pass_flag, w14, w40, w3c, w64],
            );
            stage = 2;
        }
        if stage == 2 {
            // `00be035b`: the point-light passes.
            if f.b(e, 0x32) {
                let light = if f.b(e, 0x1a) {
                    let w14 = f.w(e, 0x14);
                    e.call(
                        PASS_00BD9840,
                        &args![base, geometry, word, count, one, pass_flag, w14, 0u32, 0u32, w64],
                    );
                    e.call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                        .u32()
                } else {
                    e.call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                        .u32()
                };
                f.set_w(e, 0x40, light);
                let mut light = light;
                while light != 0 {
                    let w14 = f.w(e, 0x14);
                    let w1c = f.w(e, 0x1c);
                    let w20 = f.w(e, 0x20);
                    if f.w(e, 0x48) <= 2 {
                        let second = e
                            .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                            .u32();
                        let first = f.w(e, 0x40);
                        bs_shader_pp_lighting_property_add_diffuse_pt2_pass_2x(
                            e, this, geometry, first, second, count, one, pass_flag, w14, w1c,
                            0u32, w20, w64,
                        );
                        let left = f.w(e, 0x48);
                        f.set_w(e, 0x48, if left <= 1 { 0 } else { left - 2 });
                    } else {
                        let second = e
                            .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                            .u32();
                        f.set_w(e, 0x4c, second);
                        let third = e
                            .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                            .u32();
                        let first = f.w(e, 0x40);
                        let second = f.w(e, 0x4c);
                        bs_shader_pp_lighting_property_add_diffuse_pt3_pass_2x(
                            e, this, geometry, first, second, third, count, one, pass_flag, w14,
                            w1c, 0u32, w20, w64,
                        );
                        let left = f.w(e, 0x48).wrapping_sub(3);
                        f.set_w(e, 0x48, left);
                    }
                    light = e
                        .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                        .u32();
                    f.set_w(e, 0x40, light);
                }
            }
            // `00be0483`: the texture pass.
            if f.b(e, 0x37) {
                let w38 = f.w(e, 0x38);
                let w20 = f.w(e, 0x20);
                let w74 = f.w(e, 0x74);
                let w1c = f.w(e, 0x1c);
                let w14 = f.w(e, 0x14);
                bs_shader_pp_lighting_property_add_texture_pass_2x(
                    e, this, geometry, word, count, one, pass_flag, w14, 0u32, w1c, w74, w64, 0u32,
                    w20, w38,
                );
            }
            if f.b(e, 0x28) {
                to_specular_tail = true;
            }
            stage = if to_specular_tail { 5 } else { 6 };
        }
        if stage == 5 {
            // `00be04ce`: the specular passes.
            let lit = f.b(e, 0x31);
            let left = f.w(e, 0x6c);
            f.set_w(e, 0x28, left);
            if lit {
                let w38 = f.w(e, 0x38);
                let w14 = f.w(e, 0x14);
                let w20 = f.w(e, 0x20);
                let w33 = f.w(e, 0x33);
                bs_shader_pp_lighting_property_add_specular_pass_2x(
                    e, this, geometry, word, count, one, pass_flag, w14, 0u32, w33, w20, w64, w38,
                );
            }
            let mut light = e
                .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                .u32();
            f.set_w(e, 0x50, light);
            while light != 0 {
                let left = f.w(e, 0x28) as i32;
                let w38 = f.w(e, 0x38);
                let w14 = f.w(e, 0x14);
                let w20 = f.w(e, 0x20);
                if left > 2 {
                    let second = e
                        .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                        .u32();
                    f.set_w(e, 0x74, second);
                    let third = e
                        .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                        .u32();
                    let first = f.w(e, 0x50);
                    let second = f.w(e, 0x74);
                    bs_shader_pp_lighting_property_add_specular_pt3_pass_2x(
                        e, this, geometry, first, second, third, count, one, pass_flag, w14, w20,
                        w64, w38,
                    );
                    f.set_w(e, 0x28, (left - 3) as u32);
                } else if left > 1 {
                    let second = e
                        .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                        .u32();
                    let first = f.w(e, 0x50);
                    bs_shader_pp_lighting_property_add_specular_pt2_pass_2x(
                        e, this, geometry, first, second, count, one, pass_flag, w14, w20, w64, w38,
                    );
                    f.set_w(e, 0x28, (left - 2) as u32);
                } else if left > 0 {
                    let w33 = f.w(e, 0x33);
                    let first = f.w(e, 0x50);
                    bs_shader_pp_lighting_property_add_specular_pass_2x(
                        e, this, geometry, first, count, one, pass_flag, w14, 0u32, w33, w20, w64,
                        w38,
                    );
                    f.set_w(e, 0x28, (left - 1) as u32);
                }
                light = e
                    .call(NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                    .u32();
                f.set_w(e, 0x50, light);
            }
            stage = 6;
        }
        let mut mark_last = true;
        if stage == 6 {
            // `00be061d`: the envmap pass.
            if f.b(e, 0x1b) {
                let light = e
                    .call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![base, iterator])
                    .u32();
                let w80 = f.w(e, 0x80);
                let w7c = f.w(e, 0x7c);
                let w14 = f.w(e, 0x14);
                bs_shader_pp_lighting_property_add_envmap_pass_2x(
                    e, this, geometry, word, light, count, one, pass_flag, w14, w7c, w80,
                );
                mark_last = false;
            }
        }
        // `00be0685`.
        let mut closing = !mark_last;
        if mark_last {
            if !f.b(e, 0x13) {
                let list = e.mem.u32(base + 0x3c);
                if e.mem.u32(list + 0x10) != 0 && !f.b(e, 0x19) {
                    let pass = last_pass(e, this);
                    let pass_enum = e.mem.u16(pass + 4);
                    if pass_enum != 0x252 && pass_enum != 0x254 {
                        e.mem.set_u8(pass + 8, 1);
                    }
                }
            } else if !f.b(e, 0x19) {
                closing = true;
            }
        }
        if closing {
            // `00be06cb`.
            let w68 = f.w(e, 0x68);
            let w14 = f.w(e, 0x14);
            e.call(
                PASS_00BD9BC0,
                &args![base, geometry, count, one, pass_flag, w14, w64, w68],
            );
        }
        if e.mem.u32(base + 0xdc) != 0 {
            let w14 = f.w(e, 0x14);
            e.call(
                PASS_00BDC030,
                &args![base, geometry, count, one, pass_flag, w14],
            );
            hint_last(e, this, 1);
        }
        if e.global::<u8>(SETTING_011F9422) != 0 {
            if f.w(e, 0x84) != 0 {
                let w3c = f.w(e, 0x3c);
                let w14 = f.w(e, 0x14);
                e.call(PASS_00BDB380, &args![base, geometry, count, one, w14, w3c]);
            }
            if !f.b(e, 0x64)
                && !f.b(e, 0x70)
                && !f.b(e, 0x20)
                && !f.b(e, 0x60)
                && !f.b(e, 0x19)
                && !f.b(e, 0x38)
            {
                let w14 = f.w(e, 0x14);
                e.call(PASS_00BDA030, &args![base, geometry, count, one, w14]);
            }
        }
        // `00be078f`: the default accumulation hint.
        let hint: u8 = if f.b(e, 0x30) {
            0xa
        } else if f.b(e, 0x45) {
            4
        } else if f.b(e, 0x68) {
            if below_one(e, base + 0x2c) {
                0xe
            } else {
                1
            }
        } else {
            0
        };
        let list = e.mem.u32(base + 0x3c);
        if list != 0 {
            let mut index = 0u32;
            while index < e.mem.u32(e.mem.u32(base + 0x3c) + 0x10) {
                let list = e.mem.u32(base + 0x3c);
                let pass = e.mem.u32(e.mem.u32(list + 4) + 4 * index);
                if e.mem.u8(pass + 6) == 0 {
                    e.mem.set_u8(pass + 6, hint);
                }
                index += 1;
            }
        }
    });
}

/// A pass of `kind` added to the array at `array` (`RenderPassArray::Add`: geometry, kind,
/// then six words).
fn add_pass_to(e: &mut Engine, array: u32, geometry: u32, kind: u32, words: [u32; 6]) {
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![array, geometry, kind, words[0], words[1], words[2], words[3], words[4], words[5]],
    );
}

// Translated from 00bdc030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshaderpplightingproperty_passes_2x.cpp`), `thiscall`,
/// five words (a geometry, a u16 counter, a byte, a byte flag by address, a byte): with the
/// third word's byte 1 a pass 0x251 (0x250 when the fifth's byte is clear) is added to the
/// passes (+0x3c) with the flag byte, else the counter goes up; the flag byte is cleared.
/// Answers 0.
pub fn fn_00bdc030(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    counter: Ptr,
    add: u8,
    flag: Ptr,
    second: u8,
) -> u32 {
    let kind = if second != 0 { 0x251 } else { 0x250 };
    if add == 1 {
        let byte = u32::from(e.mem.u8(flag.addr()));
        let array = e.mem.u32(this.addr() + PASSES);
        add_pass_to(e, array, geometry, kind, [byte, 0, 0, 0, 0, 0]);
    } else {
        let count = e.mem.u16(counter.addr()).wrapping_add(1);
        e.mem.set_u16(counter.addr(), count);
    }
    e.mem.set_u8(flag.addr(), 0);
    0
}

// Translated from 00bdc0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshaderpplightingproperty_passes_2x.cpp`), `thiscall`,
/// thirteen words: a geometry, two words passed on, a u16 counter, a byte, a byte flag by
/// address and seven bytes (the tenth word not read). The seventh, eighth, ninth, eleventh,
/// twelfth and thirteenth bytes pick a pass kind 0xd7 to 0xe8 (some combinations add none and
/// leave the counter); with the fifth byte 1 that pass is added to the passes (+0x3c) with
/// (1, 2, the two words, 0, 0), else the counter goes up. The flag byte is cleared. Answers 0.
pub fn fn_00bdc0d0(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    first: u32,
    second: u32,
    counter: Ptr,
    add: u8,
    flag: Ptr,
    a: u8,
    b: u8,
    c: u8,
    _unused_9: u32,
    d: u8,
    f: u8,
    g: u8,
) -> u32 {
    let pick = |yes: u32, no: u32, test: u8| if test == 0 { yes } else { no };
    let kind = if a == 0 {
        if b == 0 {
            if c == 0 {
                if f == 0 {
                    Some(if d == 0 {
                        pick(0xd7, 0xdf, g)
                    } else {
                        pick(0xe0, 0xe8, g)
                    })
                } else {
                    Some(pick(0xd9, 0xe2, d))
                }
            } else {
                Some(pick(0xdb, 0xe4, d))
            }
        } else if c != 0 {
            None
        } else {
            Some(pick(0xd8, 0xe1, d))
        }
    } else if b == 0 {
        if c == 0 {
            if f != 0 {
                None
            } else {
                Some(pick(0xdc, 0xe5, d))
            }
        } else {
            Some(pick(0xde, 0xe7, d))
        }
    } else if c != 0 {
        None
    } else {
        Some(pick(0xdd, 0xe6, d))
    };
    if let Some(kind) = kind {
        if add == 1 {
            let array = e.mem.u32(this.addr() + PASSES);
            add_pass_to(e, array, geometry, kind, [1, 2, first, second, 0, 0]);
            e.mem.set_u8(flag.addr(), 0);
            return 0;
        }
        let count = e.mem.u16(counter.addr()).wrapping_add(1);
        e.mem.set_u16(counter.addr(), count);
    }
    e.mem.set_u8(flag.addr(), 0);
    0
}

// Translated from 00bde170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderPPLightingProperty::AddLocalMapPass_2x` (Xbox PDB), `thiscall`, three words (a
/// geometry, two bytes): when the array at +0x48 has no passes (+0x10), adds a pass 0x1ee
/// (third byte set), 0x1ec (second clear) or 0x1ed to it with (1, 0, 0, 0, 0, 0).
pub fn bs_shader_pp_lighting_property_add_local_map_pass_2x(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    second: u8,
    third: u8,
) {
    let array = e.mem.u32(this.addr() + 0x48);
    if e.mem.u32(array + 0x10) != 0 {
        return;
    }
    let kind = if third != 0 {
        0x1ee
    } else if second == 0 {
        0x1ec
    } else {
        0x1ed
    };
    add_pass_to(e, array, geometry, kind, [1, 0, 0, 0, 0, 0]);
}

// Translated from 00bdf6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsshaderpplightingproperty_passes_2x.cpp`), `thiscall`,
/// two words (a geometry, a byte): adds to the array at +0x54 a pass 0x22f (flag 0x4000 of
/// +0x20), 0x61 (bit 1 of +0x24), or without the byte 0x63 when the geometry's +0x9c has bit 9
/// of its u16 +0x18 set and else 0x62 (with the byte, none); then with passes in that array
/// (+0x10) its first pass's byte +6 becomes 10 (bit 1 or 2 of +0x24), 1 or 4 (flag 0x4000000
/// of +0x20, by bit 21 of +0x24) or 0.
pub fn fn_00bdf6c0(e: &mut Engine, this: Ptr, geometry: u32, skip: u8) {
    let base = this.addr();
    let kind = if e.mem.u32(base + 0x20) & 0x4000 != 0 {
        Some(0x22f)
    } else if e.mem.u8(base + 0x24) & 2 != 0 {
        Some(0x61)
    } else if skip != 0 {
        None
    } else {
        let data = e.mem.u32(geometry + 0x9c);
        if data != 0 && (e.mem.u16(data + 0x18) >> 9) & 1 != 0 {
            Some(0x63)
        } else {
            Some(0x62)
        }
    };
    if let Some(kind) = kind {
        let array = e.mem.u32(base + 0x54);
        add_pass_to(e, array, geometry, kind, [1, 0, 0, 0, 0, 0]);
    }
    let array = e.mem.u32(base + 0x54);
    if e.mem.u32(array + 0x10) == 0 {
        return;
    }
    let pass = e.mem.u32(e.mem.u32(array + 4));
    let flags = e.mem.u32(base + 0x24);
    let hint = if flags & 2 != 0 || flags & 4 != 0 {
        10
    } else if e.mem.u32(base + 0x20) & 0x400_0000 != 0 {
        if flags & 0x20_0000 != 0 {
            1
        } else {
            4
        }
    } else {
        0
    };
    e.mem.set_u8(pass + 6, hint);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_c030, fn_00bdc030(Ptr, u32, Ptr, u8, Ptr, u8) -> u32),
        entry!(
            0x00bd_c0d0,
            fn_00bdc0d0(Ptr, u32, u32, u32, Ptr, u8, Ptr, u8, u8, u8, u32, u8, u8, u8) -> u32
        ),
        entry!(
            0x00bd_e170,
            bs_shader_pp_lighting_property_add_local_map_pass_2x(Ptr, u32, u8, u8)
        ),
        entry!(0x00bd_f6c0, fn_00bdf6c0(Ptr, u32, u8)),
        entry!(
            0x00bd_bf60,
            bs_shader_pp_lighting_property_add_envmap_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_c530,
            fn_00bdc530(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_ca60,
            bs_shader_pp_lighting_property_add_amb_diff_tex2_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_d050,
            bs_shader_pp_lighting_property_add_amb_diff_tex_specular_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_d520,
            bs_shader_pp_lighting_property_add_amb_diff_tex_specular2_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_da20,
            bs_shader_pp_lighting_property_add_diffuse_pt2_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_dbc0,
            bs_shader_pp_lighting_property_add_diffuse_pt3_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_dd80,
            bs_shader_pp_lighting_property_add_texture_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_de10,
            bs_shader_pp_lighting_property_add_specular_pt2_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_dfb0,
            bs_shader_pp_lighting_property_add_specular_pt3_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_e1d0,
            bs_shader_pp_lighting_property_add_amb_diff_tex_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_e9b0,
            bs_shader_pp_lighting_property_add_amb_diff_tex_specular10_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_ef40,
            bs_shader_pp_lighting_property_add_specular_pass_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00bd_f3e0,
            bs_shader_pp_lighting_property_add_landscape_passes_2x(
                Ptr, u32, u32, u32, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00bd_f650,
            bs_shader_pp_lighting_property_add_depth_map_pass_2x(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00bd_f790,
            bs_shader_pp_lighting_property_get_render_passes_2x(Ptr, u32, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn adds(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == RENDER_PASS_ARRAY_ADD)
            .map(|c| c.1[1..].to_vec())
            .collect()
    }

    /// A property with a main and a depth-map list; the adder appends a
    /// 0x10-byte pass (enum, light count, light array) to the list.
    /// Returns (engine, this, count word, flag byte).
    fn rig() -> (Engine, Ptr, u32, u32) {
        let mut e = Engine::new();
        e.register(RENDER_PASS_ARRAY_ADD, |e, a| {
            let pass = e.mem.alloc(0x10);
            e.mem.set_u16(pass + 4, a[2] as u16);
            e.mem.set_u8(pass + 9, a[4] as u8);
            let lights = e.mem.alloc(0x40);
            e.mem.set_u32(pass + 0xc, lights);
            let size = e.mem.u32(a[0] + 0x10);
            let items = e.mem.u32(a[0] + 4);
            e.mem.set_u32(items + 4 * size, pass);
            e.mem.set_u32(a[0] + 0x10, size + 1);
            Ret::default()
        });
        let this: Ptr = Ptr::new(e.mem.alloc(0x100));
        for offset in [PASSES, DEPTH_PASSES] {
            let list = e.mem.alloc(0x14);
            let items = e.mem.alloc(0x100);
            e.mem.set_u32(list + 4, items);
            e.mem.set_u32(this.addr() + offset, list);
        }
        let count = e.mem.alloc(4);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        e.call_log = Some(vec![]);
        (e, this, count, flag)
    }

    fn count_of(e: &Engine, count: u32) -> u16 {
        e.mem.u16(count)
    }

    #[test]
    fn envmap_pass_choices() {
        let (mut e, this, count, flag) = rig();
        let f = |toggle: u32, second: u32, third: u32, over: u32| {
            args![this, 0x60u32, 0x11u32, over, count, 1u32, flag, toggle, second, third]
        };
        e.call(0x00bd_bf60, &f(1, 0, 0, 0));
        e.call(0x00bd_bf60, &f(0, 1, 0, 0x22));
        e.call(0x00bd_bf60, &f(0, 0, 1, 0));
        e.call(0x00bd_bf60, &f(0, 0, 0, 0));
        assert_eq!(
            adds(&e),
            vec![
                vec![0x60, 0x249, 1, 1, 0x11, 0, 0, 0],
                vec![0x60, 0x24b, 1, 1, 0x22, 0, 0, 0],
                vec![0x60, 0x24a, 1, 0, 0, 0, 0, 0],
                vec![0x60, 0x248, 1, 1, 0x11, 0, 0, 0],
            ]
        );
        // The pass word not 1: counted, the flag left alone.
        e.call(
            0x00bd_bf60,
            &args![this, 0x60u32, 0x11u32, 0u32, count, 2u32, flag, 0u32, 0u32, 0u32],
        );
        assert_eq!(count_of(&e, count), 1);
        assert_eq!(e.mem.u8(flag), 1);
    }

    #[test]
    fn three_light_table() {
        let (mut e, this, count, flag) = rig();
        let call = |e: &mut Engine, one: u32, f: [u32; 7]| {
            e.call(
                0x00bd_c530,
                &args![
                    this, 0x60u32, 1u32, 2u32, 3u32, count, one, flag, f[0], f[1], f[2], 0u32,
                    f[4], f[5], f[6]
                ],
            );
        };
        call(&mut e, 1, [0, 0, 0, 0, 0, 0, 0]);
        call(&mut e, 1, [1, 1, 0, 0, 1, 0, 0]);
        assert_eq!(
            adds(&e),
            vec![
                vec![0x60, 0xe9, 1, 3, 1, 2, 3, 0],
                vec![0x60, 0xf8, 1, 3, 1, 2, 3, 0],
            ]
        );
        assert_eq!(e.mem.u8(flag), 0);
        // A skipping combination: neither pass nor count.
        call(&mut e, 1, [1, 1, 1, 0, 0, 0, 0]);
        assert_eq!(count_of(&e, count), 0);
        call(&mut e, 0, [0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(count_of(&e, count), 1);
    }

    #[test]
    fn amb_diff_tex2_table() {
        let (mut e, this, count, flag) = rig();
        let call = |e: &mut Engine, f: [u32; 7]| {
            e.call(
                0x00bd_ca60,
                &args![
                    this, 0x60u32, 1u32, 2u32, count, 1u32, flag, f[0], f[1], f[2], 0u32, f[3],
                    f[4], f[5], f[6]
                ],
            );
        };
        call(&mut e, [0, 0, 0, 0, 0, 0, 0]);
        call(&mut e, [1, 0, 0, 1, 0, 0, 1]);
        assert_eq!(
            adds(&e),
            vec![
                vec![0x60, 0x116, 1, 2, 1, 2, 0, 0],
                vec![0x60, 0x12b, 1, 2, 1, 2, 0, 0]
            ]
        );
        call(&mut e, [1, 1, 1, 0, 0, 0, 0]);
        assert_eq!(count_of(&e, count), 0);
    }

    #[test]
    fn amb_diff_tex_specular_table() {
        let (mut e, this, count, flag) = rig();
        let call = |e: &mut Engine, f: [u32; 7]| {
            e.call(
                0x00bd_d050,
                &args![
                    this, 0x60u32, 5u32, count, 1u32, flag, f[0], f[1], f[2], 0u32, f[3], f[4],
                    f[5], f[6]
                ],
            );
        };
        call(&mut e, [0, 0, 0, 0, 0, 1, 0]);
        call(&mut e, [1, 1, 0, 1, 0, 0, 0]);
        assert_eq!(
            adds(&e),
            vec![
                vec![0x60, 0x9f, 1, 1, 5, 0, 0, 0],
                vec![0x60, 0xa7, 1, 1, 5, 0, 0, 0]
            ]
        );
        call(&mut e, [0, 0, 1, 0, 0, 0, 0]);
        assert_eq!(count_of(&e, count), 0);
    }

    #[test]
    fn amb_diff_tex_specular2_table_has_single_light_leaves() {
        let (mut e, this, count, flag) = rig();
        let call = |e: &mut Engine, f: [u32; 7]| {
            e.call(
                0x00bd_d520,
                &args![
                    this, 0x60u32, 1u32, 2u32, count, 1u32, flag, f[0], f[1], f[2], 0u32, f[3],
                    f[4], f[5], f[6]
                ],
            );
        };
        call(&mut e, [0, 1, 0, 0, 1, 0, 0]);
        call(&mut e, [0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            adds(&e),
            vec![
                vec![0x60, 0xb0, 1, 1, 1, 0, 0, 0],
                vec![0x60, 0xab, 1, 2, 1, 2, 0, 0]
            ]
        );
    }

    #[test]
    fn point_light_tables_use_the_flag_as_first_pass() {
        let (mut e, this, count, flag) = rig();
        e.call(
            0x00bd_da20,
            &args![this, 0x60u32, 1u32, 2u32, count, 1u32, flag, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
        e.mem.set_u8(flag, 0);
        e.call(
            0x00bd_dbc0,
            &args![
                this, 0x60u32, 1u32, 2u32, 3u32, count, 1u32, flag, 1u32, 1u32, 0u32, 0u32, 0u32
            ],
        );
        e.mem.set_u8(flag, 1);
        e.call(
            0x00bd_de10,
            &args![this, 0x60u32, 1u32, 2u32, count, 1u32, flag, 1u32, 0u32, 0u32, 1u32],
        );
        e.mem.set_u8(flag, 1);
        e.call(
            0x00bd_dfb0,
            &args![this, 0x60u32, 1u32, 2u32, 3u32, count, 1u32, flag, 0u32, 0u32, 1u32, 0u32],
        );
        assert_eq!(
            adds(&e),
            vec![
                vec![0x60, 0x134, 1, 2, 1, 2, 0, 0],
                vec![0x60, 0x142, 0, 3, 1, 2, 3, 0],
                vec![0x60, 0x170, 1, 2, 1, 2, 0, 0],
                vec![0x60, 0x177, 1, 3, 1, 2, 3, 0],
            ]
        );
        assert_eq!(e.mem.u8(flag), 0);
    }

    #[test]
    fn texture_pass_2x_and_its_1x_fallback() {
        let (mut e, this, count, flag) = rig();
        e.register(ADD_TEXTURE_PASS_1X, |_, _| 7u32.into_ret());
        let words = |use_2x: u32, skip: u32, one: u32| {
            args![
                this, 0x60u32, 2u32, count, one, flag, skip, 7u32, 8u32, 9u32, 10u32, 11u32,
                use_2x, 13u32
            ]
        };
        assert_eq!(e.call(0x00bd_dd80, &words(0, 0, 1)).u32(), 7);
        let fallback: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ADD_TEXTURE_PASS_1X)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            fallback,
            vec![vec![
                this.addr(),
                0x60,
                2,
                count,
                1,
                flag,
                0,
                7,
                8,
                9,
                10,
                13
            ]]
        );
        e.call(0x00bd_dd80, &words(1, 0, 1));
        assert_eq!(adds(&e), vec![vec![0x60, 0x153, 1, 0, 0, 0, 0, 0]]);
        e.call(0x00bd_dd80, &words(1, 1, 1));
        e.call(0x00bd_dd80, &words(1, 0, 0));
        assert_eq!(count_of(&e, count), 1);
        assert_eq!(adds(&e).len(), 1);
    }

    /// A geometry with a shader (+0xA8: flags at +0x20, alpha +0x28, fade
    /// +0x2C) and no material; the variants enabled.
    fn variant_geometry(e: &mut Engine, alpha: f32, flags: u64) -> u32 {
        e.map(0x011a_d000, 0x1000);
        e.set_global(MATERIAL_VARIANTS_011AD80D, 1u8);
        e.register(HAS_FLAG, |e, a| {
            let word = e.mem.u32(a[0] + 0x20 + (a[1] / 32) * 4);
            ((word >> (a[1] % 32)) & 1).into_ret()
        });
        let shader = e.mem.alloc(0x40);
        e.mem.set_u32(shader + 0x20, flags as u32);
        e.mem.set_u32(shader + 0x24, (flags >> 32) as u32);
        e.mem.set_f32(shader + 0x28, alpha);
        e.mem.set_f32(shader + 0x2c, alpha);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xa8, shader);
        geometry
    }

    #[test]
    fn amb_diff_tex_variants() {
        let (mut e, this, count, flag) = rig();
        let geometry = variant_geometry(&mut e, 1.0, (1 << 0x25) | (1 << 0x22));
        let call = |e: &mut Engine, f: [u32; 9]| {
            e.call(
                0x00bd_e1d0,
                &args![
                    this, geometry, 5u32, count, 1u32, flag, f[0], f[1], f[2], 0u32, f[3], f[4],
                    f[5], f[6], f[7], f[8], 0u32
                ],
            );
        };
        // The first variant: 0xFD (flag 0x22 set).
        call(&mut e, [0; 9]);
        // The second variant (f11 set): 0x102.
        call(&mut e, [0, 0, 0, 0, 1, 0, 0, 0, 0]);
        // A plain leaf.
        call(&mut e, [1, 1, 0, 1, 0, 0, 0, 0, 0]);
        let passes: Vec<u32> = adds(&e).iter().map(|w| w[1]).collect();
        assert_eq!(passes, vec![0xfd, 0x102, 0x112]);
        // A translucent shader: the plain passes.
        let geometry = variant_geometry(&mut e, 0.5, 1 << 0x25);
        e.call_log = Some(vec![]);
        e.call(
            0x00bd_e1d0,
            &args![
                this, geometry, 5u32, count, 1u32, flag, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                0u32, 0u32, 0u32, 0u32
            ],
        );
        assert_eq!(adds(&e)[0][1], 0xfb);
    }

    #[test]
    fn specular10_sets_the_lights_and_the_enum() {
        let (mut e, this, count, flag) = rig();
        let geometry = variant_geometry(&mut e, 0.5, 0);
        e.register(ACTIVE_NON_SHADOW_LIGHT_COUNT, |_, _| 7u32.into_ret());
        e.register(PASS_FREE_LIGHTS, |_, _| Ret::default());
        e.register(PASS_ALLOCATE_LIGHTS, |_, _| Ret::default());
        let lights = [0x0010_0000u32, 0x0020_0000, 0x0030_0000];
        e.register_double(FIRST_ACTIVE_NON_SHADOW_LIGHT, move |_, _| {
            lights[0].into_ret()
        });
        e.register_double(NEXT_ACTIVE_NON_SHADOW_LIGHT, {
            let mut index = 0usize;
            move |_, _| {
                index += 1;
                lights.get(index).copied().unwrap_or(0).into_ret()
            }
        });
        let pass = e
            .call(
                0x00bd_e9b0,
                &args![this, geometry, 0x99u32, count, 1u32, flag, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        // 8 lights clamped to 6 (no f8, f9, f10): the light word then three.
        assert_eq!(e.mem.u8(pass + 9), 6);
        let array = e.mem.u32(pass + 0xc);
        assert_eq!(
            [
                e.mem.u32(array),
                e.mem.u32(array + 4),
                e.mem.u32(array + 12)
            ],
            [0x99, lights[0], lights[2]]
        );
        // f7 set, f8 clear, six lights: 0xC4.
        assert_eq!(e.mem.u16(pass + 4), 0xc4);
        assert_eq!(e.mem.u8(flag), 0);
    }

    #[test]
    fn specular_pass_skips_black_lights() {
        let (mut e, this, count, flag) = rig();
        e.map(0x011f_4000, 0x1000);
        let light = e.mem.alloc(0x100);
        let colour = e.mem.alloc(0x100);
        e.mem.set_u32(light + 0xf8, colour);
        // Black: nothing.
        e.call(
            0x00bd_ef40,
            &args![this, 0x60u32, light, count, 1u32, flag, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert!(adds(&e).is_empty());
        assert_eq!(e.mem.u8(flag), 1);
        e.mem.set_f32(colour + 0xd4, 1.0);
        e.mem.set_u8(light + 0xf4, 1);
        e.call(
            0x00bd_ef40,
            &args![this, 0x60u32, light, count, 1u32, flag, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
        assert_eq!(adds(&e), vec![vec![0x60, 0x16b, 1, 1, light, 0, 0, 0]]);
    }

    #[test]
    fn landscape_passes_take_three_lit_lights() {
        let (mut e, this, count, flag) = rig();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.set_global(LANDSCAPE_EXTRA_011F94AB, 1u8);
        let make = |e: &mut Engine, lit: bool| {
            let light = e.mem.alloc(0x100);
            let colour = e.mem.alloc(0x100);
            e.mem.set_u32(light + 0xf8, colour);
            if lit {
                e.mem.set_f32(colour + 0xd4, 1.0);
            }
            light
        };
        let lights = [
            make(&mut e, false),
            make(&mut e, true),
            make(&mut e, true),
            make(&mut e, false),
            make(&mut e, true),
        ];
        e.register_double(FIRST_ACTIVE_NON_SHADOW_LIGHT, move |_, _| {
            lights[0].into_ret()
        });
        e.register_double(NEXT_ACTIVE_NON_SHADOW_LIGHT, {
            let mut index = 0usize;
            move |_, _| {
                index += 1;
                lights.get(index).copied().unwrap_or(0).into_ret()
            }
        });
        e.put_vtable(0x0090_0000, &[0; 0xf0 / 4]);
        e.mem.set_u32(0x0090_0000 + 0xec, 0x0040_00ec);
        e.register(0x0040_00ec, |_, _| 9u32.into_ret());
        e.mem.set_u32(this.addr(), 0x0090_0000);
        e.call(
            0x00bd_f3e0,
            &args![this, 0x60u32, 0x77u32, count, 1u32, flag, 0u32, 1u32],
        );
        // 7 textures (clamped), lights: 0x1FB + 56 + 1.
        assert_eq!(
            adds(&e),
            vec![
                vec![
                    0x60,
                    0x1fb + 56 + 1,
                    1,
                    4,
                    0x77,
                    lights[1],
                    lights[2],
                    lights[4]
                ],
                vec![0x60, 0x230, 0, 1, 0x77, 0, 0, 0],
            ]
        );
        let list = e.mem.u32(this.addr() + PASSES);
        let second = e.mem.u32(e.mem.u32(list + 4) + 4);
        assert_eq!(e.mem.u8(second + 0xb), 9);
        assert_eq!(e.mem.u8(flag), 0);
        // The pass word not 1: counted twice.
        e.call(
            0x00bd_f3e0,
            &args![this, 0x60u32, 0x77u32, count, 0u32, flag, 0u32, 0u32],
        );
        assert_eq!(count_of(&e, count), 2);
    }

    #[test]
    fn depth_map_pass_kinds() {
        let (mut e, this, _, _) = rig();
        let accumulator = e.mem.alloc(0x200);
        e.mem.set_u32(accumulator + 0x1a0, 0x1a01);
        e.call(0x00bd_f650, &args![this, 0x60u32, 1u32, 0u32, accumulator]);
        e.call(0x00bd_f650, &args![this, 0x60u32, 0u32, 1u32, accumulator]);
        assert_eq!(
            adds(&e),
            vec![
                vec![0x60, 0x12, 1, 1, 0x1a01, 0, 0, 0],
                vec![0x60, 0x11, 1, 1, 0x1a01, 0, 0, 0],
            ]
        );
        let depth = e.mem.u32(this.addr() + DEPTH_PASSES);
        assert_eq!(e.mem.u32(depth + 0x10), 2);
    }

    /// The rig of `GetRenderPasses_2x`: the property's flags `flags0`, its
    /// +0xB4 word, alpha and fade alpha 1, a geometry with data and
    /// property state, the accumulator (+0x194 object with the light word
    /// `0xacc0` at +0xE0), the render settings mapped, no lights.
    fn passes_2x_rig(flags0: u32) -> (Engine, Ptr, u32, u32, u32) {
        let (mut e, this, count, _) = rig();
        for page in [
            0x011f_4000u32,
            0x011f_9000,
            0x011f_d000,
            0x010a_9000,
            0x011a_d000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(PASS_MASK_010A91D4, 0xfu32);
        e.mem.set_u32(this.addr() + 0x20, flags0);
        let b4 = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + 0xb4, b4);
        e.mem.set_f32(this.addr() + 0x28, 1.0);
        e.mem.set_f32(this.addr() + 0x2c, 1.0);
        let geometry = e.mem.alloc(0xc0);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(geometry + 0xb8, data);
        let property_state = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0x9c, property_state);
        let accumulator = e.mem.alloc(0x200);
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object + 0xe0, 0xacc0);
        let holder = e.mem.alloc(0x200);
        e.mem.set_u32(holder + 0x194, object);
        e.register_double(GET_ACCUMULATOR, move |_, _| holder.into_ret());
        e.register(GET_PROPERTY, |_, _| Ret::default());
        e.register(KIND_OF, |_, _| Ret::default());
        e.register(ACTIVE_NON_SHADOW_LIGHT_COUNT, |_, _| Ret::default());
        e.register(FIRST_ACTIVE_NON_SHADOW_LIGHT, |_, _| Ret::default());
        e.register(NEXT_ACTIVE_NON_SHADOW_LIGHT, |_, _| Ret::default());
        e.register(DEPTH_PASSES_WANTED, |_, _| Ret::default());
        e.register(HAS_FLAG, |e, a| {
            let word = e.mem.u32(a[0] + 0x20 + (a[1] / 32) * 4);
            ((word >> (a[1] % 32)) & 1).into_ret()
        });
        // The pass helpers of other units append a pass with their own
        // address as its enum (low 16 bits).
        for helper in [
            PASS_00BD9F00,
            PASS_00BD9F90,
            PASS_00BD9E60,
            PASS_00BDC0D0,
            PASS_00BD9540,
            PASS_00BD9840,
            PASS_00BD9BC0,
            PASS_00BDC030,
            PASS_00BDB380,
            PASS_00BDA030,
            PASS_00BD9DA0,
        ] {
            e.register_double(helper, move |e, a| {
                let list = e.mem.u32(a[0] + PASSES);
                let pass = e.mem.alloc(0x10);
                e.mem.set_u16(pass + 4, helper as u16);
                let size = e.mem.u32(list + 0x10);
                let items = e.mem.u32(list + 4);
                e.mem.set_u32(items + 4 * size, pass);
                e.mem.set_u32(list + 0x10, size + 1);
                Ret::default()
            });
        }
        e.call_log = Some(vec![]);
        (e, this, geometry, count, accumulator)
    }

    fn pass_enums(e: &Engine, this: Ptr) -> Vec<(u16, u8, u8)> {
        let list = e.mem.u32(this.addr() + PASSES);
        (0..e.mem.u32(list + 0x10))
            .map(|i| {
                let pass = e.mem.u32(e.mem.u32(list + 4) + 4 * i);
                (e.mem.u16(pass + 4), e.mem.u8(pass + 6), e.mem.u8(pass + 8))
            })
            .collect()
    }

    #[test]
    fn get_render_passes_2x_hands_bit_15_to_00bd9da0() {
        let (mut e, this, geometry, count, accumulator) = passes_2x_rig(0x8000);
        e.call(
            0x00bd_f790,
            &args![this, geometry, 1u32, count, 0u32, accumulator, 1u32],
        );
        assert_eq!(pass_enums(&e, this), vec![(0x9da0, 6, 0)]);
    }

    #[test]
    fn get_render_passes_2x_hands_kinds_to_slot_0x114() {
        let (mut e, this, geometry, count, accumulator) = passes_2x_rig(0);
        e.register(KIND_OF, |_, a| {
            if a[1] == RTTI_012024C8 { a[0] } else { 0 }.into_ret()
        });
        e.put_vtable(0x0090_0000, &[0; 0x118 / 4]);
        e.mem.set_u32(0x0090_0000 + 0x114, 0x0040_0114);
        e.register(0x0040_0114, |_, _| Ret::default());
        e.mem.set_u32(this.addr(), 0x0090_0000);
        e.mem.set_u32(accumulator + 0x19c, 0x19c1);
        e.call(
            0x00bd_f790,
            &args![this, geometry, 5u32, count, 0u32, accumulator, 1u32],
        );
        let calls: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x0040_0114)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            calls,
            vec![vec![
                this.addr(),
                geometry,
                5,
                count,
                0x19c1,
                accumulator,
                1
            ]]
        );
    }

    #[test]
    fn get_render_passes_2x_simple_property_marks_and_hints() {
        // Mask bit 0 only: `00bd9540`, then the last pass is marked and
        // every pass without a hint gets 0xA (the accumulator's +0x34 is 2).
        let (mut e, this, geometry, count, accumulator) = passes_2x_rig(0);
        e.mem.set_u32(accumulator + 0x34, 2);
        e.call(
            0x00bd_f790,
            &args![this, geometry, 1u32, count, 0u32, accumulator, 1u32],
        );
        assert_eq!(pass_enums(&e, this), vec![(0x9540, 0xa, 1)]);
    }

    #[test]
    fn get_render_passes_2x_ambient_table_ends_the_list() {
        // Kind 3, mask bits 0, 1 and 2, a kind-of match on `01200788` and
        // the property's bit 11: the ambient-diffuse table without lights
        // (`AddAmbDiffTexPass_2x` with f11 set: its second variant, off
        // without the material variants: 0x101), then straight to the end.
        let (mut e, this, geometry, count, accumulator) = passes_2x_rig(0x800);
        e.register(KIND_OF, |_, a| {
            if a[1] == RTTI_01200788 { a[0] } else { 0 }.into_ret()
        });
        e.call(
            0x00bd_f790,
            &args![this, geometry, 7u32, count, 3u32, accumulator, 1u32],
        );
        let enums: Vec<u16> = pass_enums(&e, this).iter().map(|p| p.0).collect();
        assert_eq!(enums, vec![0x101]);
        // The last pass is marked; no hint condition holds.
        assert_eq!(pass_enums(&e, this)[0], (0x101, 0, 1));
    }

    #[test]
    fn get_render_passes_2x_point_lights_and_texture_pass() {
        // Mask bits 1 and 2 (not 0): straight to the point-light part
        // (`00bd9840`, no lights) and the 2x texture pass (bit 11), the last
        // pass marked.
        let (mut e, this, geometry, count, accumulator) = passes_2x_rig(0x800);
        e.call(
            0x00bd_f790,
            &args![this, geometry, 6u32, count, 0u32, accumulator, 1u32],
        );
        assert_eq!(pass_enums(&e, this), vec![(0x9840, 0, 0), (0x153, 0, 1)]);
    }
}

#[cfg(test)]
mod n070_tests {
    use super::*;

    fn adds(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == RENDER_PASS_ARRAY_ADD)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        for (offset, array) in [(PASSES, 0x3c00u32), (0x48, 0), (0x54, 0)] {
            let value = if array != 0 { array } else { e.mem.alloc(0x14) };
            e.mem.set_u32(this + offset, value);
        }
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_00bdc030_adds_or_counts() {
        let (mut e, this) = rig();
        let counter = e.mem.alloc(4);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 3);
        e.call(
            0x00bd_c030,
            &args![this, 0x77u32, counter, 1u32, flag, 1u32],
        );
        assert_eq!(adds(&e), vec![vec![0x3c00, 0x77, 0x251, 3, 0, 0, 0, 0, 0]]);
        assert_eq!(e.mem.u8(flag), 0);
        e.call(
            0x00bd_c030,
            &args![this, 0x77u32, counter, 1u32, flag, 0u32],
        );
        assert_eq!(adds(&e)[1][2], 0x250);
        e.call(
            0x00bd_c030,
            &args![this, 0x77u32, counter, 0u32, flag, 0u32],
        );
        assert_eq!(e.mem.u16(counter), 1);
        assert_eq!(adds(&e).len(), 2);
    }

    #[test]
    fn fn_00bdc0d0_picks_the_kind() {
        let (mut e, this) = rig();
        let counter = e.mem.alloc(4);
        let flag = e.mem.alloc(4);
        // (a, b, c, d, f, g) -> kind (0 = none).
        let cases: [([u32; 6], u32); 14] = [
            ([0, 0, 0, 0, 0, 0], 0xd7),
            ([0, 0, 0, 0, 0, 1], 0xdf),
            ([0, 0, 0, 1, 0, 0], 0xe0),
            ([0, 0, 0, 1, 0, 1], 0xe8),
            ([0, 0, 0, 0, 1, 0], 0xd9),
            ([0, 0, 0, 1, 1, 0], 0xe2),
            ([0, 0, 1, 0, 0, 0], 0xdb),
            ([0, 0, 1, 1, 0, 0], 0xe4),
            ([0, 1, 0, 1, 0, 0], 0xe1),
            ([0, 1, 1, 0, 0, 0], 0),
            ([1, 0, 0, 0, 0, 0], 0xdc),
            ([1, 0, 0, 0, 1, 0], 0),
            ([1, 0, 1, 1, 0, 0], 0xe7),
            ([1, 1, 0, 1, 0, 0], 0xe6),
        ];
        for ([a, b, c, d, f, g], kind) in cases {
            e.call_log = Some(vec![]);
            e.mem.set_u8(flag, 1);
            e.call(
                0x00bd_c0d0,
                &args![
                    this, 0x77u32, 0x11u32, 0x22u32, counter, 1u32, flag, a, b, c, 0u32, d, f, g
                ],
            );
            let added = adds(&e);
            if kind == 0 {
                assert!(added.is_empty());
            } else {
                assert_eq!(
                    added,
                    vec![vec![0x3c00, 0x77, kind, 1, 2, 0x11, 0x22, 0, 0]]
                );
            }
            assert_eq!(e.mem.u8(flag), 0);
        }
        // Without the add byte: the counter, but not for a combination without a kind.
        e.call(
            0x00bd_c0d0,
            &args![
                this, 0u32, 0u32, 0u32, counter, 0u32, flag, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                0u32
            ],
        );
        e.call(
            0x00bd_c0d0,
            &args![
                this, 0u32, 0u32, 0u32, counter, 0u32, flag, 0u32, 1u32, 1u32, 0u32, 0u32, 0u32,
                0u32
            ],
        );
        assert_eq!(e.mem.u16(counter), 1);
    }

    #[test]
    fn add_local_map_pass_2x_only_into_an_empty_array() {
        let (mut e, this) = rig();
        let array = e.mem.u32(this + 0x48);
        for ((second, third), kind) in [((0u32, 1u32), 0x1ee), ((0, 0), 0x1ec), ((1, 0), 0x1ed)] {
            e.call(0x00bd_e170, &args![this, 0x77u32, second, third]);
            assert_eq!(
                adds(&e).last().unwrap(),
                &vec![array, 0x77, kind, 1, 0, 0, 0, 0, 0]
            );
        }
        e.mem.set_u32(array + 0x10, 1);
        e.call(0x00bd_e170, &args![this, 0x77u32, 0u32, 0u32]);
        assert_eq!(adds(&e).len(), 3);
    }

    #[test]
    fn fn_00bdf6c0_adds_and_sets_the_hint() {
        let (mut e, this) = rig();
        let array = e.mem.u32(this + 0x54);
        let pass = e.mem.alloc(8);
        let passes = e.mem.alloc(4);
        e.mem.set_u32(passes, pass);
        e.mem.set_u32(array + 4, passes);
        let geometry = e.mem.alloc(0xa0);
        let data = e.mem.alloc(0x1c);
        e.mem.set_u16(data + 0x18, 0x200);
        e.mem.set_u32(geometry + 0x9c, data);
        e.call(0x00bd_f6c0, &args![this, geometry, 0u32]);
        assert_eq!(adds(&e)[0][2], 0x63);
        // No passes in the array: the hint stays.
        assert_eq!(e.mem.u8(pass + 6), 0);
        e.mem.set_u32(array + 0x10, 1);
        e.mem.set_u16(data + 0x18, 0);
        e.mem.set_u32(this + 0x20, 0x400_0000);
        e.mem.set_u32(this + 0x24, 0x20_0000);
        e.call(0x00bd_f6c0, &args![this, geometry, 0u32]);
        assert_eq!(adds(&e)[1][2], 0x62);
        assert_eq!(e.mem.u8(pass + 6), 1);
        e.mem.set_u32(this + 0x24, 0);
        e.call(0x00bd_f6c0, &args![this, geometry, 1u32]);
        assert_eq!(adds(&e).len(), 2);
        assert_eq!(e.mem.u8(pass + 6), 4);
        e.mem.set_u32(this + 0x24, 2);
        e.call(0x00bd_f6c0, &args![this, geometry, 1u32]);
        assert_eq!(adds(&e)[2][2], 0x61);
        assert_eq!(e.mem.u8(pass + 6), 10);
        e.mem.set_u32(this + 0x20, 0x4000);
        e.mem.set_u32(this + 0x24, 0);
        e.call(0x00bd_f6c0, &args![this, geometry, 1u32]);
        assert_eq!(adds(&e)[3][2], 0x22f);
        assert_eq!(e.mem.u8(pass + 6), 0);
    }
}
