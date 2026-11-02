//! `BSShader/run_00b6f8d0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::bsshaderproperty::BSShaderProperty;

/// `BSShaderProperty::SetAlpha` (`thiscall`, one word: the alpha's bits).
const SET_ALPHA: u32 = 0x00ba_8ab0;
/// `BSShaderProperty::CheckCreateRenderPassArray` (`thiscall`, one word: the size).
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
/// `BSShaderProperty::RenderPassArray::Add` (`thiscall`, eight words: geometry, pass enum,
/// first-pass flag, light count, four light words).
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// `004a2020` (`thiscall`, one word: a bit number): whether that bit of the flag words at
/// `this + 0x20` is set.
const FLAG_BIT_SET: u32 = 0x004a_2020;
/// `BSStripParticleSystem::ms_RTTI` (`01202560`), compared with the geometry's `GetRTTI`.
const STRIP_PARTICLE_SYSTEM_RTTI: u32 = 0x0120_2560;

/// Whether bit `bit` of the property's flag words is set (`004a2020`).
fn flag_bit(e: &mut Engine, this: Ptr<BSShaderProperty>, bit: u32) -> bool {
    e.call(FLAG_BIT_SET, &args![this, bit]).bool()
}

// Translated from 00b6f5c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra made no function of it): slot +0xa0 of
/// `BSShaderNoLightingProperty`'s vtable (`010ae670`), `thiscall`, `RET 0x18` (geometry,
/// a render state word, an unread word, a state byte, the render state block, an unread
/// word). Builds the property's render-pass list for `geometry` and returns it
/// (`pRenderPassList`, +0x3c), or 0 when the state byte is 7.
///
/// - The state key is `state_byte << 8 | state_word`.
/// - When `geometry + 0x9c` is not null, the alpha is set (`SetAlpha`) to `fFadeAlpha`
///   (+0x2c), times the float at +0x3c of the object at `geometry + 0xa4` when that is not
///   null.
/// - When the key equals `iLastRenderPassState` (+0x38) the list is returned as it is.
///   Otherwise the list is made (`CheckCreateRenderPassArray(1)`), emptied (`iSize` 0), and
///   one pass added (one first-pass flag, no lights). For a geometry whose `GetRTTI`
///   (slot +8) is `BSStripParticleSystem`'s it is `0x56`, or `0x57` when the byte at +0x6c
///   of the object at `geometry + 0xb8` is non-zero. Otherwise, when the geometry's slot
///   +0x30 answers non-zero, it is `0x52`/`0x54` (that byte zero; flag bit 0x33 clear/set)
///   or `0x53`/`0x55` (byte non-zero). Otherwise the pass comes from flag bit 1 and,
///   when the property's word at +0x60 is non-zero, flag bit 3, with
///   `ulFlags[1]` bit 0x20 (`b`) and `ulFlags[0]` bit 0x40 (`a`, which also marks the pass
///   as needing alpha below): with +0x60 set, `0x42` (`a` clear) or `0x4a` (`a` set) when
///   neither bit 3 nor `b` is set, else `0x44`/`0x4c`; with +0x60 clear, `0x43`/`0x4b`
///   when `b` is set, else `0x45`/`0x4d`; each plus 4 when flag bit 1 is set.
/// - The new pass (the array's last element) gets its byte +6: 10 when the geometry's
///   `+0x9c` object has bit 0 of its byte +0x18 clear and either the block's word +0x34 is
///   2, `ulFlags[1]` bit 4 is set, or flag bit 0x21 is set; else 4 when `ulFlags[0]` has
///   bit 0x4000000, 5 when bit 0x8000000, 3 when `iShaderPropertyType` (+0x1c) is 0x11
///   and the word at +0x114 is 8 (beyond `BSShaderNoLightingProperty`'s 0x80 bytes; read
///   only for that type); else 1 when `fAlpha` is below 1.0 (`FCOMP`: not when NaN), the
///   byte +0x18 bit is set, the geometry's slot +0x30 answers non-zero, or `a` was set.
///   Otherwise it is left as it is.
/// - The key is stored in `iLastRenderPassState`.
pub fn fn_00b6f5c0(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    state_word: u32,
    _unused_3: u32,
    state_byte: u32,
    render_state: u32,
    _unused_6: u32,
) -> u32 {
    let key = (state_byte << 8) | state_word;
    if state_byte == 7 {
        return 0;
    }
    let geometry_state = geometry.wrapping_add(0x9c);
    if geometry_state != 0 {
        let alpha_object = e.mem.u32(geometry_state.wrapping_add(8));
        let fade = e.get(this, BSShaderProperty::fFadeAlpha);
        let alpha = if alpha_object != 0 {
            let factor = x87_load32(e.mem.u32(alpha_object + 0x3c));
            x87_store32(x87_mul(factor, x87_m32(fade)))
        } else {
            x87_quiet_bits(fade)
        };
        e.call(SET_ALPHA, &args![this, alpha]);
    }
    if e.get(this, BSShaderProperty::iLastRenderPassState) as u32 == key {
        return e.get(this, BSShaderProperty::pRenderPassList);
    }

    e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![this, 1u32]);
    let list = e.get(this, BSShaderProperty::pRenderPassList);
    // BSShaderProperty::RenderPassArray::iSize (Xbox PDB) +0x10.
    e.mem.set_u32(list + 0x10, 0);

    let mut needs_alpha = false;
    let strip_particles =
        geometry != 0 && e.vcall(geometry, 8, &args![]).u32() == STRIP_PARTICLE_SYSTEM_RTTI;
    let pass = if strip_particles {
        let object = e.mem.u32(geometry + 0xb8);
        u32::from(e.mem.u8(object + 0x6c) != 0) + 0x56
    } else if e.vcall(geometry, 0x30, &args![]).u32() != 0 {
        let object = e.mem.u32(geometry + 0xb8);
        let byte_set = e.mem.u8(object + 0x6c) != 0;
        let bit = u32::from(flag_bit(e, this, 0x33));
        if byte_set {
            bit * 2 + 0x53
        } else {
            bit * 2 + 0x52
        }
    } else {
        let b = e.get(this, BSShaderProperty::ulFlags1) & 0x20 != 0;
        let a = e.get(this, BSShaderProperty::ulFlags0) & 0x40 != 0;
        let has_word_60 = e.mem.u32(this.addr() + 0x60) != 0;
        let base = if has_word_60 {
            let bit3 = flag_bit(e, this, 3);
            if !bit3 && !b {
                0x42
            } else {
                0x44
            }
        } else if b {
            0x43
        } else {
            0x45
        };
        let bit1 = u32::from(flag_bit(e, this, 1));
        if a {
            needs_alpha = true;
            bit1 * 4 + base + 8
        } else {
            bit1 * 4 + base
        }
    };
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );

    let list = e.get(this, BSShaderProperty::pRenderPassList);
    let size = e.mem.u32(list + 0x10);
    // NiTArray::m_pBase (Xbox PDB) +4: the pass just added is the last element.
    let base = e.mem.u32(list + 4);
    let state_object = e.mem.u32(geometry.wrapping_add(0x9c));
    let state_bit = e.mem.u8(state_object + 0x18) & 1 != 0;
    let pass_object = e
        .mem
        .u32(base.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));

    let mut value = None;
    if !state_bit
        && (e.mem.u32(render_state + 0x34) == 2
            || e.get(this, BSShaderProperty::ulFlags1) & 4 != 0
            || flag_bit(e, this, 0x21))
    {
        value = Some(10);
    }
    if value.is_none() {
        let flags = e.get(this, BSShaderProperty::ulFlags0);
        value = if flags & 0x0400_0000 != 0 {
            Some(4)
        } else if flags & 0x0800_0000 != 0 {
            Some(5)
        } else if e.get(this, BSShaderProperty::iShaderPropertyType) == 0x11
            && e.mem.u32(this.addr() + 0x114) == 8
        {
            Some(3)
        } else {
            let alpha = f64::from(f32::from_bits(e.get(this, BSShaderProperty::fAlpha)));
            // FLD1; FCOMP fAlpha; TEST AH,0x41: taken (pass 1) only when 1.0 > fAlpha.
            if 1.0 > alpha
                || state_bit
                || e.vcall(geometry, 0x30, &args![]).u32() != 0
                || needs_alpha
            {
                Some(1)
            } else {
                None
            }
        };
    }
    if let Some(value) = value {
        e.mem.set_u8(pass_object + 6, value);
    }
    e.set(this, BSShaderProperty::iLastRenderPassState, key as i32);
    e.get(this, BSShaderProperty::pRenderPassList)
}

/// `NiSourceTexture::ms_RTTI` (`011f444c`) and `NiRenderedTexture::ms_RTTI` (`011f4ac4`), the
/// records `fn_00b6f8d0` looks for in its texture's RTTI chain.
const SOURCE_TEXTURE_RTTI: u32 = 0x011f_444c;
const RENDERED_TEXTURE_RTTI: u32 = 0x011f_4ac4;
/// `00ba97b0` (unit `bsshaderproperty.cpp`; `thiscall`, one word): makes sure the water-fog
/// pass list exists, as `fn_00b6f930` calls it with 1.
const CHECK_CREATE_WATER_FOG_PASS_ARRAY_00BA97B0: u32 = 0x00ba_97b0;

/// Walks an `NiRTTI` chain (the parent at `+4`) from `rtti`: whether `wanted` is in it.
fn rtti_chain_has(e: &Engine, rtti: u32, wanted: u32) -> bool {
    let mut rtti = rtti;
    while rtti != 0 {
        if rtti == wanted {
            return true;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    false
}

// Translated from 00b6f8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0xbc of `BSShaderNoLightingProperty`'s vtable `010ae670`
/// (`thiscall`, two words: a `cdecl` callback taking three words, and a word passed through to
/// it). When the base map (`+0x60`, `spBaseMap` in the Xbox PDB) is set, its RTTI chain (its
/// slot +8, the parents at `+4`) is searched for `NiSourceTexture` and, when not found,
/// searched again (with a second `GetRTTI`) for `NiRenderedTexture` (that result is not used);
/// either way the callback gets the texture, 0 and the word.
pub fn fn_00b6f8d0(e: &mut Engine, this: Ptr, callback: u32, data: u32) {
    let base = this.addr();
    let texture = e.mem.u32(base.wrapping_add(0x60));
    if texture == 0 {
        return;
    }
    let rtti = e.vcall(texture, 8, &args![]).u32();
    if !rtti_chain_has(e, rtti, SOURCE_TEXTURE_RTTI) {
        let texture = e.mem.u32(base.wrapping_add(0x60));
        if texture != 0 {
            let rtti = e.vcall(texture, 8, &args![]).u32();
            rtti_chain_has(e, rtti, RENDERED_TEXTURE_RTTI);
        }
    }
    let texture = e.mem.u32(base.wrapping_add(0x60));
    e.call(callback, &args![texture, 0u32, data]);
}

// Translated from 00b6f930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra made no function of it): slot +0xb8 of
/// `BSShaderNoLightingProperty`'s vtable `010ae670` (`GetWaterFogPassList` in the Xbox PDB's
/// slot list; `thiscall`, one word: the geometry). Makes sure the water-fog pass list exists
/// (`00ba97b0` with 1); when it is empty (`iSize`, +0x10 of the list at `+0x50`) adds one pass
/// for the geometry (`RenderPassArray::Add` with six zero words after it):
/// - with bit 0x20 of the byte `+0x24` (`ulFlags[1]`): 0x2ef/0x2ee when bit 9 of the 16-bit
///   flags at +0x18 of the geometry's property at `+0x9c` is set (a null property counts as
///   clear), else 0x2e9/0x2e8;
/// - without it: 0x2e3/0x2e2, else 0x2dd/0x2dc;
///
/// the higher number of each pair when bit 2 of the byte `+0x20` (`ulFlags[0]`) is set.
/// Answers the list.
pub fn fn_00b6f930(e: &mut Engine, this: Ptr<BSShaderProperty>, geometry: u32) -> u32 {
    let property = e.mem.u32(geometry.wrapping_add(0x9c));
    let bit9 = property != 0 && (e.mem.u16(property.wrapping_add(0x18)) >> 9) as u8 & 1 != 0;
    let flagged = e.mem.u8(this.addr().wrapping_add(0x24)) & 0x20 != 0;
    e.call(
        CHECK_CREATE_WATER_FOG_PASS_ARRAY_00BA97B0,
        &args![this, 1u32],
    );
    let list = e.get(this, BSShaderProperty::pWaterFogRenderPassList);
    if e.mem.u32(list.wrapping_add(0x10)) != 0 {
        return e.get(this, BSShaderProperty::pWaterFogRenderPassList);
    }
    let high = e.mem.u8(this.addr().wrapping_add(0x20)) & 2 != 0;
    let pass: u32 = match (flagged, bit9, high) {
        (true, true, true) => 0x2ef,
        (true, true, false) => 0x2ee,
        (true, false, true) => 0x2e9,
        (true, false, false) => 0x2e8,
        (false, true, true) => 0x2e3,
        (false, true, false) => 0x2e2,
        (false, false, true) => 0x2dd,
        (false, false, false) => 0x2dc,
    };
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, pass, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    e.get(this, BSShaderProperty::pWaterFogRenderPassList)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b6_f5c0,
            fn_00b6f5c0(Ptr<BSShaderProperty>, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00b6_f8d0, fn_00b6f8d0(Ptr, u32, u32)),
        entry!(0x00b6_f930, fn_00b6f930(Ptr<BSShaderProperty>, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct World {
        e: Engine,
        this: u32,
        geometry: u32,
        render_state: u32,
        pass: u32,
        list: u32,
    }

    const RTTI_SLOT: u32 = 0x0000_5008;
    const SLOT_30: u32 = 0x0000_5030;

    /// A property with a one-pass list, a geometry whose `+0x9c` object, `+0xa4` alpha
    /// object and `+0xb8` object exist, and the callees doubled: `SetAlpha` stores the bits
    /// at +0x28, `Add` appends nothing (the list already holds its one pass with `iSize` 1
    /// after `Add`), bit tests read the real flag words, the geometry's RTTI is `rtti` and its
    /// slot +0x30 answers `slot30`.
    fn world(rtti: u32, slot30: u32) -> World {
        let mut e = Engine::new();
        e.register(SET_ALPHA, |e, a| {
            e.mem.set_u32(a[0] + 0x28, a[1]);
            Ret::default()
        });
        e.register(CHECK_CREATE_RENDER_PASS_ARRAY, |_, _| Ret::default());
        e.register(RENDER_PASS_ARRAY_ADD, |e, a| {
            e.mem.set_u32(a[0] + 0x10, 1);
            Ret::default()
        });
        e.register(FLAG_BIT_SET, |e, a| {
            let word = e.mem.u32(a[0] + 0x20 + (a[1] >> 5) * 4);
            (word & (1 << (a[1] % 32)) != 0).into_ret()
        });
        e.register_double(RTTI_SLOT, move |_, _| rtti.into_ret());
        e.register_double(SLOT_30, move |_, _| slot30.into_ret());
        let this = e.mem.alloc(0x120);
        let pass = e.mem.alloc(0x10);
        let list = e.mem.alloc(0x14);
        let elements = e.mem.alloc(4);
        e.mem.set_u32(elements, pass);
        e.mem.set_u32(list + 4, elements);
        e.mem.set_u32(this + 0x3c, list);
        e.mem.set_u32(this + 0x38, 0xffff_ffff);
        e.mem.set_f32(this + 0x28, 1.0);
        e.mem.set_f32(this + 0x2c, 1.0);
        let geometry = e.mem.alloc(0xc0);
        let vtable = e.mem.alloc(0x34);
        e.mem.set_u32(vtable + 8, RTTI_SLOT);
        e.mem.set_u32(vtable + 0x30, SLOT_30);
        e.mem.set_u32(geometry, vtable);
        let state_object = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0x9c, state_object);
        let b8 = e.mem.alloc(0x70);
        e.mem.set_u32(geometry + 0xb8, b8);
        let render_state = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        World {
            e,
            this,
            geometry,
            render_state,
            pass,
            list,
        }
    }

    fn run(w: &mut World, word: u32, byte: u32) -> u32 {
        let (this, geometry, render_state) = (w.this, w.geometry, w.render_state);
        w.e.call(
            0x00b6_f5c0,
            &[this, geometry, word, 0, byte, render_state, 0],
        )
        .u32()
    }

    fn added_pass(w: &mut World) -> Option<u32> {
        w.e.call_log
            .take()
            .unwrap()
            .into_iter()
            .find(|(a, _)| *a == RENDER_PASS_ARRAY_ADD)
            .map(|(_, words)| {
                assert_eq!(words[0], w.list);
                assert_eq!(words[1], w.geometry);
                assert_eq!(&words[3..], &[1, 0, 0, 0, 0, 0]);
                words[2]
            })
    }

    #[test]
    fn state_byte_7_returns_0_and_does_nothing() {
        let mut w = world(0, 0);
        assert_eq!(run(&mut w, 3, 7), 0);
        let log = w.e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn alpha_is_fade_alpha_times_the_alpha_objects_factor() {
        let mut w = world(0, 0);
        let alpha_object = w.e.mem.alloc(0x40);
        w.e.mem.set_f32(alpha_object + 0x3c, 0.25);
        w.e.mem.set_u32(w.geometry + 0xa4, alpha_object);
        w.e.mem.set_u32(w.this + 0x38, 0x0203);
        assert_eq!(run(&mut w, 3, 2), w.list);
        assert_eq!(w.e.mem.f32(w.this + 0x28), 0.25);
        // The same key: nothing is rebuilt.
        assert_eq!(added_pass(&mut w), None);

        // Without the alpha object the fade alpha itself (a signalling NaN comes back quiet).
        let mut w = world(0, 0);
        w.e.mem.set_u32(w.this + 0x2c, 0x7f80_0001);
        w.e.mem.set_u32(w.this + 0x38, 0x0203);
        run(&mut w, 3, 2);
        assert_eq!(w.e.mem.u32(w.this + 0x28), 0x7fc0_0001);
    }

    #[test]
    fn strip_particle_systems_get_56_or_57() {
        for (byte, pass) in [(0u8, 0x56u32), (1, 0x57)] {
            let mut w = world(STRIP_PARTICLE_SYSTEM_RTTI, 0);
            let b8 = w.e.mem.u32(w.geometry + 0xb8);
            w.e.mem.set_u8(b8 + 0x6c, byte);
            assert_eq!(run(&mut w, 0x11, 0x22), w.list);
            assert_eq!(added_pass(&mut w), Some(pass));
            assert_eq!(w.e.mem.u32(w.this + 0x38), 0x2211);
            assert_eq!(w.e.mem.u32(w.this + 0x3c), w.list);
        }
    }

    #[test]
    fn slot_30_geometries_get_52_to_55() {
        for (byte, bit, pass) in [
            (0u8, false, 0x52u32),
            (0, true, 0x54),
            (1, false, 0x53),
            (1, true, 0x55),
        ] {
            let mut w = world(0x1234, 1);
            let b8 = w.e.mem.u32(w.geometry + 0xb8);
            w.e.mem.set_u8(b8 + 0x6c, byte);
            if bit {
                // Bit 0x33: word 1 (+0x24), bit 0x13.
                w.e.mem.set_u32(w.this + 0x24, 1 << 0x13);
            }
            run(&mut w, 0, 0);
            assert_eq!(added_pass(&mut w), Some(pass));
            // Slot +0x30 answering non-zero marks the pass with 1.
            assert_eq!(w.e.mem.u8(w.pass + 6), 1);
        }
    }

    #[test]
    fn other_geometries_get_42_to_51() {
        // (word +0x60, bit 3, b = ulFlags[1] 0x20, a = ulFlags[0] 0x40, bit 1) -> pass.
        let cases = [
            (1u32, false, false, false, false, 0x42u32),
            (1, true, false, false, false, 0x44),
            (1, false, true, false, false, 0x44),
            (1, false, false, true, false, 0x4a),
            (1, true, false, true, true, 0x50),
            (0, false, true, false, false, 0x43),
            (0, false, false, false, false, 0x45),
            (0, false, true, true, false, 0x4b),
            (0, false, false, true, true, 0x51),
            (0, false, false, false, true, 0x49),
        ];
        for (word60, bit3, b, a, bit1, pass) in cases {
            let mut w = world(0, 0);
            w.e.mem.set_u32(w.this + 0x60, word60);
            let mut flags0 = 0u32;
            if bit3 {
                flags0 |= 8;
            }
            if bit1 {
                flags0 |= 2;
            }
            if a {
                flags0 |= 0x40;
            }
            w.e.mem.set_u32(w.this + 0x20, flags0);
            w.e.mem.set_u32(w.this + 0x24, if b { 0x20 } else { 0 });
            run(&mut w, 0, 0);
            assert_eq!(
                added_pass(&mut w),
                Some(pass),
                "{word60} {bit3} {b} {a} {bit1}"
            );
            // `a` marks the pass with 1; otherwise (alpha 1.0, slot +0x30 zero) it stays.
            assert_eq!(w.e.mem.u8(w.pass + 6), if a { 1 } else { 0 });
        }
    }

    #[test]
    fn the_pass_byte_follows_the_state_and_flags() {
        // Byte +0x18 bit clear and the block's +0x34 is 2: 10.
        let mut w = world(0, 0);
        w.e.mem.set_u32(w.render_state + 0x34, 2);
        run(&mut w, 0, 0);
        assert_eq!(w.e.mem.u8(w.pass + 6), 10);
        // ulFlags[1] bit 4: 10.
        let mut w = world(0, 0);
        w.e.mem.set_u32(w.this + 0x24, 4);
        run(&mut w, 0, 0);
        assert_eq!(w.e.mem.u8(w.pass + 6), 10);
        // Flag bit 0x21 (word 1 bit 1): 10.
        let mut w = world(0, 0);
        w.e.mem.set_u32(w.this + 0x24, 2);
        run(&mut w, 0, 0);
        assert_eq!(w.e.mem.u8(w.pass + 6), 10);
        // With the +0x18 bit set the 10 cases fall through: bit 0x4000000 gives 4.
        let mut w = world(0, 0);
        let state_object = w.e.mem.u32(w.geometry + 0x9c);
        w.e.mem.set_u8(state_object + 0x18, 1);
        w.e.mem.set_u32(w.render_state + 0x34, 2);
        w.e.mem.set_u32(w.this + 0x20, 0x0400_0000);
        run(&mut w, 0, 0);
        assert_eq!(w.e.mem.u8(w.pass + 6), 4);
        // Bit 0x8000000: 5.
        let mut w = world(0, 0);
        w.e.mem.set_u32(w.this + 0x20, 0x0800_0000);
        run(&mut w, 0, 0);
        assert_eq!(w.e.mem.u8(w.pass + 6), 5);
        // Type 0x11 with 8 at +0x114: 3; with another word: falls to the alpha test.
        let mut w = world(0, 0);
        w.e.mem.set_u32(w.this + 0x1c, 0x11);
        w.e.mem.set_u32(w.this + 0x114, 8);
        run(&mut w, 0, 0);
        assert_eq!(w.e.mem.u8(w.pass + 6), 3);
        let mut w = world(0, 0);
        w.e.mem.set_u32(w.this + 0x1c, 0x11);
        w.e.mem.set_u32(w.this + 0x114, 9);
        run(&mut w, 0, 0);
        assert_eq!(w.e.mem.u8(w.pass + 6), 0);
        // The +0x18 bit alone (no other reason): 1.
        let mut w = world(0, 0);
        let state_object = w.e.mem.u32(w.geometry + 0x9c);
        w.e.mem.set_u8(state_object + 0x18, 1);
        run(&mut w, 0, 0);
        assert_eq!(w.e.mem.u8(w.pass + 6), 1);
    }

    #[test]
    fn alpha_below_one_marks_the_pass_but_nan_does_not() {
        // SetAlpha stores the fade alpha (no alpha object), so the fade alpha decides.
        for (fade, marked) in [
            (0.5f32, true),
            (1.0, false),
            (2.0, false),
            (f32::NAN, false),
        ] {
            let mut w = world(0, 0);
            w.e.mem.set_f32(w.this + 0x2c, fade);
            run(&mut w, 0, 0);
            assert_eq!(w.e.mem.u8(w.pass + 6), u8::from(marked), "{fade}");
        }
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    const CALLBACK: u32 = 0x0f00_0100;
    const GET_RTTI: u32 = 0x0f00_0108;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00b6f8d0_hands_out_the_base_map() {
        let mut e = Engine::new();
        e.put_vtable(0x0f00_1000, &[0, 0, GET_RTTI]);
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(CALLBACK, |_, _| Ret::default());
        let own = e.mem.alloc(8);
        e.mem.set_u32(own + 4, SOURCE_TEXTURE_RTTI);
        let texture = e.mem.alloc(0x10);
        e.mem.set_u32(texture, 0x0f00_1000);
        e.mem.set_u32(texture + 8, own);
        let this = e.mem.alloc(0x88);
        e.mem.set_u32(this + 0x60, texture);
        e.call_log = Some(vec![]);
        e.call(0x00b6_f8d0, &args![this, CALLBACK, 9u32]);
        assert_eq!(calls(&e, CALLBACK), vec![vec![texture, 0, 9]]);
        assert_eq!(calls(&e, GET_RTTI).len(), 1);
        // Neither record in the chain: asked twice, still handed out.
        e.mem.set_u32(own + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b6_f8d0, &args![this, CALLBACK, 9u32]);
        assert_eq!(calls(&e, GET_RTTI).len(), 2);
        assert_eq!(calls(&e, CALLBACK).len(), 1);
        // No base map: nothing.
        e.mem.set_u32(this + 0x60, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b6_f8d0, &args![this, CALLBACK, 9u32]);
        assert!(calls(&e, CALLBACK).is_empty());
    }

    #[test]
    fn fn_00b6f930_adds_the_water_fog_pass_once() {
        let mut e = Engine::new();
        e.register(CHECK_CREATE_WATER_FOG_PASS_ARRAY_00BA97B0, |_, _| {
            Ret::default()
        });
        e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
        let list = e.mem.alloc(0x14);
        let this = e.mem.alloc(0x88);
        e.mem.set_u32(this + 0x50, list);
        let property = e.mem.alloc(0x20);
        let geometry = e.mem.alloc(0xa0);
        let pass_for = |e: &mut Engine, flags20: u8, flags24: u8, bits: Option<u16>| {
            e.mem.set_u8(this + 0x20, flags20);
            e.mem.set_u8(this + 0x24, flags24);
            match bits {
                Some(bits) => {
                    e.mem.set_u16(property + 0x18, bits);
                    e.mem.set_u32(geometry + 0x9c, property);
                }
                None => e.mem.set_u32(geometry + 0x9c, 0),
            }
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00b6_f930, &args![this, geometry]).u32(), list);
            let added = calls(e, RENDER_PASS_ARRAY_ADD);
            assert_eq!(
                calls(e, CHECK_CREATE_WATER_FOG_PASS_ARRAY_00BA97B0),
                vec![vec![this, 1]]
            );
            assert_eq!(added.len(), 1);
            assert_eq!(&added[0][..2], &[list, geometry]);
            assert_eq!(&added[0][3..], &[0; 6]);
            added[0][2]
        };
        assert_eq!(pass_for(&mut e, 2, 0x20, Some(0x200)), 0x2ef);
        assert_eq!(pass_for(&mut e, 0, 0x20, Some(0x200)), 0x2ee);
        assert_eq!(pass_for(&mut e, 2, 0x20, Some(0x1ff)), 0x2e9);
        assert_eq!(pass_for(&mut e, 0, 0x20, None), 0x2e8);
        assert_eq!(pass_for(&mut e, 2, 0, Some(0x200)), 0x2e3);
        assert_eq!(pass_for(&mut e, 0, 0, Some(0x200)), 0x2e2);
        assert_eq!(pass_for(&mut e, 2, 0, None), 0x2dd);
        assert_eq!(pass_for(&mut e, 0, 0xdf, Some(0)), 0x2dc);
        // A list that has passes is answered as it is.
        e.mem.set_u32(list + 0x10, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_f930, &args![this, geometry]).u32(), list);
        assert!(calls(&e, RENDER_PASS_ARRAY_ADD).is_empty());
    }
}
