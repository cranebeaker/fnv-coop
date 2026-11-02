//! `NiParticle/nipsysgrowfademodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2a370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGrowFadeModifier::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x2c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_grow_fade_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x2cu32]);
    }
    this
}

// Translated from 00c2a360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGrowFadeModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26f8.
pub fn ni_p_sys_grow_fade_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26f8
}

use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysmodifier::stream_read;

layout! {
    /// `NiPSysGrowFadeModifier` (Xbox PDB), 0x2c bytes; the PC code uses the
    /// same offsets. Floats are declared as their bit patterns.
    pub struct NiPSysGrowFadeModifier: 0x2c {
        /// `m_fGrowTime` (Xbox PDB).
        0x18 m_fGrowTime: u32,
        /// `m_usGrowGeneration` (Xbox PDB).
        0x1C m_usGrowGeneration: u16,
        /// `m_fFadeTime` (Xbox PDB).
        0x20 m_fFadeTime: u32,
        /// `m_usFadeGeneration` (Xbox PDB).
        0x24 m_usFadeGeneration: u16,
        /// `m_fBaseScale` (Xbox PDB).
        0x28 m_fBaseScale: u32,
    }
}

/// The vtable of `NiPSysGrowFadeModifier` (RTTI
/// `.?AVNiPSysGrowFadeModifier@@`, `010bf0bc`).
const VTABLE: u32 = 0x010b_f0bc;
/// The smallest size `Update` gives a particle: the `float` at `010c09c0`
/// (0.0001).
const MIN_SIZE: u32 = 0x010c_09c0;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiPSysModifier`'s constructor (`00c31720`), member copy (`00c31780`)
/// and `LoadBinary` (`00c318b0`).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
/// `NiStream`'s `m_BSStreamHeader.uiVersion` (+4): from Bethesda stream
/// version 17 the base scale is stored.
const STREAM_BS_VERSION: u32 = 0x4;
const BS_VERSION_BASE_SCALE: u32 = 0x10;

// Translated from 00c2a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGrowFadeModifier::Update` (Xbox PDB): `thiscall`, `RET 8` (time,
/// not read; particle data). For each active particle: the grow factor is
/// `age / grow time` for the grow generation while the grow time is above
/// the age and not zero, else 1; the fade factor likewise
/// `(life span - age) / fade time` for the fade generation; the smaller of
/// the two (the fade factor when they compare unordered) times
/// `1 - base scale`, plus the base scale, at least the `float` at
/// `010c09c0`, becomes the particle's size. The x87 operations are done
/// instruction by instruction (the factors pass through the stack word of
/// the data argument and two locals as in the exe).
pub fn ni_p_sys_grow_fade_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysGrowFadeModifier>,
    _time: f32,
    data: Ptr<NiPSysData>,
) {
    let base = e.get(this, NiPSysGrowFadeModifier::m_fBaseScale);
    let one_minus_base = x87_load32(x87_store32(x87_sub(1.0, x87_load32(base))));
    let mut i: u16 = 0;
    if i >= e.get(data, NiPSysData::m_usActive) {
        return;
    }
    let one = 1.0f32.to_bits();
    loop {
        let k = u32::from(i);
        let info = e.get(data, NiPSysData::m_pkParticleInfo) + k * 0x1c;
        let generation = e.mem.u16(info + 0x18);
        let age = e.mem.u32(info + 0xc);
        let mut grow = one;
        let grow_time = e.get(this, NiPSysGrowFadeModifier::m_fGrowTime);
        if generation == e.get(this, NiPSysGrowFadeModifier::m_usGrowGeneration)
            && x87_load32(grow_time) > x87_load32(age)
            && !(0.0 == x87_m32(grow_time))
        {
            grow = x87_store32(x87_div(x87_load32(age), x87_m32(grow_time)));
        }
        let mut fade = one;
        let remaining = x87_store32(x87_sub(x87_load32(e.mem.u32(info + 0x10)), x87_m32(age)));
        let fade_time = e.get(this, NiPSysGrowFadeModifier::m_fFadeTime);
        if generation == e.get(this, NiPSysGrowFadeModifier::m_usFadeGeneration)
            && x87_load32(fade_time) > x87_load32(remaining)
            && !(0.0 == x87_m32(fade_time))
        {
            fade = x87_store32(x87_div(x87_load32(remaining), x87_m32(fade_time)));
        }
        // `FLD fade; FLD grow; FCOM; TEST AH,5; JP`: the grow factor when it
        // is below the fade factor.
        let smaller = if x87_load32(grow) < x87_load32(fade) {
            grow
        } else {
            fade
        };
        let smaller = x87_quiet_bits(smaller);
        let mut size = x87_store32(x87_add(
            x87_mul(x87_load32(smaller), one_minus_base),
            x87_m32(e.get(this, NiPSysGrowFadeModifier::m_fBaseScale)),
        ));
        let min_size = e.global::<u32>(MIN_SIZE);
        // `FLD size; FLD min; FCOM; TEST AH,0x41; JNZ`: the minimum replaces
        // the size only when it is above it.
        if x87_load32(min_size) > x87_load32(size) {
            size = x87_quiet_bits(min_size);
        }
        let sizes = e.get(data, NiPSysData::m_pfSizes);
        i = i.wrapping_add(1);
        e.mem.set_u32(sizes + 4 * k, x87_quiet_bits(size));
        if i >= e.get(data, NiPSysData::m_usActive) {
            break;
        }
    }
}

// Translated from 00c2a4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGrowFadeModifier::CreateClone` (Xbox PDB): a 0x2c-byte object from
/// `00aa13e0` (constructed: the `NiPSysModifier` constructor, the vtable
/// `010bf0bc`, times and base scale 0.0, generations 0; 0 when the
/// allocation fails), the `NiPSysModifier` member copy into it (it still
/// runs into a null clone), then every field (the floats through the x87
/// stack). Returns the clone.
pub fn ni_p_sys_grow_fade_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysGrowFadeModifier>,
    process: Ptr,
) -> Ptr<NiPSysGrowFadeModifier> {
    let clone = Ptr::<NiPSysGrowFadeModifier>::new(e.call(NI_NEW, &args![0x2cu32]).u32());
    if !clone.is_null() {
        e.call(MODIFIER_CONSTRUCTOR, &args![clone]);
        let zero = 0.0f32.to_bits();
        e.set(clone, NiPSysGrowFadeModifier::m_fGrowTime, zero);
        e.set(clone, NiPSysGrowFadeModifier::m_fFadeTime, zero);
        e.set(clone, NiPSysGrowFadeModifier::m_fBaseScale, zero);
        e.mem.set_u32(clone.addr(), VTABLE);
        e.set(clone, NiPSysGrowFadeModifier::m_usGrowGeneration, 0);
        e.set(clone, NiPSysGrowFadeModifier::m_usFadeGeneration, 0);
    }
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let copy_float = |e: &mut Engine, field| {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(clone, field, value);
    };
    copy_float(e, NiPSysGrowFadeModifier::m_fGrowTime);
    let grow = e.get(this, NiPSysGrowFadeModifier::m_usGrowGeneration);
    e.set(clone, NiPSysGrowFadeModifier::m_usGrowGeneration, grow);
    copy_float(e, NiPSysGrowFadeModifier::m_fFadeTime);
    let fade = e.get(this, NiPSysGrowFadeModifier::m_usFadeGeneration);
    e.set(clone, NiPSysGrowFadeModifier::m_usFadeGeneration, fade);
    copy_float(e, NiPSysGrowFadeModifier::m_fBaseScale);
    clone
}

// Translated from 00c2a530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGrowFadeModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, the grow time (4 bytes), grow generation (2), fade time
/// (4), fade generation (2), and from Bethesda stream version 17 the base
/// scale (4). The component-size word is the stream argument's stack word.
pub fn ni_p_sys_grow_fade_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysGrowFadeModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_LOAD_BINARY, &args![this, s]);
    e.with_stack(4, |e, slot| {
        for (off, size) in [
            (NiPSysGrowFadeModifier::m_fGrowTime.off, 4),
            (NiPSysGrowFadeModifier::m_usGrowGeneration.off, 2),
            (NiPSysGrowFadeModifier::m_fFadeTime.off, 4),
            (NiPSysGrowFadeModifier::m_usFadeGeneration.off, 2),
        ] {
            stream_read(e, s, this.addr() + off, size, size, slot.addr());
        }
        if e.mem.u32(s + STREAM_BS_VERSION) > BS_VERSION_BASE_SCALE {
            stream_read(
                e,
                s,
                this.addr() + NiPSysGrowFadeModifier::m_fBaseScale.off,
                4,
                4,
                slot.addr(),
            );
        }
    });
}

use crate::units::niparticle::nipsysmodifier::{
    add_class_name, add_labelled, stream_write, NI_GET_VIEWER_STRING_FLOAT,
    NI_GET_VIEWER_STRING_U16,
};

/// `NiPSysModifier::IsEqual` (`00c31830`), `SaveBinary` (`00c31950`) and
/// `GetViewerStrings` (`00c319e0`): `thiscall`, one word.
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// The `NiRTTI` record of `NiPSysGrowFadeModifier` (`012026f8`).
const RTTI: u32 = 0x0120_26f8;
/// Labels: `"Grow Time"`, `"Grow Generation"`, `"Fade Time"`,
/// `"Fade Generation"`, `"Base Scale"`.
const LABEL_GROW_TIME: u32 = 0x010b_f198;
const LABEL_GROW_GENERATION: u32 = 0x010b_f188;
const LABEL_FADE_TIME: u32 = 0x010b_f17c;
const LABEL_FADE_GENERATION: u32 = 0x010b_f16c;
const LABEL_BASE_SCALE: u32 = 0x010b_f160;

// Translated from 00c2a2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGrowFadeModifier::IsEqual` (Xbox PDB): `NiPSysModifier::IsEqual`,
/// then equal grow times (`FUCOMPP`: a NaN is unequal), grow generations,
/// fade times, base scales and fade generations, in that order.
pub fn ni_p_sys_grow_fade_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysGrowFadeModifier>,
    other: Ptr<NiPSysGrowFadeModifier>,
) -> bool {
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let same_float =
        |e: &Engine, field| x87_load32(e.get(this, field)) == x87_load32(e.get(other, field));
    same_float(e, NiPSysGrowFadeModifier::m_fGrowTime)
        && e.get(this, NiPSysGrowFadeModifier::m_usGrowGeneration)
            == e.get(other, NiPSysGrowFadeModifier::m_usGrowGeneration)
        && same_float(e, NiPSysGrowFadeModifier::m_fFadeTime)
        && same_float(e, NiPSysGrowFadeModifier::m_fBaseScale)
        && e.get(this, NiPSysGrowFadeModifier::m_usFadeGeneration)
            == e.get(other, NiPSysGrowFadeModifier::m_usFadeGeneration)
}

// Translated from 00c2a610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGrowFadeModifier::SaveBinary` (Xbox PDB): `NiPSysModifier`'s
/// `SaveBinary`, then the grow time (4 bytes), grow generation (2), fade
/// time (4), fade generation (2) and base scale (4), each write's component
/// size kept in the stack word that held the stream argument.
pub fn ni_p_sys_grow_fade_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysGrowFadeModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_SAVE_BINARY, &args![this, s]);
    e.with_stack(4, |e, argument_slot| {
        e.mem.set_u32(argument_slot.addr(), s);
        for (off, size) in [
            (NiPSysGrowFadeModifier::m_fGrowTime.off, 4),
            (NiPSysGrowFadeModifier::m_usGrowGeneration.off, 2),
            (NiPSysGrowFadeModifier::m_fFadeTime.off, 4),
            (NiPSysGrowFadeModifier::m_usFadeGeneration.off, 2),
            (NiPSysGrowFadeModifier::m_fBaseScale.off, 4),
        ] {
            stream_write(e, s, this.addr() + off, size, size, argument_slot.addr());
        }
    });
}

// Translated from 00c2a6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGrowFadeModifier::GetViewerStrings` (Xbox PDB):
/// `NiPSysModifier::GetViewerStrings`, then the class name (`NiRTTI`
/// `012026f8`), `"Grow Time"`, `"Grow Generation"`, `"Fade Time"`,
/// `"Fade Generation"` and `"Base Scale"` appended (the floats pushed
/// through `FLD`/`FSTP`, the generations zero-extended).
pub fn ni_p_sys_grow_fade_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysGrowFadeModifier>,
    strings: Ptr,
) {
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    add_class_name(e, strings, RTTI);
    let grow_time = x87_quiet_bits(e.get(this, NiPSysGrowFadeModifier::m_fGrowTime));
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_FLOAT,
        LABEL_GROW_TIME,
        grow_time,
    );
    let grow_generation = u32::from(e.get(this, NiPSysGrowFadeModifier::m_usGrowGeneration));
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_U16,
        LABEL_GROW_GENERATION,
        grow_generation,
    );
    let fade_time = x87_quiet_bits(e.get(this, NiPSysGrowFadeModifier::m_fFadeTime));
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_FLOAT,
        LABEL_FADE_TIME,
        fade_time,
    );
    let fade_generation = u32::from(e.get(this, NiPSysGrowFadeModifier::m_usFadeGeneration));
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_U16,
        LABEL_FADE_GENERATION,
        fade_generation,
    );
    let base_scale = x87_quiet_bits(e.get(this, NiPSysGrowFadeModifier::m_fBaseScale));
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_FLOAT,
        LABEL_BASE_SCALE,
        base_scale,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_a2f0,
            ni_p_sys_grow_fade_modifier_is_equal(
                Ptr<NiPSysGrowFadeModifier>,
                Ptr<NiPSysGrowFadeModifier>,
            ) -> bool
        ),
        entry!(
            0x00c2_a610,
            ni_p_sys_grow_fade_modifier_save_binary(Ptr<NiPSysGrowFadeModifier>, Ptr)
        ),
        entry!(
            0x00c2_a6e0,
            ni_p_sys_grow_fade_modifier_get_viewer_strings(Ptr<NiPSysGrowFadeModifier>, Ptr)
        ),
        entry!(
            0x00c2_a390,
            ni_p_sys_grow_fade_modifier_update(Ptr<NiPSysGrowFadeModifier>, f32, Ptr<NiPSysData>)
        ),
        entry!(
            0x00c2_a4b0,
            ni_p_sys_grow_fade_modifier_create_clone(
                Ptr<NiPSysGrowFadeModifier>,
                Ptr,
            ) -> Ptr<NiPSysGrowFadeModifier>
        ),
        entry!(
            0x00c2_a530,
            ni_p_sys_grow_fade_modifier_load_binary(Ptr<NiPSysGrowFadeModifier>, Ptr)
        ),
        entry!(0x00c2a360, ni_p_sys_grow_fade_modifier_get_r_t_t_i() -> u32),
        entry!(0x00c2a370, ni_p_sys_grow_fade_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_grow_fade_modifier_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c3_1750, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c2a370, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c2a370, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x2c]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_grow_fade_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_a360, &args![]).u32(), 0x0120_26f8);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    /// Particles given as (generation, age, life span).
    fn grow_fade(
        particles: &[(u16, f32, f32)],
    ) -> (Engine, Ptr<NiPSysGrowFadeModifier>, Ptr<NiPSysData>) {
        let mut e = Engine::new();
        e.map(MIN_SIZE & !0xfff, 0x1000);
        e.set_global(MIN_SIZE, 0.0001f32);
        let this = e.new_object::<NiPSysGrowFadeModifier>();
        e.set(this, NiPSysGrowFadeModifier::m_fGrowTime, 2.0f32.to_bits());
        e.set(this, NiPSysGrowFadeModifier::m_usGrowGeneration, 0);
        e.set(this, NiPSysGrowFadeModifier::m_fFadeTime, 4.0f32.to_bits());
        e.set(this, NiPSysGrowFadeModifier::m_usFadeGeneration, 1);
        e.set(this, NiPSysGrowFadeModifier::m_fBaseScale, 0.5f32.to_bits());
        let data = e.new_object::<NiPSysData>();
        let n = particles.len() as u32;
        let info = e.mem.alloc(0x1c * n);
        for (i, (generation, age, life)) in particles.iter().enumerate() {
            let p = info + 0x1c * i as u32;
            e.mem.set_u16(p + 0x18, *generation);
            e.mem.set_f32(p + 0xc, *age);
            e.mem.set_f32(p + 0x10, *life);
        }
        let sizes = e.mem.alloc(4 * n);
        e.set(data, NiPSysData::m_pkParticleInfo, info);
        e.set(data, NiPSysData::m_pfSizes, sizes);
        e.set(data, NiPSysData::m_usActive, n as u16);
        (e, this, data)
    }

    #[test]
    fn update_grows_young_and_fades_old_particles() {
        let (mut e, this, data) = grow_fade(&[
            (0, 1.0, 10.0),
            (1, 9.0, 10.0),
            (2, 1.0, 10.0),
            (0, 3.0, 10.0),
        ]);
        e.call(0x00c2_a390, &args![this, 0u32, data]);
        let sizes = e.get(data, NiPSysData::m_pfSizes);
        // Grow factor 0.5: 0.5 * 0.5 + 0.5.
        assert_eq!(e.mem.f32(sizes), 0.75);
        // Fade factor 1/4: 0.25 * 0.5 + 0.5.
        assert_eq!(e.mem.f32(sizes + 4), 0.625);
        // Neither generation, and an age past the grow time: full size.
        assert_eq!(e.mem.f32(sizes + 8), 1.0);
        assert_eq!(e.mem.f32(sizes + 12), 1.0);
    }

    #[test]
    fn update_keeps_the_minimum_size_and_skips_zero_times() {
        let (mut e, this, data) = grow_fade(&[(0, 0.0, 10.0)]);
        e.set(this, NiPSysGrowFadeModifier::m_fBaseScale, 0.0f32.to_bits());
        e.call(0x00c2_a390, &args![this, 0u32, data]);
        let sizes = e.get(data, NiPSysData::m_pfSizes);
        assert_eq!(e.mem.f32(sizes), 0.0001);
        // A zero grow time leaves the factor at 1 (no division by zero).
        e.set(this, NiPSysGrowFadeModifier::m_fGrowTime, 0.0f32.to_bits());
        e.call(0x00c2_a390, &args![this, 0u32, data]);
        assert_eq!(e.mem.f32(sizes), 1.0);
    }

    #[test]
    fn create_clone_constructs_and_copies() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiPSysGrowFadeModifier>();
        e.set(this, NiPSysGrowFadeModifier::m_fFadeTime, 0x7f80_0001);
        e.set(this, NiPSysGrowFadeModifier::m_usFadeGeneration, 4);
        let clone = e
            .call(0x00c2_a4b0, &args![this, 0x4000u32])
            .ptr::<NiPSysGrowFadeModifier>();
        assert_eq!(e.mem.u32(clone.addr()), VTABLE);
        assert_eq!(
            e.get(clone, NiPSysGrowFadeModifier::m_fFadeTime),
            0x7fc0_0001
        );
        assert_eq!(e.get(clone, NiPSysGrowFadeModifier::m_usFadeGeneration), 4);
    }

    #[test]
    fn load_binary_reads_the_base_scale_from_version_17() {
        for (bs_version, expected) in [(0x11u32, 3.0f32), (0x10, 0.0)] {
            let mut e = Engine::new();
            e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
            let mut bytes = 1.0f32.to_le_bytes().to_vec();
            bytes.extend_from_slice(&[5, 0]);
            bytes.extend_from_slice(&2.0f32.to_le_bytes());
            bytes.extend_from_slice(&[6, 0]);
            bytes.extend_from_slice(&3.0f32.to_le_bytes());
            let stream = test_stream::new(&mut e, 0x1402_0007, &bytes);
            e.mem.set_u32(stream + STREAM_BS_VERSION, bs_version);
            let this = e.new_object::<NiPSysGrowFadeModifier>();
            e.call(0x00c2_a530, &args![this, stream]);
            assert_eq!(e.get(this, NiPSysGrowFadeModifier::m_usGrowGeneration), 5);
            assert_eq!(e.get(this, NiPSysGrowFadeModifier::m_usFadeGeneration), 6);
            assert_eq!(
                f32::from_bits(e.get(this, NiPSysGrowFadeModifier::m_fBaseScale)),
                expected
            );
        }
    }
}

#[cfg(test)]
mod stream_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    fn modifier(e: &mut Engine) -> Ptr<NiPSysGrowFadeModifier> {
        let m = e.new_object::<NiPSysGrowFadeModifier>();
        e.set(m, NiPSysGrowFadeModifier::m_fGrowTime, 1.5f32.to_bits());
        e.set(m, NiPSysGrowFadeModifier::m_usGrowGeneration, 2);
        e.set(m, NiPSysGrowFadeModifier::m_fFadeTime, 2.5f32.to_bits());
        e.set(m, NiPSysGrowFadeModifier::m_usFadeGeneration, 3);
        e.set(m, NiPSysGrowFadeModifier::m_fBaseScale, 0.5f32.to_bits());
        m
    }

    #[test]
    fn is_equal_compares_every_field_after_the_base() {
        let mut e = Engine::new();
        e.register(MODIFIER_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let a = modifier(&mut e);
        let b = modifier(&mut e);
        assert!(e.call(0x00c2_a2f0, &args![a, b]).bool());
        for (off, value) in [
            (0x18u32, 1.25f32.to_bits()),
            (0x1c, 9),
            (0x20, 1.25f32.to_bits()),
            (0x24, 9),
            (0x28, 1.25f32.to_bits()),
        ] {
            let c = modifier(&mut e);
            if off == 0x1c || off == 0x24 {
                e.mem.set_u16(c.addr() + off, value as u16);
            } else {
                e.mem.set_u32(c.addr() + off, value);
            }
            assert!(!e.call(0x00c2_a2f0, &args![a, c]).bool(), "{off:#x}");
        }
        // A NaN grow time is unequal to itself.
        e.set(a, NiPSysGrowFadeModifier::m_fGrowTime, 0x7fc0_0000);
        e.set(b, NiPSysGrowFadeModifier::m_fGrowTime, 0x7fc0_0000);
        assert!(!e.call(0x00c2_a2f0, &args![a, b]).bool());
        // -0 equals +0.
        e.set(a, NiPSysGrowFadeModifier::m_fGrowTime, 0x8000_0000);
        e.set(b, NiPSysGrowFadeModifier::m_fGrowTime, 0);
        assert!(e.call(0x00c2_a2f0, &args![a, b]).bool());
        e.register(MODIFIER_IS_EQUAL, |_, _| Ret::default());
        assert!(!e.call(0x00c2_a2f0, &args![a, b]).bool());
    }

    #[test]
    fn save_binary_writes_the_five_fields() {
        let mut e = Engine::new();
        e.register(MODIFIER_SAVE_BINARY, |_, _| Ret::default());
        let writes = test_stream::writes(&mut e);
        let stream = test_stream::new_output(&mut e);
        let this = modifier(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_a610, &args![this, stream]);
        assert_eq!(
            calls_to(&e, MODIFIER_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(
            *writes.borrow(),
            vec![
                (1.5f32.to_le_bytes().to_vec(), 4),
                (vec![2, 0], 2),
                (2.5f32.to_le_bytes().to_vec(), 4),
                (vec![3, 0], 2),
                (0.5f32.to_le_bytes().to_vec(), 4),
            ]
        );
        assert_eq!(test_stream::output_position(&e, stream), 16);
    }

    #[test]
    fn get_viewer_strings_appends_the_fields() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_0005);
        e.register(MODIFIER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        let this = modifier(&mut e);
        e.set(this, NiPSysGrowFadeModifier::m_fBaseScale, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00c2_a6e0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, MODIFIER_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(
            *stored.borrow(),
            vec![
                0x0abc_0005,
                LABEL_GROW_TIME,
                LABEL_GROW_GENERATION,
                LABEL_FADE_TIME,
                LABEL_FADE_GENERATION,
                LABEL_BASE_SCALE
            ]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![
                vec![LABEL_GROW_TIME, 1.5f32.to_bits()],
                vec![LABEL_FADE_TIME, 2.5f32.to_bits()],
                vec![LABEL_BASE_SCALE, 0x7fc0_0001],
            ]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_U16),
            vec![
                vec![LABEL_GROW_GENERATION, 2],
                vec![LABEL_FADE_GENERATION, 3]
            ]
        );
    }
}
