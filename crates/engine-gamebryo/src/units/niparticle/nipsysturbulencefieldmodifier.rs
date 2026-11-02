//! `NiParticle/nipsysturbulencefieldmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c26920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysTurbulenceFieldModifier::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x3c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_turbulence_field_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x3cu32]);
    }
    this
}

// Translated from 00c273c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysTurbulenceFieldModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2698.
pub fn ni_p_sys_turbulence_field_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2698
}

use crate::units::niparticle::nipsysmodifier::stream_read;

/// `NiPSysFieldModifier`'s fields as the PC code reads them (Xbox PDB
/// names): `m_pkTarget` +0x10, `m_pkFieldObj` +0x18, `m_fMagnitude` +0x1C,
/// `m_fAttenuation` +0x20, `m_bUseMaxDistance` +0x24, `m_fMaxDistance`
/// +0x28, `m_fMaxDistanceSqr` +0x2C; this class's `m_fFrequency` +0x30,
/// `m_fFrequencyUpdateTime` +0x34 and `m_fLastUpdateTime` +0x38.
const TARGET: u32 = 0x10;
const FIELD_OBJECT: u32 = 0x18;
const MAGNITUDE: u32 = 0x1c;
const ATTENUATION: u32 = 0x20;
const USE_MAX_DISTANCE: u32 = 0x24;
const MAX_DISTANCE: u32 = 0x28;
const MAX_DISTANCE_SQUARED: u32 = 0x2c;
const FREQUENCY: u32 = 0x30;
const FREQUENCY_UPDATE_TIME: u32 = 0x34;
const LAST_UPDATE_TIME: u32 = 0x38;
/// An `NiAVObject`'s world transform (+0x68, 13 words).
const WORLD: u32 = 0x68;
/// `NiPSysData`: vertices (+0x20), active count (`u16` +0x50), particle
/// infos (+0x70, 0x1C each, the velocity first).
const DATA_VERTICES: u32 = 0x20;
const DATA_ACTIVE: u32 = 0x50;
const DATA_INFO: u32 = 0x70;
/// The class's vtable (RTTI `.?AVNiPSysTurbulenceFieldModifier@@`).
const TURBULENCE_VTABLE: u32 = 0x010b_e774;
/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiPSysFieldModifier` constructor (`00c31ff0`), member copy
/// (`00c32020`, `(clone, process)`), `IsEqual` (`00c32060`), `LoadBinary`
/// (`00c32120`) and `GetViewerStrings` (`00c322b0`), all `thiscall`.
const FIELD_CONSTRUCTOR: u32 = 0x00c3_1ff0;
const FIELD_COPY_MEMBERS: u32 = 0x00c3_2020;
const FIELD_IS_EQUAL: u32 = 0x00c3_2060;
const FIELD_LOAD_BINARY: u32 = 0x00c3_2120;
const FIELD_GET_VIEWER_STRINGS: u32 = 0x00c3_22b0;
/// `float` constants: -FLT_MAX (`01015f5c`, the bits 0xFF7FFFFF) and
/// 10000.0 (`01022958`); `double` constants: the lowest frequency
/// (`01032980`, 0.0001 as a float widened), 32767.0 (`RAND_MAX`) and 1.0.
const NO_TIME_FLOAT: u32 = 0x0101_5f5c;
const LONG_UPDATE_TIME: u32 = 0x0102_2958;
const MIN_FREQUENCY_DOUBLE: u32 = 0x0103_2980;
const RAND_MAX_DOUBLE: u32 = 0x0102_9790;
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `_rand` (`00ecadb8`) and the square-root helper (`00ec6040`, `ST0` in and
/// out).
const RAND: u32 = 0x00ec_adb8;
const SQRT: u32 = 0x00ec_6040;
/// `NiTransform::Invert` (`004b4880`, `thiscall`, the result) and the
/// transform product (`0062c250`, `thiscall`, `(result, other)`).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_TIMES_TRANSFORM: u32 = 0x0062_c250;
/// `NiGetViewerString` (`00a7d330`) and its `float` overload (`00a7d550`),
/// the viewer-string array's grow (`0096ad30`) and store (`0096ae90`), the
/// class's `NiRTTI` record and `"Frequency"` (`010be818`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
const TURBULENCE_RTTI: u32 = 0x0120_2698;
const FREQUENCY_NAME: u32 = 0x010b_e818;

/// `2 * rand() / 32767.0 - 1.0` rounded to a `float` (`FILD`, `FADD
/// ST0,ST0`, `FDIV`, `FSUB`).
fn random_signed(e: &mut Engine) -> u32 {
    let r = f64::from(e.call(RAND, &args![]).i32());
    let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
    let one: f64 = e.global(ONE_DOUBLE);
    x87_store32(x87_sub(x87_div(x87_add(r, r), rand_max), one))
}

/// The update interval for a frequency (`FCOM` with the `double` at
/// `01032980`): `1 / frequency` unless the frequency is below it (a NaN is
/// not), then 10000.0 (`01022958`).
fn update_time_for(e: &Engine, frequency: f64) -> u32 {
    let lowest: f64 = e.global(MIN_FREQUENCY_DOUBLE);
    if frequency < lowest {
        x87_quiet_bits(e.global::<u32>(LONG_UPDATE_TIME))
    } else {
        x87_store32(x87_div(1.0, frequency))
    }
}

/// The random push on one particle's velocity: three signed random values
/// times `scale` (stored), each added to the velocity. `swap_z` is the
/// operand order of the z sum (`Update` loads the push first).
fn push_randomly(e: &mut Engine, info: u32, scale: f64, [x, y, z]: [u32; 3], swap_z: bool) {
    let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
    let m = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
    let pushes = [x, y, z].map(|r| x87_store32(x87_mul(scale, x87_m32(r))));
    let vx = x87_add(ld(e, info), x87_m32(pushes[0]));
    e.mem.set_u32(info, x87_store32(vx));
    let vy = x87_add(ld(e, info + 4), x87_m32(pushes[1]));
    e.mem.set_u32(info + 4, x87_store32(vy));
    let vz = if swap_z {
        x87_add(x87_load32(pushes[2]), m(e, info + 8))
    } else {
        x87_add(ld(e, info + 8), x87_m32(pushes[2]))
    };
    e.mem.set_u32(info + 8, x87_store32(vz));
}

/// The field object's transform in the particle system's space, as the
/// update functions build it: the field object's world copied to `field`,
/// the system's to `system`, the system's inverted into `inverse` and the
/// product `inverse * field` written over `system`.
fn field_in_system_space(e: &mut Engine, this: Ptr, field: u32, system: u32, inverse: u32) {
    let field_object = e.mem.u32(this.addr() + FIELD_OBJECT);
    let target = e.mem.u32(this.addr() + TARGET);
    for i in 0..13 {
        let value = e.mem.u32(field_object + WORLD + 4 * i);
        e.mem.set_u32(field + 4 * i, value);
    }
    for i in 0..13 {
        let value = e.mem.u32(target + WORLD + 4 * i);
        e.mem.set_u32(system + 4 * i, value);
    }
    e.call(TRANSFORM_INVERT, &args![system, inverse]);
    e.call(TRANSFORM_TIMES_TRANSFORM, &args![inverse, system, field]);
}

/// A particle's offset from the field's centre (`translate`, three floats)
/// stored at `offset`, and its squared length: `(dy*dy + dx*dx) + dz*dz`,
/// the exe's order.
fn squared_offset(e: &mut Engine, vertex: u32, translate: u32, offset: u32) -> f64 {
    for k in 0..3 {
        let v = x87_sub(
            x87_load32(e.mem.u32(vertex + 4 * k)),
            x87_m32(e.mem.u32(translate + 4 * k)),
        );
        e.mem.set_u32(offset + 4 * k, x87_store32(v));
    }
    let ld = |i: u32| x87_load32(e.mem.u32(offset + 4 * i));
    let (dx, dy, dz) = (ld(0), ld(1), ld(2));
    x87_add(x87_add(x87_mul(dy, dy), x87_mul(dx, dx)), x87_mul(dz, dz))
}

// Translated from 00c27390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysTurbulenceFieldModifier::IsEqual` (Xbox PDB): the field-modifier
/// comparison (`00c32060`), then the frequencies (`FUCOMPP`: NaN unequal).
pub fn ni_p_sys_turbulence_field_modifier_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(FIELD_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    e.mem.f32(other.addr() + FREQUENCY) == e.mem.f32(this.addr() + FREQUENCY)
}

// Translated from 00c273d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the turbulence update without attenuation or
/// a maximum distance (`thiscall`, `RET 8`: a word it never reads, then the
/// particle data; `Update` calls it with the time). Every active particle's
/// velocity gets `magnitude` times three signed random values
/// (`2 * rand() / 32767 - 1`).
pub fn fn_00c273d0(e: &mut Engine, this: Ptr, _unused_1: u32, data: Ptr) {
    let d = data.addr();
    let mut index = 0u16;
    while index < e.mem.u16(d + DATA_ACTIVE) {
        let info = e
            .mem
            .u32(d + DATA_INFO)
            .wrapping_add(u32::from(index) * 0x1c);
        let x = random_signed(e);
        let y = random_signed(e);
        let z = random_signed(e);
        index = index.wrapping_add(1);
        let magnitude = x87_load32(e.mem.u32(this.addr() + MAGNITUDE));
        push_randomly(e, info, magnitude, [x, y, z], false);
    }
}

// Translated from 00c274c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysTurbulenceFieldModifier::UpateNoAttenuation` (Xbox PDB; the map's
/// spelling): `thiscall`, `RET 8` (a word it never reads, then the particle
/// data). The field's transform in the system's space; each particle whose
/// squared distance from the field's centre is not above
/// `m_fMaxDistanceSqr` (a NaN fails) gets the random push times
/// `magnitude`. Locals sit at the exe's frame offsets.
pub fn ni_p_sys_turbulence_field_modifier_upate_no_attenuation(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    data: Ptr,
) {
    e.with_stack(0xe0, |e, frame| {
        let f = frame.addr();
        field_in_system_space(e, this, f + 0x78, f + 0x44, f + 0xac);
        let d = data.addr();
        let mut index = 0u16;
        while index < e.mem.u16(d + DATA_ACTIVE) {
            let i = u32::from(index);
            let info = e.mem.u32(d + DATA_INFO).wrapping_add(i * 0x1c);
            let vertex = e.mem.u32(d + DATA_VERTICES).wrapping_add(i * 12);
            for k in 0..3 {
                let value = e.mem.u32(vertex + 4 * k);
                e.mem.set_u32(f + 0x2c + 4 * k, value);
            }
            let squared = squared_offset(e, f + 0x2c, f + 0x68, f + 0x38);
            e.mem.set_u32(f + 0x10, x87_store32(squared));
            let squared = x87_load32(e.mem.u32(f + 0x10));
            let limit = x87_load32(e.mem.u32(this.addr() + MAX_DISTANCE_SQUARED));
            if limit >= squared {
                let x = random_signed(e);
                let y = random_signed(e);
                let z = random_signed(e);
                let magnitude = x87_load32(e.mem.u32(this.addr() + MAGNITUDE));
                push_randomly(e, info, magnitude, [x, y, z], false);
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c27670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysTurbulenceFieldModifier::CreateClone` (Xbox PDB): 0x3C bytes from
/// the Ni allocator, constructed (`00c31ff0`, frequency 0.0, the vtable,
/// last update -FLT_MAX, update interval 10000.0; null when the allocation
/// fails, then used anyway as the exe does), the field members copied
/// (`00c32020`), the frequency copied (`FLD`/`FSTP`) and the interval set
/// from it (`1 / frequency`, or 10000.0 below 0.0001). Answers the clone.
pub fn ni_p_sys_turbulence_field_modifier_create_clone(
    e: &mut Engine,
    this: Ptr,
    process: Ptr,
) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x3cu32]).u32();
    if block != 0 {
        e.call(FIELD_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block + FREQUENCY, 0.0f32.to_bits());
        e.mem.set_u32(block, TURBULENCE_VTABLE);
        let no_time = x87_quiet_bits(e.global::<u32>(NO_TIME_FLOAT));
        e.mem.set_u32(block + LAST_UPDATE_TIME, no_time);
        let long = x87_quiet_bits(e.global::<u32>(LONG_UPDATE_TIME));
        e.mem.set_u32(block + FREQUENCY_UPDATE_TIME, long);
    }
    e.call(FIELD_COPY_MEMBERS, &args![this, block, process]);
    let frequency = x87_quiet_bits(e.mem.u32(this.addr() + FREQUENCY));
    e.mem.set_u32(block.wrapping_add(FREQUENCY), frequency);
    let interval = update_time_for(e, x87_load32(frequency));
    e.mem
        .set_u32(block.wrapping_add(FREQUENCY_UPDATE_TIME), interval);
    Ptr::new(block)
}

// Translated from 00c27700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysTurbulenceFieldModifier::LoadBinary` (Xbox PDB): the
/// field-modifier part (`00c32120`), then the frequency (four bytes; the
/// count word is the argument slot) and the update interval from it.
pub fn ni_p_sys_turbulence_field_modifier_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(FIELD_LOAD_BINARY, &args![this, stream]);
    let field = this.addr() + FREQUENCY;
    e.with_stack(4, |e, slot| {
        stream_read(e, stream.addr(), field, 4, 4, slot.addr());
    });
    let frequency = x87_load32(e.mem.u32(field));
    let interval = update_time_for(e, frequency);
    e.mem.set_u32(this.addr() + FREQUENCY_UPDATE_TIME, interval);
}

// Translated from 00c277b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysTurbulenceFieldModifier::Update` (Xbox PDB): runs only when the
/// last update time is after `time` (or unordered with it) or the interval
/// since it has passed (`last + interval` not above `time`); then the time
/// becomes the last update time and nothing more happens for a zero
/// magnitude, no active particles or no field object. Without attenuation
/// it hands over to `UpateNoAttenuation` (with a maximum distance) or
/// `00c273d0` (the time pushed by `FLD`/`FSTP`). Otherwise each particle
/// within the maximum distance (when used; distance through `00ec6040`, a
/// NaN fails) gets the random push times `magnitude / (1 + attenuation *
/// distance)`. Locals sit at the exe's frame offsets.
pub fn ni_p_sys_turbulence_field_modifier_update(e: &mut Engine, this: Ptr, time: u32, data: Ptr) {
    let t = this.addr();
    let d = data.addr();
    let now = x87_load32(time);
    let last = x87_load32(e.mem.u32(t + LAST_UPDATE_TIME));
    if last <= now {
        let due = x87_add(
            x87_load32(e.mem.u32(t + FREQUENCY_UPDATE_TIME)),
            x87_m32(e.mem.u32(t + LAST_UPDATE_TIME)),
        );
        if due > now {
            return;
        }
    }
    e.mem.set_u32(t + LAST_UPDATE_TIME, x87_store32(now));
    if 0.0 == x87_m32(e.mem.u32(t + MAGNITUDE)) {
        return;
    }
    if e.mem.u16(d + DATA_ACTIVE) == 0 || e.mem.u32(t + FIELD_OBJECT) == 0 {
        return;
    }
    if 0.0 == x87_m32(e.mem.u32(t + ATTENUATION)) {
        let time = x87_store32(now);
        if e.mem.u8(t + USE_MAX_DISTANCE) != 0 {
            ni_p_sys_turbulence_field_modifier_upate_no_attenuation(e, this, time, data);
        } else {
            fn_00c273d0(e, this, time, data);
        }
        return;
    }
    e.with_stack(0xd8, |e, frame| {
        let f = frame.addr();
        field_in_system_space(e, this, f + 0x70, f + 0x3c, f + 0xa4);
        let mut index = 0u16;
        while index < e.mem.u16(d + DATA_ACTIVE) {
            let i = u32::from(index);
            let info = e.mem.u32(d + DATA_INFO).wrapping_add(i * 0x1c);
            let vertex = e.mem.u32(d + DATA_VERTICES).wrapping_add(i * 12);
            for k in 0..3 {
                let value = e.mem.u32(vertex + 4 * k);
                e.mem.set_u32(f + 0x24 + 4 * k, value);
            }
            let squared = squared_offset(e, f + 0x24, f + 0x60, f + 0x30);
            e.mem.set_u32(f + 0x14, x87_store32(squared));
            let root = e.call(SQRT, &args![x87_load32(e.mem.u32(f + 0x14))]).f64();
            e.mem.set_u32(f + 0x14, x87_quiet_bits(x87_store32(root)));
            let distance = x87_load32(e.mem.u32(f + 0x14));
            let within = e.mem.u8(t + USE_MAX_DISTANCE) == 0
                || x87_load32(e.mem.u32(t + MAX_DISTANCE)) >= distance;
            if within {
                let x = random_signed(e);
                let y = random_signed(e);
                // The third value subtracts an `FLD1` 1.0 (`FSUB ST1,ST0`)
                // instead of the double at `01012070`.
                let r = f64::from(e.call(RAND, &args![]).i32());
                let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
                let z = x87_store32(x87_sub(x87_div(x87_add(r, r), rand_max), 1.0));
                let denominator = x87_add(
                    1.0,
                    x87_mul(
                        x87_load32(e.mem.u32(t + ATTENUATION)),
                        x87_m32(e.mem.u32(f + 0x14)),
                    ),
                );
                let scale = x87_div(x87_m32(e.mem.u32(t + MAGNITUDE)), denominator);
                let scale = x87_load32(x87_store32(scale));
                push_randomly(e, info, scale, [x, y, z], true);
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c27a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysTurbulenceFieldModifier::GetViewerStrings` (Xbox PDB): the
/// field-modifier strings (`00c322b0`), the class name and `"Frequency"`
/// with the frequency (`FLD`/`FSTP`, quieting a signalling NaN).
pub fn ni_p_sys_turbulence_field_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    e.call(FIELD_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(TURBULENCE_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let frequency = x87_quiet_bits(e.mem.u32(this.addr() + FREQUENCY));
    let text = e
        .call(
            NI_GET_VIEWER_STRING_FLOAT,
            &args![FREQUENCY_NAME, frequency],
        )
        .u32();
    push_viewer_string(e, strings, text);
}

/// Appends `text` to the viewer-string array (the inlined `Add`).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2_7390, ni_p_sys_turbulence_field_modifier_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c2_73d0, fn_00c273d0(Ptr, u32, Ptr)),
        entry!(
            0x00c2_74c0,
            ni_p_sys_turbulence_field_modifier_upate_no_attenuation(Ptr, u32, Ptr)
        ),
        entry!(
            0x00c2_7670,
            ni_p_sys_turbulence_field_modifier_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00c2_7700,
            ni_p_sys_turbulence_field_modifier_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c2_77b0,
            ni_p_sys_turbulence_field_modifier_update(Ptr, u32, Ptr)
        ),
        entry!(
            0x00c2_7a30,
            ni_p_sys_turbulence_field_modifier_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c273c0, ni_p_sys_turbulence_field_modifier_get_r_t_t_i() -> u32),
        entry!(0x00c26920, ni_p_sys_turbulence_field_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_turbulence_field_modifier_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00c26920, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c26920, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x3c]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_turbulence_field_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_73c0, &args![]).u32(), 0x0120_2698);
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn f64_arg(a: &[u32], i: usize) -> f64 {
        f64::from_bits(u64::from(a[i]) | u64::from(a[i + 1]) << 32)
    }

    fn read(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }

    /// Globals and doubles: every `rand()` is 32767 (a signed value of 1.0),
    /// transforms copy, the square root is real.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for (address, value) in [
            (RAND_MAX_DOUBLE, 32767.0f64),
            (ONE_DOUBLE, 1.0),
            (MIN_FREQUENCY_DOUBLE, 0.0001),
        ] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        e.map(NO_TIME_FLOAT, 4);
        e.set_global(NO_TIME_FLOAT, 0xff7f_ffffu32);
        e.map(LONG_UPDATE_TIME, 4);
        e.set_global(LONG_UPDATE_TIME, 10000.0f32);
        e.register(RAND, |_, _| word(32767));
        e.register(SQRT, |_, a| Ret {
            st0: f64_arg(a, 0).sqrt(),
            ..Ret::default()
        });
        e.register(TRANSFORM_INVERT, |e, a| {
            let bytes = e.mem.bytes(a[0], 52);
            e.mem.write(a[1], &bytes);
            Ret::default()
        });
        e.register(TRANSFORM_TIMES_TRANSFORM, |e, a| {
            let bytes = e.mem.bytes(a[2], 52);
            e.mem.write(a[1], &bytes);
            word(a[1])
        });
        e
    }

    /// A modifier with magnitude 2 and identity transforms, and data with
    /// one particle at `vertex`; answers (modifier, data, info).
    fn setup(e: &mut Engine, vertex: [f32; 3]) -> (Ptr, Ptr, u32) {
        let this = e.mem.alloc(0x3c);
        e.mem.set_f32(this + MAGNITUDE, 2.0);
        for owner in [FIELD_OBJECT, TARGET] {
            let object = e.mem.alloc(0x100);
            for i in [0u32, 4, 8] {
                e.mem.set_f32(object + WORLD + i * 4, 1.0);
            }
            e.mem.set_u32(this + owner, object);
        }
        let data = e.mem.alloc(0x7c);
        e.mem.set_u16(data + DATA_ACTIVE, 1);
        let info = e.mem.alloc(0x1c);
        e.mem.set_u32(data + DATA_INFO, info);
        let vertices = e.mem.alloc(12);
        for (i, v) in vertex.iter().enumerate() {
            e.mem.set_f32(vertices + 4 * i as u32, *v);
        }
        e.mem.set_u32(data + DATA_VERTICES, vertices);
        (Ptr::new(this), Ptr::new(data), info)
    }

    #[test]
    fn is_equal_compares_the_frequency() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x3c);
        let other = e.mem.alloc(0x3c);
        e.register(FIELD_IS_EQUAL, |_, _| word(1));
        assert!(e.call(0x00c2_7390, &args![this, other]).bool());
        e.mem.set_f32(other + FREQUENCY, 1.0);
        assert!(!e.call(0x00c2_7390, &args![this, other]).bool());
        e.register(FIELD_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00c2_7390, &args![this, this]).bool());
    }

    #[test]
    fn fn_00c273d0_pushes_every_particle_by_the_magnitude() {
        let mut e = engine();
        let (this, data, info) = setup(&mut e, [0.0; 3]);
        e.mem.set_f32(info + 8, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_73d0, &args![this, 0u32, data]);
        assert_eq!(read(&e, info), [2.0, 2.0, 3.0]);
        assert_eq!(calls_to(&e, RAND).len(), 3);
    }

    #[test]
    fn upate_no_attenuation_pushes_within_the_squared_maximum() {
        for (limit, pushed) in [(9.0f32, true), (8.0, false)] {
            let mut e = engine();
            let (this, data, info) = setup(&mut e, [3.0, 0.0, 0.0]);
            e.mem.set_f32(this.addr() + MAX_DISTANCE_SQUARED, limit);
            e.call(0x00c2_74c0, &args![this, 0u32, data]);
            let expected = if pushed { [2.0; 3] } else { [0.0; 3] };
            assert_eq!(read(&e, info), expected);
        }
    }

    #[test]
    fn create_clone_copies_the_frequency_and_sets_the_interval() {
        for (frequency, interval) in [(0.5f32, 2.0f32), (0.0, 10000.0)] {
            let mut e = engine();
            e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(FIELD_CONSTRUCTOR, |_, _| Ret::default());
            e.register(FIELD_COPY_MEMBERS, |_, _| Ret::default());
            let this = e.mem.alloc(0x3c);
            e.mem.set_f32(this + FREQUENCY, frequency);
            let clone = e.call(0x00c2_7670, &args![this, 0x7000u32]).u32();
            assert_eq!(e.mem.u32(clone), TURBULENCE_VTABLE);
            assert_eq!(e.mem.f32(clone + FREQUENCY), frequency);
            assert_eq!(e.mem.f32(clone + FREQUENCY_UPDATE_TIME), interval);
            assert_eq!(e.mem.u32(clone + LAST_UPDATE_TIME), 0xff7f_ffff);
        }
    }

    #[test]
    fn load_binary_reads_the_frequency_and_sets_the_interval() {
        let mut e = engine();
        e.register(FIELD_LOAD_BINARY, |_, _| Ret::default());
        let stream = test_stream::new(&mut e, 0x1400_0002, &4.0f32.to_le_bytes());
        let this = e.mem.alloc(0x3c);
        e.call(0x00c2_7700, &args![this, stream]);
        assert_eq!(e.mem.f32(this + FREQUENCY), 4.0);
        assert_eq!(e.mem.f32(this + FREQUENCY_UPDATE_TIME), 0.25);
    }

    #[test]
    fn update_waits_for_the_interval_then_pushes() {
        let mut e = engine();
        let (this, data, info) = setup(&mut e, [3.0, 4.0, 0.0]);
        let t = this.addr();
        e.mem.set_f32(t + LAST_UPDATE_TIME, 1.0);
        e.mem.set_f32(t + FREQUENCY_UPDATE_TIME, 0.5);
        // Not due yet (1.0 + 0.5 > 1.2): nothing changes.
        e.call(0x00c2_77b0, &args![this, 1.2f32, data]);
        assert_eq!(e.mem.f32(t + LAST_UPDATE_TIME), 1.0);
        assert_eq!(read(&e, info), [0.0; 3]);
        // Due, no attenuation: `00c273d0` pushes by the magnitude.
        e.call(0x00c2_77b0, &args![this, 1.5f32, data]);
        assert_eq!(e.mem.f32(t + LAST_UPDATE_TIME), 1.5);
        assert_eq!(read(&e, info), [2.0; 3]);
        // A last time after the time always updates; with attenuation 0.2
        // the push is 2 / (1 + 0.2 * 5) = 1.
        e.mem.set_f32(t + ATTENUATION, 0.2);
        e.call(0x00c2_77b0, &args![this, 0.5f32, data]);
        assert_eq!(e.mem.f32(t + LAST_UPDATE_TIME), 0.5);
        assert_eq!(read(&e, info), [3.0; 3]);
        // Beyond the maximum distance: no push.
        e.mem.set_u8(t + USE_MAX_DISTANCE, 1);
        e.mem.set_f32(t + MAX_DISTANCE, 4.0);
        e.call(0x00c2_77b0, &args![this, 0.0f32, data]);
        assert_eq!(read(&e, info), [3.0; 3]);
        // Magnitude 0: only the time is kept.
        e.mem.set_f32(t + MAGNITUDE, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_77b0, &args![this, -1.0f32, data]);
        assert_eq!(e.mem.f32(t + LAST_UPDATE_TIME), -1.0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn get_viewer_strings_appends_the_name_and_the_frequency() {
        let mut e = Engine::new();
        e.map(TURBULENCE_RTTI, 4);
        e.set_global(TURBULENCE_RTTI, 0x0555_0000u32);
        e.register(FIELD_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0001));
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, a| word(a[1]));
        e.register(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let stored = Rc::new(RefCell::new(vec![]));
        let seen = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[2]));
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this + FREQUENCY, 0x7f80_0001);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_7a30, &args![this, strings]);
        assert_eq!(*stored.borrow(), vec![0x7000_0001, 0x7fc0_0001]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT)[0][0],
            FREQUENCY_NAME
        );
    }
}
