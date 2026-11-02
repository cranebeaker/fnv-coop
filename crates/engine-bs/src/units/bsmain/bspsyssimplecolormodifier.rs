//! `BSMain/bspsyssimplecolormodifier.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31650` (named `NiPSysModifier::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C31650: u32 = 0x00c3_1650;

// Translated from 00c60910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSPSysSimpleColorModifier` vtable word 010c33d4, right
/// after `BSPSysSimpleColorModifier::LoadBinary` (010c33d0). Forwards `this` and the stream to
/// `00c31650` (`NiPSysModifier::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c60910(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C31650, &args![this, stream]);
}

/// `00c31670` (named `NiPSysModifier::RegisterStreamables` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00C31670: u32 = 0x00c3_1670;

// Translated from 00c60930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSPSysSimpleColorModifier` vtable word 010c33d8, after the
/// `LinkObject` word 010c33d4 and before `BSPSysSimpleColorModifier::SaveBinary` (010c33dc).
/// Forwards `this` and the stream to `00c31670` (`NiPSysModifier::RegisterStreamables`).
/// Returns the `bool` in `AL`: Xbox PDB `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z`
/// returns `_N`; `stream` is the `NiStream` reference as an address. Parameter count from the
/// `RET 4` of both bodies.
pub fn fn_00c60930(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00C31670, &args![this, stream])
        .bool()
}

// Translated from 00c60d40 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01204390 without reading the receiver. The map names this body `BSPSysSimpleColorModifier::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c60d40(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_4390
}

/// The vtable of `BSPSysSimpleColorModifier` (the constructor stores it).
const BS_PSYS_SIMPLE_COLOR_MODIFIER_VTABLE: u32 = 0x010c_3384;
/// `BSPSysSimpleColorModifier` size (Xbox PDB and the allocations).
const BS_PSYS_SIMPLE_COLOR_MODIFIER_SIZE: u32 = 0x60;
/// `BSPSysSimpleColorModifier` fields (Xbox PDB names; PC offsets as the
/// code uses them): `kColor1` +0x18, `kColor2` +0x28, `kColor3` +0x38
/// (`NiColorA`, alpha at +0x0c of each), `fFadeInPct` +0x48,
/// `fFadeOutPct` +0x4c, `fColor1EndPct` +0x50, `fColor2StartPct` +0x54,
/// `fColor2EndPct` +0x58 and `fColor3StartPct` +0x5c.
const COLOR1: u32 = 0x18;
const COLOR2: u32 = 0x28;
const COLOR3: u32 = 0x38;
const FADE_IN: u32 = 0x48;
const FADE_OUT: u32 = 0x4c;
const COLOR1_END: u32 = 0x50;
const COLOR2_START: u32 = 0x54;
const COLOR2_END: u32 = 0x58;
const COLOR3_START: u32 = 0x5c;
const ALPHA: u32 = 0x0c;
/// The particle system data `Update` reads: the colour array +0x28 (16
/// bytes per particle), the active count (16 bits) +0x50 and the particle
/// info array +0x70 (0x1c bytes each, the age +0x0c and the life span
/// +0x10).
const DATA_COLORS: u32 = 0x28;
const DATA_ACTIVE: u32 = 0x50;
const DATA_PARTICLE_INFO: u32 = 0x70;
const PARTICLE_INFO_SIZE: u32 = 0x1c;
const PARTICLE_AGE: u32 = 0x0c;
const PARTICLE_LIFE_SPAN: u32 = 0x10;
/// `0.0` (`double`, `.rdata`), the zero the percentages are compared with.
const ZERO: u32 = 0x0101_2060;

/// `00c31720` (unit `nipsysmodifier.cpp`): the `NiPSysModifier` constructor
/// body; `NiPSysModifier::~NiPSysModifier` (`00c31750`); `00c31780` (same
/// unit, `thiscall`, `(destination, cloning)`): its member copy;
/// `NiPSysModifier::LoadBinary` (`00c318b0`).
const PSYS_MODIFIER_CONSTRUCT: u32 = 0x00c3_1720;
const PSYS_MODIFIER_DESTRUCT: u32 = 0x00c3_1750;
const PSYS_MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const PSYS_MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
/// `00a26df0` (unnamed in the engine map; `cdecl`, `(stream, &float)`):
/// reads one float.
const STREAM_READ_FLOAT: u32 = 0x00a2_6df0;
/// `NiColorA::LoadBinary` (`00a69800`, `thiscall` on the colour).
const COLOR_A_LOAD_BINARY: u32 = 0x00a6_9800;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00c60240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSPSysSimpleColorModifier` constructor.
/// The `NiPSysModifier` constructor body (`00c31720`), the vtable, then the
/// three colours set to 0.0 (x87 `FLDZ`/`FSTP`). The percentages are left
/// as they are. Returns `this`.
pub fn fn_00c60240(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PSYS_MODIFIER_CONSTRUCT, &args![this]);
    let object = this.addr();
    e.mem.set_u32(object, BS_PSYS_SIMPLE_COLOR_MODIFIER_VTABLE);
    for color in [COLOR1, COLOR2, COLOR3] {
        for k in 0..4 {
            e.mem.set_f32(object.wrapping_add(color + 4 * k), 0.0);
        }
    }
    this
}

/// `a - b` for an x87 load and a float memory operand.
fn sub_m32(a: u32, b: u32) -> f64 {
    x87_sub(x87_load32(a), x87_m32(b))
}

/// `(to - from) * t + from` per component, x87, each stored as a float.
fn lerp(e: &Engine, this: u32, from: u32, to: u32, t: u32, k: u32) -> u32 {
    let from = e.mem.u32(this.wrapping_add(from + 4 * k));
    let to = e.mem.u32(this.wrapping_add(to + 4 * k));
    let value = x87_mul(sub_m32(to, from), x87_m32(t));
    x87_store32(x87_add(value, x87_m32(from)))
}

// Translated from 00c602e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysSimpleColorModifier::Update` (Xbox PDB): colours each active
/// particle of `data` (when it has a colour array) by its age fraction
/// (age / life span, x87, stored as a float). RGB: before colour 2 starts
/// (when the colour 1 to 2 span is not zero) colour 1 until colour 1 ends,
/// then blended toward colour 2; otherwise colour 2 until it ends (or
/// always when the colour 2 to 3 span is zero), blended toward colour 3
/// until colour 3 starts, then colour 3. Alpha: past a non-zero fade-out
/// point blended from colour 2's to colour 3's alpha over the rest of the
/// life; before a non-zero fade-in point blended from colour 1's to colour
/// 2's; otherwise colour 2's. Comparisons follow the exe's flag tests,
/// NaN included (a NaN fraction takes colour 2 and its alpha). The first
/// argument word (the time) is not read.
pub fn bs_psys_simple_color_modifier_update(e: &mut Engine, this: Ptr, _unused_1: u32, data: Ptr) {
    let this = this.addr();
    let data = data.addr();
    let colors = e.mem.u32(data.wrapping_add(DATA_COLORS));
    if colors == 0 {
        return;
    }
    let field = |e: &Engine, offset: u32| e.mem.u32(this.wrapping_add(offset));
    let zero = e.mem.f64(ZERO);
    let span1 = x87_store32(sub_m32(field(e, COLOR2_START), field(e, COLOR1_END)));
    let span2 = x87_store32(sub_m32(field(e, COLOR3_START), field(e, COLOR2_END)));
    let count = e.mem.u16(data.wrapping_add(DATA_ACTIVE));
    let mut i = 0u16;
    while u32::from(i) < u32::from(count) {
        let info = e
            .mem
            .u32(data.wrapping_add(DATA_PARTICLE_INFO))
            .wrapping_add(u32::from(i) * PARTICLE_INFO_SIZE);
        let age_bits = x87_store32(x87_div(
            x87_load32(e.mem.u32(info.wrapping_add(PARTICLE_AGE))),
            x87_m32(e.mem.u32(info.wrapping_add(PARTICLE_LIFE_SPAN))),
        ));
        let age = x87_load32(age_bits);
        let out = colors.wrapping_add(u32::from(i) * 16);
        let copy = |e: &mut Engine, color: u32| {
            for k in 0..3 {
                let value = x87_quiet_bits(field(e, color + 4 * k));
                e.mem.set_u32(out.wrapping_add(4 * k), value);
            }
        };
        let in_first_part = x87_load32(span1) != zero && x87_load32(field(e, COLOR2_START)) > age;
        if in_first_part {
            if x87_load32(field(e, COLOR1_END)) > age {
                copy(e, COLOR1);
            } else {
                let t = x87_store32(x87_div(
                    sub_m32(age_bits, field(e, COLOR1_END)),
                    x87_m32(span1),
                ));
                for k in 0..3 {
                    let value = lerp(e, this, COLOR1, COLOR2, t, k);
                    e.mem.set_u32(out.wrapping_add(4 * k), value);
                }
            }
        } else if x87_load32(span2) == zero || !(x87_load32(field(e, COLOR2_END)) < age) {
            copy(e, COLOR2);
        } else if x87_load32(field(e, COLOR3_START)) < age {
            copy(e, COLOR3);
        } else {
            let t = x87_store32(x87_div(
                sub_m32(age_bits, field(e, COLOR2_END)),
                x87_m32(span2),
            ));
            for k in 0..3 {
                let value = lerp(e, this, COLOR2, COLOR3, t, k);
                e.mem.set_u32(out.wrapping_add(4 * k), value);
            }
        }
        let fade_out = x87_load32(field(e, FADE_OUT));
        let fade_in = x87_load32(field(e, FADE_IN));
        let alpha = if fade_out != zero && fade_out < age {
            let rest = x87_sub(1.0, x87_load32(field(e, FADE_OUT)));
            let t = x87_store32(x87_div(sub_m32(age_bits, field(e, FADE_OUT)), rest));
            lerp(e, this, COLOR2 + ALPHA, COLOR3 + ALPHA, t, 0)
        } else if fade_in != zero && fade_in > age {
            let t = x87_store32(x87_div(x87_m32(age_bits), fade_in));
            lerp(e, this, COLOR1 + ALPHA, COLOR2 + ALPHA, t, 0)
        } else {
            x87_quiet_bits(field(e, COLOR2 + ALPHA))
        };
        e.mem.set_u32(out.wrapping_add(ALPHA), alpha);
        i = i.wrapping_add(1);
    }
}

// Translated from 00c60670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c60670 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30042, state variable) is not translated
/// `BSPSysSimpleColorModifier::CreateClone` (Xbox PDB): allocates 0x60
/// bytes, constructs a modifier in it when that succeeds (`00c60240`), runs
/// the member copy (`00c60700`) into it (into address 0 when the
/// allocation failed) and returns it.
pub fn bs_psys_simple_color_modifier_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![BS_PSYS_SIMPLE_COLOR_MODIFIER_SIZE])
        .u32();
    let object = if block == 0 {
        Ptr::new(0)
    } else {
        fn_00c60240(e, Ptr::new(block))
    };
    fn_00c60700(e, this, object, cloning);
    object
}

// Translated from 00c60700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSPSysSimpleColorModifier`'s member copy
/// into `destination` for a clone made by `cloning`. The `NiPSysModifier`
/// copy (`00c31780`), the three colours word by word, then the six
/// percentages through the x87 stack.
pub fn fn_00c60700(e: &mut Engine, this: Ptr, destination: Ptr, cloning: u32) {
    e.call(
        PSYS_MODIFIER_COPY_MEMBERS,
        &args![this, destination, cloning],
    );
    let source = this.addr();
    let destination = destination.addr();
    for offset in (COLOR1..FADE_IN).step_by(4) {
        let word = e.mem.u32(source.wrapping_add(offset));
        e.mem.set_u32(destination.wrapping_add(offset), word);
    }
    for offset in (FADE_IN..BS_PSYS_SIMPLE_COLOR_MODIFIER_SIZE).step_by(4) {
        let value = x87_quiet_bits(e.mem.u32(source.wrapping_add(offset)));
        e.mem.set_u32(destination.wrapping_add(offset), value);
    }
}

// Translated from 00c607d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c607d0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f3006c, state variable) is not translated
/// Unnamed in the engine map (the `BSPSysSimpleColorModifier` factory, no
/// arguments): allocates 0x60 bytes and constructs a modifier in it
/// (`00c60240`) when that succeeds. Returns it, or 0.
pub fn fn_00c607d0(e: &mut Engine) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![BS_PSYS_SIMPLE_COLOR_MODIFIER_SIZE])
        .u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00c60240(e, Ptr::new(block))
}

// Translated from 00c60850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysSimpleColorModifier::LoadBinary` (Xbox PDB):
/// `NiPSysModifier::LoadBinary`, the six percentages (`00a26df0` each, in
/// field order), then the three colours (`NiColorA::LoadBinary`).
pub fn bs_psys_simple_color_modifier_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PSYS_MODIFIER_LOAD_BINARY, &args![this, stream]);
    let object = this.addr();
    for offset in (FADE_IN..BS_PSYS_SIMPLE_COLOR_MODIFIER_SIZE).step_by(4) {
        e.call(
            STREAM_READ_FLOAT,
            &args![stream, object.wrapping_add(offset)],
        );
    }
    for color in [COLOR1, COLOR2, COLOR3] {
        e.call(
            COLOR_A_LOAD_BINARY,
            &args![object.wrapping_add(color), stream],
        );
    }
}

// Translated from 00c60d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysSimpleColorModifier::_scalar_deleting_destructor_` (Xbox PDB):
/// `NiPSysModifier::~NiPSysModifier`, then the sized delete of 0x60 bytes
/// when bit 0 of `flags` is set. Returns `this`.
pub fn bs_psys_simple_color_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(PSYS_MODIFIER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(
            NI_SIZED_DELETE,
            &args![this, BS_PSYS_SIMPLE_COLOR_MODIFIER_SIZE],
        );
    }
    this
}

/// `NiPSysModifier::SaveBinary` (`00c31950`), `NiPSysModifier::IsEqual` (`00c31830`) and
/// `NiPSysModifier::GetViewerStrings` (`00c319e0`), `thiscall` with one word.
const PSYS_MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const PSYS_MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const PSYS_MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// `NiStreamSaveBinary<float>` (`00a3da50`, `cdecl`, `(stream, &float)`).
const STREAM_SAVE_FLOAT: u32 = 0x00a3_da50;
/// `NiColorA::SaveBinary` (`00a698a0`, `thiscall` on the colour, `(stream)`), the colour
/// comparison `004a4cc0` (`thiscall` on a colour, `(&other)`, answers a byte) and
/// `NiColorA::GetViewerString` (`00a69720`, `thiscall` on the colour, `(label)`).
const COLOR_A_SAVE_BINARY: u32 = 0x00a6_98a0;
const COLOR_A_EQUAL: u32 = 0x004a_4cc0;
const COLOR_A_GET_VIEWER_STRING: u32 = 0x00a6_9720;
/// `NiGetViewerString` (`00a7d330`, `(name)`), its `(label, float)` form `00a7d550` and the
/// viewer-string array's add (`0096a610`, `thiscall`, `(&string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// The class's `ms_RTTI` (its first word is the class name) and the double the
/// percentages are shown multiplied by (`01017a40`, 100.0 in the exe).
const RTTI: u32 = 0x0120_4390;
const PERCENT_SCALE: u32 = 0x0101_7a40;
/// The viewer labels of the percentages (`fColor1EndPct`, `fColor2StartPct`,
/// `fColor2EndPct`, `fColor3StartPct`, `fFadeInPct`, `fFadeOutPct`, in the code's order)
/// and of the three colours.
const COLOR1_END_LABEL: u32 = 0x010c_3494;
const COLOR2_START_LABEL: u32 = 0x010c_3480;
const COLOR2_END_LABEL: u32 = 0x010c_3470;
const COLOR3_START_LABEL: u32 = 0x010c_345c;
const FADE_IN_LABEL: u32 = 0x010c_3450;
const FADE_OUT_LABEL: u32 = 0x010c_3440;
const COLOR1_LABEL: u32 = 0x010c_3438;
const COLOR2_LABEL: u32 = 0x010c_3430;
const COLOR3_LABEL: u32 = 0x010c_3428;

// Translated from 00c60950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysSimpleColorModifier::SaveBinary` (Xbox PDB, engine map): `NiPSysModifier::SaveBinary`,
/// the six percentages from `fFadeInPct` to `fColor3StartPct` (`NiStreamSaveBinary<float>`,
/// in memory order), then the three colours (`NiColorA::SaveBinary`).
pub fn bs_psys_simple_color_modifier_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PSYS_MODIFIER_SAVE_BINARY, &args![this, stream]);
    let object = this.addr();
    for field in [
        FADE_IN,
        FADE_OUT,
        COLOR1_END,
        COLOR2_START,
        COLOR2_END,
        COLOR3_START,
    ] {
        e.call(
            STREAM_SAVE_FLOAT,
            &args![stream, object.wrapping_add(field)],
        );
    }
    for color in [COLOR1, COLOR2, COLOR3] {
        e.call(
            COLOR_A_SAVE_BINARY,
            &args![object.wrapping_add(color), stream],
        );
    }
}

// Translated from 00c60a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysSimpleColorModifier::IsEqual` (Xbox PDB, engine map): `NiPSysModifier::IsEqual`,
/// then the six percentages in memory order (x87 `FUCOMPP`: unordered is not equal) and
/// the three colours (`004a4cc0` on the other's colour with this one's), all equal.
pub fn bs_psys_simple_color_modifier_is_equal(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if e.call(PSYS_MODIFIER_IS_EQUAL, &args![this, other]).u8() == 0 {
        return false;
    }
    let object = this.addr();
    for field in [
        FADE_IN,
        FADE_OUT,
        COLOR1_END,
        COLOR2_START,
        COLOR2_END,
        COLOR3_START,
    ] {
        let theirs = x87_load32(e.mem.u32(other.wrapping_add(field)));
        let mine = x87_load32(e.mem.u32(object.wrapping_add(field)));
        if mine != theirs {
            return false;
        }
    }
    for color in [COLOR1, COLOR2, COLOR3] {
        let equal = e
            .call(
                COLOR_A_EQUAL,
                &args![other.wrapping_add(color), object.wrapping_add(color)],
            )
            .u8();
        if equal == 0 {
            return false;
        }
    }
    true
}

// Translated from 00c60b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysSimpleColorModifier::GetViewerStrings` (Xbox PDB, engine map):
/// `NiPSysModifier::GetViewerStrings`, the class name, each percentage times the double at
/// `01017a40` (x87 `FMUL`, stored as a `float`) with its label, then each colour's own
/// viewer string (`NiColorA::GetViewerString`), each added to the list.
pub fn bs_psys_simple_color_modifier_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(PSYS_MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let object = this.addr();
    // The ten string locals, from EBP-0x28 (the last) to EBP-4 (the first).
    e.with_stack(0x28, |e, slots| {
        let mut slot = slots.addr().wrapping_add(0x24);
        let name = e.global::<u32>(RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
        e.mem.set_u32(slot, text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        let scale = e.mem.f64(PERCENT_SCALE);
        for (field, label) in [
            (COLOR1_END, COLOR1_END_LABEL),
            (COLOR2_START, COLOR2_START_LABEL),
            (COLOR2_END, COLOR2_END_LABEL),
            (COLOR3_START, COLOR3_START_LABEL),
            (FADE_IN, FADE_IN_LABEL),
            (FADE_OUT, FADE_OUT_LABEL),
        ] {
            let value = x87_mul(x87_load32(e.mem.u32(object.wrapping_add(field))), scale);
            let shown = x87_quiet_bits(x87_store32(value));
            let text = e
                .call(NI_GET_VIEWER_STRING_FLOAT, &args![label, shown])
                .u32();
            slot = slot.wrapping_sub(4);
            e.mem.set_u32(slot, text);
            e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        }
        for (color, label) in [
            (COLOR1, COLOR1_LABEL),
            (COLOR2, COLOR2_LABEL),
            (COLOR3, COLOR3_LABEL),
        ] {
            let text = e
                .call(
                    COLOR_A_GET_VIEWER_STRING,
                    &args![object.wrapping_add(color), label],
                )
                .u32();
            slot = slot.wrapping_sub(4);
            e.mem.set_u32(slot, text);
            e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c6_0950,
            bs_psys_simple_color_modifier_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c6_0a10,
            bs_psys_simple_color_modifier_is_equal(Ptr, u32) -> bool
        ),
        entry!(
            0x00c6_0b20,
            bs_psys_simple_color_modifier_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c6_0d40, fn_00c60d40(Ptr) -> u32),
        entry!(0x00c6_0910, fn_00c60910(Ptr, Ptr)),
        entry!(0x00c6_0930, fn_00c60930(Ptr, Ptr) -> bool),
        entry!(0x00c6_0240, fn_00c60240(Ptr) -> Ptr),
        entry!(
            0x00c6_02e0,
            bs_psys_simple_color_modifier_update(Ptr, u32, Ptr)
        ),
        entry!(0x00c6_0670, bs_psys_simple_color_modifier_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00c6_0700, fn_00c60700(Ptr, Ptr, u32)),
        entry!(0x00c6_07d0, fn_00c607d0() -> Ptr),
        entry!(
            0x00c6_0850,
            bs_psys_simple_color_modifier_load_binary(Ptr, Ptr)
        ),
        entry!(0x00c6_0d10, bs_psys_simple_color_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c60910_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C31650, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_0910, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C31650)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c60930_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00C31670, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c6_0930, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00C31670)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c60d40_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_0d40, &args![0x1000u32]).u32(), 0x0120_4390);
        assert_eq!(e.call(0x00c6_0d40, &args![0u32]).u32(), 0x0120_4390);
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    #[test]
    fn constructor_sets_the_vtable_and_clears_the_colours() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, PSYS_MODIFIER_CONSTRUCT, 0);
        let this = e.mem.alloc(0x60);
        e.mem.write(this, &[0xaa; 0x60]);
        assert_eq!(e.call(0x00c6_0240, &args![this]).u32(), this);
        assert_eq!(*log.borrow(), [(PSYS_MODIFIER_CONSTRUCT, vec![this])]);
        assert_eq!(e.mem.u32(this), BS_PSYS_SIMPLE_COLOR_MODIFIER_VTABLE);
        assert_eq!(e.mem.bytes(this + 0x18, 0x30), vec![0; 0x30]);
        assert_eq!(e.mem.bytes(this + 0x48, 0x18), vec![0xaa; 0x18]);
    }

    /// A modifier with exact binary percentages and colours 1, 2, 3 (alphas
    /// 0.5, 1.0, 0.0), and particle data whose particles have the given
    /// (age, life span) pairs.
    fn modifier(e: &mut Engine, particles: &[(f32, f32)]) -> (u32, u32, u32) {
        e.map(ZERO, 8);
        let this = e.mem.alloc(0x60);
        for (offset, value) in [
            (FADE_IN, 0.125f32),
            (FADE_OUT, 0.875),
            (COLOR1_END, 0.25),
            (COLOR2_START, 0.5),
            (COLOR2_END, 0.625),
            (COLOR3_START, 0.75),
        ] {
            e.mem.set_f32(this + offset, value);
        }
        for (color, value, alpha) in [
            (COLOR1, 1.0f32, 0.5f32),
            (COLOR2, 2.0, 1.0),
            (COLOR3, 3.0, 0.0),
        ] {
            for k in 0..3 {
                e.mem.set_f32(this + color + 4 * k, value);
            }
            e.mem.set_f32(this + color + ALPHA, alpha);
        }
        let data = e.mem.alloc(0x74);
        let colors = e.mem.alloc(16 * particles.len() as u32);
        let info = e.mem.alloc(0x1c * particles.len() as u32);
        for (i, (age, life)) in particles.iter().enumerate() {
            e.mem.set_f32(info + 0x1c * i as u32 + PARTICLE_AGE, *age);
            e.mem
                .set_f32(info + 0x1c * i as u32 + PARTICLE_LIFE_SPAN, *life);
        }
        e.mem.set_u32(data + DATA_COLORS, colors);
        e.mem.set_u16(data + DATA_ACTIVE, particles.len() as u16);
        e.mem.set_u32(data + DATA_PARTICLE_INFO, info);
        (this, data, colors)
    }

    #[test]
    fn update_colours_each_particle_by_its_age() {
        let mut e = Engine::new();
        let particles = [
            (1.0, 16.0),
            (6.0, 16.0),
            (9.0, 16.0),
            (11.0, 16.0),
            (15.0, 16.0),
            (0.0, 0.0),
        ];
        let (this, data, colors) = modifier(&mut e, &particles);
        e.call(0x00c6_02e0, &args![this, 0u32, data]);
        let expected = [
            (1.0f32, 0.75f32),
            (1.5, 1.0),
            (2.0, 1.0),
            (2.5, 1.0),
            (3.0, 0.5),
            (2.0, 1.0),
        ];
        for (i, (rgb, alpha)) in expected.iter().enumerate() {
            let out = colors + 16 * i as u32;
            for k in 0..3 {
                assert_eq!(e.mem.f32(out + 4 * k), *rgb, "particle {i}");
            }
            assert_eq!(e.mem.f32(out + ALPHA), *alpha, "particle {i}");
        }
        // No colour array: nothing is written.
        e.mem.set_u32(data + DATA_COLORS, 0);
        e.mem.set_f32(colors, 7.0);
        e.call(0x00c6_02e0, &args![this, 0u32, data]);
        assert_eq!(e.mem.f32(colors), 7.0);
    }

    #[test]
    fn update_with_a_zero_first_span_starts_at_colour_2() {
        let mut e = Engine::new();
        let (this, data, colors) = modifier(&mut e, &[(1.0, 16.0)]);
        e.mem.set_f32(this + COLOR2_START, 0.25);
        e.mem.set_f32(this + FADE_IN, 0.0);
        e.call(0x00c6_02e0, &args![this, 0u32, data]);
        assert_eq!(e.mem.f32(colors), 2.0);
        assert_eq!(e.mem.f32(colors + ALPHA), 1.0);
    }

    #[test]
    fn copy_members_copies_colours_and_percentages() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, PSYS_MODIFIER_COPY_MEMBERS, 0);
        let this = e.mem.alloc(0x60);
        for b in 0x18..0x60 {
            e.mem.set_u8(this + b, b as u8);
        }
        e.mem.set_u32(this + FADE_IN, 0x7f80_0001);
        let destination = e.mem.alloc(0x60);
        e.call(0x00c6_0700, &args![this, destination, 0x9000u32]);
        assert_eq!(
            *log.borrow(),
            [(PSYS_MODIFIER_COPY_MEMBERS, vec![this, destination, 0x9000])]
        );
        assert_eq!(
            e.mem.bytes(destination + 0x18, 0x30),
            e.mem.bytes(this + 0x18, 0x30)
        );
        assert_eq!(e.mem.u32(destination + FADE_IN), 0x7fc0_0001);
        assert_eq!(
            e.mem.bytes(destination + 0x4c, 0x14),
            e.mem.bytes(this + 0x4c, 0x14)
        );
    }

    #[test]
    fn create_object_and_create_clone_build_modifiers() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x60);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, PSYS_MODIFIER_CONSTRUCT, 0);
        logged(&mut e, &log, PSYS_MODIFIER_COPY_MEMBERS, 0);
        assert_eq!(e.call(0x00c6_07d0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), BS_PSYS_SIMPLE_COLOR_MODIFIER_VTABLE);
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00c6_0670, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(
            log.borrow().last().unwrap(),
            &(PSYS_MODIFIER_COPY_MEMBERS, vec![this, block, 0x9000])
        );
        e.map(0, 0x100);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00c6_07d0, &args![]).u32(), 0);
        assert_eq!(e.call(0x00c6_0670, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x60]),
                (NI_ALLOCATE, vec![0x60]),
                (PSYS_MODIFIER_COPY_MEMBERS, vec![this, 0, 0x9000]),
            ]
        );
    }

    #[test]
    fn load_binary_reads_the_percentages_then_the_colours() {
        let mut e = Engine::new();
        let log = Log::default();
        for address in [
            PSYS_MODIFIER_LOAD_BINARY,
            STREAM_READ_FLOAT,
            COLOR_A_LOAD_BINARY,
        ] {
            logged(&mut e, &log, address, 0);
        }
        let this = e.mem.alloc(0x60);
        e.call(0x00c6_0850, &args![this, 0x5000u32]);
        let log = log.borrow();
        assert_eq!(log[0], (PSYS_MODIFIER_LOAD_BINARY, vec![this, 0x5000]));
        for k in 0..6u32 {
            assert_eq!(
                log[1 + k as usize],
                (STREAM_READ_FLOAT, vec![0x5000, this + 0x48 + 4 * k])
            );
        }
        assert_eq!(log[7], (COLOR_A_LOAD_BINARY, vec![this + 0x18, 0x5000]));
        assert_eq!(log[9], (COLOR_A_LOAD_BINARY, vec![this + 0x38, 0x5000]));
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, PSYS_MODIFIER_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        assert_eq!(e.call(0x00c6_0d10, &args![0x5000u32, 1u32]).u32(), 0x5000);
        e.call(0x00c6_0d10, &args![0x5000u32, 2u32]);
        assert_eq!(
            *log.borrow(),
            [
                (PSYS_MODIFIER_DESTRUCT, vec![0x5000]),
                (NI_SIZED_DELETE, vec![0x5000, 0x60]),
                (PSYS_MODIFIER_DESTRUCT, vec![0x5000]),
            ]
        );
    }
}

#[cfg(test)]
mod n068_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    const PERCENTAGES: [u32; 6] = [
        FADE_IN,
        FADE_OUT,
        COLOR1_END,
        COLOR2_START,
        COLOR2_END,
        COLOR3_START,
    ];

    fn modifier(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x60);
        for (i, field) in (1u32..).zip(PERCENTAGES) {
            e.mem.set_f32(this + field, i as f32 * 0.125);
        }
        this
    }

    #[test]
    fn save_binary_writes_percentages_then_colours() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, PSYS_MODIFIER_SAVE_BINARY, 0);
        logged(&mut e, &log, STREAM_SAVE_FLOAT, 0);
        logged(&mut e, &log, COLOR_A_SAVE_BINARY, 0);
        e.call(0x00c6_0950, &args![0x1000u32, 0x6000u32]);
        let mut expected = vec![(PSYS_MODIFIER_SAVE_BINARY, vec![0x1000, 0x6000])];
        for field in PERCENTAGES {
            expected.push((STREAM_SAVE_FLOAT, vec![0x6000, 0x1000 + field]));
        }
        for color in [COLOR1, COLOR2, COLOR3] {
            expected.push((COLOR_A_SAVE_BINARY, vec![0x1000 + color, 0x6000]));
        }
        assert_eq!(*log.borrow(), expected);
    }

    #[test]
    fn is_equal_compares_percentages_then_colours() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, PSYS_MODIFIER_IS_EQUAL, 1);
        logged(&mut e, &log, COLOR_A_EQUAL, 1);
        let this = modifier(&mut e);
        let other = modifier(&mut e);
        assert!(e.call(0x00c6_0a10, &args![this, other]).bool());
        assert_eq!(
            calls(&log, COLOR_A_EQUAL),
            vec![
                vec![other + COLOR1, this + COLOR1],
                vec![other + COLOR2, this + COLOR2],
                vec![other + COLOR3, this + COLOR3],
            ]
        );
        for field in PERCENTAGES {
            let kept = e.mem.u32(other + field);
            e.mem.set_u32(other + field, 0x7fc0_0000);
            assert!(!e.call(0x00c6_0a10, &args![this, other]).bool());
            e.mem.set_u32(other + field, kept);
        }
        logged(&mut e, &log, COLOR_A_EQUAL, 0x100);
        assert!(!e.call(0x00c6_0a10, &args![this, other]).bool());
        logged(&mut e, &log, COLOR_A_EQUAL, 1);
        logged(&mut e, &log, PSYS_MODIFIER_IS_EQUAL, 0);
        assert!(!e.call(0x00c6_0a10, &args![this, other]).bool());
    }

    #[test]
    fn viewer_strings_show_percentages_scaled_and_the_colours() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, PSYS_MODIFIER_GET_VIEWER_STRINGS, 0);
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0101_0000);
        e.map(PERCENT_SCALE, 8);
        e.mem.set_f64(PERCENT_SCALE, 100.0);
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        logged(&mut e, &log, NI_GET_VIEWER_STRING_FLOAT, 0x7000);
        e.register(COLOR_A_GET_VIEWER_STRING, |_, a| (a[0] + a[1]).into_ret());
        let l = log.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            l.borrow_mut().push((0, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        let this = modifier(&mut e);
        e.call(0x00c6_0b20, &args![this, 0x6000u32]);
        let shown: Vec<(u32, f32)> = calls(&log, NI_GET_VIEWER_STRING_FLOAT)
            .iter()
            .map(|c| (c[0], f32::from_bits(c[1])))
            .collect();
        assert_eq!(
            shown,
            vec![
                (COLOR1_END_LABEL, 37.5),
                (COLOR2_START_LABEL, 50.0),
                (COLOR2_END_LABEL, 62.5),
                (COLOR3_START_LABEL, 75.0),
                (FADE_IN_LABEL, 12.5),
                (FADE_OUT_LABEL, 25.0),
            ]
        );
        let added = calls(&log, 0);
        assert_eq!(added.len(), 10);
        assert_eq!(added[0], vec![0x6000, 0x0101_0001]);
        assert_eq!(added[7], vec![0x6000, this + COLOR1 + COLOR1_LABEL]);
        assert_eq!(added[9], vec![0x6000, this + COLOR3 + COLOR3_LABEL]);
    }
}
