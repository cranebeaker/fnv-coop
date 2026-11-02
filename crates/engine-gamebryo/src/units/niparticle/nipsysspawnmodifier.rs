//! `NiParticle/nipsysspawnmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c23ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2550.
pub fn ni_p_sys_spawn_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2550
}

use crate::units::niparticle::nipsysmodifier::stream_read;

layout! {
    /// `NiPSysSpawnModifier` (Xbox PDB), 0x34 bytes; the PC code uses the
    /// same offsets. Floats are declared as their bit patterns.
    pub struct NiPSysSpawnModifier: 0x34 {
        /// `m_usNumSpawnGenerations` (Xbox PDB).
        0x18 m_usNumSpawnGenerations: u16,
        /// `m_fPercentageSpawned` (Xbox PDB).
        0x1C m_fPercentageSpawned: u32,
        /// `m_usMinNumToSpawn` (Xbox PDB).
        0x20 m_usMinNumToSpawn: u16,
        /// `m_usMaxNumToSpawn` (Xbox PDB).
        0x22 m_usMaxNumToSpawn: u16,
        /// `m_fSpawnSpeedChaos` (Xbox PDB).
        0x24 m_fSpawnSpeedChaos: u32,
        /// `m_fSpawnDirChaos` (Xbox PDB).
        0x28 m_fSpawnDirChaos: u32,
        /// `m_fLifeSpan` (Xbox PDB).
        0x2C m_fLifeSpan: u32,
        /// `m_fLifeSpanVar` (Xbox PDB).
        0x30 m_fLifeSpanVar: u32,
    }
}

/// The vtable of `NiPSysSpawnModifier` (RTTI `.?AVNiPSysSpawnModifier@@`,
/// `010be124`).
const VTABLE: u32 = 0x010b_e124;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiPSysModifier`'s constructor (`00c31720`), member copy (`00c31780`)
/// and `LoadBinary` (`00c318b0`).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;

// Translated from 00c23d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::CreateClone` (Xbox PDB): a 0x34-byte object from
/// `00aa13e0` (constructed: the `NiPSysModifier` constructor, the vtable
/// `010be124`, one generation, 100% spawned (1.0), one to one particles,
/// chaos and life span 0.0; 0 when the allocation fails), the
/// `NiPSysModifier` member copy into it (it still runs into a null clone),
/// then every field (the floats through the x87 stack). Returns the clone.
pub fn ni_p_sys_spawn_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysSpawnModifier>,
    process: Ptr,
) -> Ptr<NiPSysSpawnModifier> {
    let clone = Ptr::<NiPSysSpawnModifier>::new(e.call(NI_NEW, &args![0x34u32]).u32());
    if !clone.is_null() {
        e.call(MODIFIER_CONSTRUCTOR, &args![clone]);
        let zero = 0.0f32.to_bits();
        e.set(
            clone,
            NiPSysSpawnModifier::m_fPercentageSpawned,
            1.0f32.to_bits(),
        );
        e.set(clone, NiPSysSpawnModifier::m_fSpawnSpeedChaos, zero);
        e.set(clone, NiPSysSpawnModifier::m_fSpawnDirChaos, zero);
        e.mem.set_u32(clone.addr(), VTABLE);
        e.set(clone, NiPSysSpawnModifier::m_fLifeSpan, zero);
        e.set(clone, NiPSysSpawnModifier::m_usNumSpawnGenerations, 1);
        e.set(clone, NiPSysSpawnModifier::m_fLifeSpanVar, zero);
        e.set(clone, NiPSysSpawnModifier::m_usMinNumToSpawn, 1);
        e.set(clone, NiPSysSpawnModifier::m_usMaxNumToSpawn, 1);
    }
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let copy_float = |e: &mut Engine, field| {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(clone, field, value);
    };
    let generations = e.get(this, NiPSysSpawnModifier::m_usNumSpawnGenerations);
    e.set(
        clone,
        NiPSysSpawnModifier::m_usNumSpawnGenerations,
        generations,
    );
    copy_float(e, NiPSysSpawnModifier::m_fPercentageSpawned);
    let min = e.get(this, NiPSysSpawnModifier::m_usMinNumToSpawn);
    e.set(clone, NiPSysSpawnModifier::m_usMinNumToSpawn, min);
    let max = e.get(this, NiPSysSpawnModifier::m_usMaxNumToSpawn);
    e.set(clone, NiPSysSpawnModifier::m_usMaxNumToSpawn, max);
    copy_float(e, NiPSysSpawnModifier::m_fSpawnSpeedChaos);
    copy_float(e, NiPSysSpawnModifier::m_fSpawnDirChaos);
    copy_float(e, NiPSysSpawnModifier::m_fLifeSpan);
    copy_float(e, NiPSysSpawnModifier::m_fLifeSpanVar);
    clone
}

// Translated from 00c247f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, then the generations (2 bytes), percentage (4), minimum
/// and maximum counts (2 each), speed and direction chaos, life span and
/// its variation (4 each). The component-size word is the stream
/// argument's stack word.
pub fn ni_p_sys_spawn_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysSpawnModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_LOAD_BINARY, &args![this, s]);
    e.with_stack(4, |e, slot| {
        for (off, size) in [
            (NiPSysSpawnModifier::m_usNumSpawnGenerations.off, 2),
            (NiPSysSpawnModifier::m_fPercentageSpawned.off, 4),
            (NiPSysSpawnModifier::m_usMinNumToSpawn.off, 2),
            (NiPSysSpawnModifier::m_usMaxNumToSpawn.off, 2),
            (NiPSysSpawnModifier::m_fSpawnSpeedChaos.off, 4),
            (NiPSysSpawnModifier::m_fSpawnDirChaos.off, 4),
            (NiPSysSpawnModifier::m_fLifeSpan.off, 4),
            (NiPSysSpawnModifier::m_fLifeSpanVar.off, 4),
        ] {
            stream_read(e, s, this.addr() + off, size, size, slot.addr());
        }
    });
}

/// `NiPSysModifier::IsEqual` (`00c31830`), `SaveBinary` (`00c31950`) and
/// `GetViewerStrings` (`00c319e0`): `thiscall`, one word each.
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// `_rand` (`00ecadb8`), `_ftol2` (`00ec62c0`, the `ST0` value as a leading
/// `f64`, popped), the square-root helper (`00ec6040`, `ST0` in and out)
/// and `_CIfmod` (`00ec913a`, `ST1` modulo `ST0`, answers in `ST0`).
const RAND: u32 = 0x00ec_adb8;
const FTOL: u32 = 0x00ec_62c0;
const SQRT: u32 = 0x00ec_6040;
const CI_FMOD: u32 = 0x00ec_913a;
/// `NiMatrix3::operator*(NiPoint3)` (`00524c40`, `thiscall`, `(result,
/// point)`), `NiMatrix3::operator*(float)` (`00a85520`, `thiscall`,
/// `(result, scale)`), `NiMatrix3::operator*(NiMatrix3)` (`0043f8d0`,
/// `thiscall`, `(result, other)`) and `00476980` (`thiscall`, three point
/// addresses: builds the matrix from them, answers it); each answers its
/// result's address.
const MATRIX_TIMES_POINT: u32 = 0x0052_4c40;
const MATRIX_TIMES_SCALAR: u32 = 0x00a8_5520;
const MATRIX_TIMES_MATRIX: u32 = 0x0043_f8d0;
const MATRIX_FROM_POINTS: u32 = 0x0047_6980;
/// `NiTransform::Invert` (`004b4880`, `thiscall`, one word: the result).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
/// `NiParticleSystem::InitializeNewParticle` (`00c1aee0`, `thiscall`, one
/// word: the new particle's index).
const INITIALIZE_NEW_PARTICLE: u32 = 0x00c1_aee0;
/// `NiPSysData::AddParticle`'s vtable slot (+0xA8): answers the new
/// particle's index in AX, or the `u16` at `010be248` (0xFFFF) when full.
const ADD_PARTICLE_SLOT: u32 = 0xa8;
const NO_PARTICLE: u32 = 0x010b_e248;
/// `NiPSysSpawnModifier::SpawnParticle`'s vtable slot (+0xA8).
const SPAWN_PARTICLE_SLOT: u32 = 0xa8;
/// `double` constants: 32767.0 (`RAND_MAX`), 1.0, π (`0101ff40`), 512.0,
/// 0.5 (`01011588`) and 0.0 (`01012060`); `float` constants: 0.5
/// (`01016248`), 2π (`011aeb34`), 0.0001 (`010c09c0`) and -1.0 (`01012054`).
const RAND_MAX_DOUBLE: u32 = 0x0102_9790;
const ONE_DOUBLE: u32 = 0x0101_2070;
const PI_DOUBLE: u32 = 0x0101_ff40;
const TABLE_SIZE_DOUBLE: u32 = 0x0102_8338;
const HALF_DOUBLE: u32 = 0x0101_1588;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const HALF_FLOAT: u32 = 0x0101_6248;
const TWO_PI_FLOAT: u32 = 0x011a_eb34;
const EPSILON_FLOAT: u32 = 0x010c_09c0;
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// The two 512-entry `float` tables read by angle index, and the identity
/// matrix (`011a9448`, nine floats).
const TABLE_A: u32 = 0x011f_52e0;
const TABLE_B: u32 = 0x011f_4ae0;
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// `NiGetViewerString` (`00a7d330`, the class name) and its `unsigned
/// short` (`00a7d490`) and `float` (`00a7d550`) overloads (`cdecl`,
/// `(name, value)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_U16: u32 = 0x00a7_d490;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// The viewer-string array's grow (`0096ad30`) and store (`0096ae90`).
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The class's `NiRTTI` record (its first word is the class name) and the
/// viewer-string names: "Num Spawn Generations", "Percentage Spawned", "Min
/// Num to Spawn", "Max Num to Spawn", "Spawn Speed Chaos", "Spawn Dir
/// Chaos", "Life Span", "Life Span Variation".
const SPAWN_MODIFIER_RTTI: u32 = 0x0120_2550;
const NAME_GENERATIONS: u32 = 0x010b_e230;
const NAME_PERCENTAGE: u32 = 0x010b_e21c;
const NAME_MIN: u32 = 0x010b_e208;
const NAME_MAX: u32 = 0x010b_e1f4;
const NAME_SPEED_CHAOS: u32 = 0x010b_e1e0;
const NAME_DIR_CHAOS: u32 = 0x010b_e1d0;
const NAME_LIFE_SPAN: u32 = 0x010b_df44;
const NAME_LIFE_SPAN_VARIATION: u32 = 0x010b_df30;
/// `NiStream`'s output stream (+0x250); its write function is at +0xC
/// (`cdecl (stream, source, size, &count, 1)`), its position at +4.
const STREAM_OUTPUT: u32 = 0x250;
/// `NiPSysModifier::m_pkTarget` (Xbox PDB, +0x10): the particle system.
const MODIFIER_TARGET: u32 = 0x10;
/// `NiParticleSystem`'s world transform (+0x68, 13 words) and model data
/// (+0xB8), as the PC code reads them.
const SYSTEM_WORLD: u32 = 0x68;
const SYSTEM_DATA: u32 = 0xb8;

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

/// `rand() / 32767.0` rounded to a `float` (`FILD`, `FDIV double`, `FSTP`).
fn random_unit(e: &mut Engine) -> u32 {
    let r = e.call(RAND, &args![]).i32();
    let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
    x87_store32(x87_div(f64::from(r), rand_max))
}

// Translated from 00c23b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::IsEqual` (Xbox PDB): the modifier comparison
/// (`00c31830`), then the generations, the percentage (float, `FUCOMPP`:
/// NaN unequal), the minimum and maximum counts, both chaos values, the life
/// span and its variation.
pub fn ni_p_sys_spawn_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysSpawnModifier>,
    other: Ptr<NiPSysSpawnModifier>,
) -> bool {
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let float_equal =
        |e: &Engine, off: u32| e.mem.f32(other.addr() + off) == e.mem.f32(this.addr() + off);
    e.get(other, NiPSysSpawnModifier::m_usNumSpawnGenerations)
        == e.get(this, NiPSysSpawnModifier::m_usNumSpawnGenerations)
        && float_equal(e, NiPSysSpawnModifier::m_fPercentageSpawned.off)
        && e.get(other, NiPSysSpawnModifier::m_usMinNumToSpawn)
            == e.get(this, NiPSysSpawnModifier::m_usMinNumToSpawn)
        && e.get(other, NiPSysSpawnModifier::m_usMaxNumToSpawn)
            == e.get(this, NiPSysSpawnModifier::m_usMaxNumToSpawn)
        && float_equal(e, NiPSysSpawnModifier::m_fSpawnSpeedChaos.off)
        && float_equal(e, NiPSysSpawnModifier::m_fSpawnDirChaos.off)
        && float_equal(e, NiPSysSpawnModifier::m_fLifeSpan.off)
        && float_equal(e, NiPSysSpawnModifier::m_fLifeSpanVar.off)
}

// Translated from 00c23c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::operator__ov2` (engine map; the body is the sum): copies the nine
/// floats of `this` into `result` (`REP MOVSD`), then adds `other`
/// element by element in place (`FLD result[0]; FADD other[0]` for the
/// first, `FLD other[i]; FADD result[i]` for the rest). Answers `result`.
pub fn ni_matrix3_add(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    for i in 0..9 {
        let value = e.mem.u32(this.addr() + 4 * i);
        e.mem.set_u32(result.addr() + 4 * i, value);
    }
    let (r, o) = (result.addr(), other.addr());
    let sum = x87_add(x87_load32(e.mem.u32(r)), x87_m32(e.mem.u32(o)));
    e.mem.set_u32(r, x87_store32(sum));
    for i in 1..9 {
        let sum = x87_add(
            x87_load32(e.mem.u32(o + 4 * i)),
            x87_m32(e.mem.u32(r + 4 * i)),
        );
        e.mem.set_u32(r + 4 * i, x87_store32(sum));
    }
    result
}

// Translated from 00c23c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::operator__ov3` (engine map; the body is the difference): copies `this`
/// into `result`, then subtracts `other` element by element in place
/// (`FLD result[i]; FSUB other[i]`). Answers `result`.
pub fn ni_matrix3_subtract(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    for i in 0..9 {
        let value = e.mem.u32(this.addr() + 4 * i);
        e.mem.set_u32(result.addr() + 4 * i, value);
    }
    let (r, o) = (result.addr(), other.addr());
    for i in 0..9 {
        let difference = x87_sub(
            x87_load32(e.mem.u32(r + 4 * i)),
            x87_m32(e.mem.u32(o + 4 * i)),
        );
        e.mem.set_u32(r + 4 * i, x87_store32(difference));
    }
    result
}

// Translated from 00c23db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::SpawnParticles` (Xbox PDB): when the generation of
/// particle `index` (the `u16` at +0x18 of its `NiParticleInfo` in the
/// system's data, +0xB8 then +0x70) is below `m_usNumSpawnGenerations` and
/// `m_fPercentageSpawned` is not below `rand()/32767` (a NaN passes), it
/// spawns `min + chop((max - min) * rand()/32767)` particles, one more when
/// the fraction (`_CIfmod` by 1.0) is above 0.5, at least one (a count of 0
/// in 16 bits becomes 1), each through the `SpawnParticle` slot (+0xA8)
/// with both floats (`FLD`/`FSTP`, quieting a signalling NaN), the index
/// word and the system. The two scratch words the exe keeps in its
/// argument slots are not observable.
pub fn ni_p_sys_spawn_modifier_spawn_particles(
    e: &mut Engine,
    this: Ptr<NiPSysSpawnModifier>,
    time: u32,
    age: u32,
    index: u32,
    system: Ptr,
) {
    let data = e.mem.u32(system.addr() + SYSTEM_DATA);
    let infos = e.mem.u32(data + 0x70);
    let generation = e
        .mem
        .u16(infos.wrapping_add((index & 0xffff) * 0x1c) + 0x18);
    if generation >= e.get(this, NiPSysSpawnModifier::m_usNumSpawnGenerations) {
        return;
    }
    let roll = random_unit(e);
    let percentage = x87_load32(e.get(this, NiPSysSpawnModifier::m_fPercentageSpawned));
    if percentage < x87_load32(roll) {
        return;
    }
    let roll = random_unit(e);
    let min = u32::from(e.get(this, NiPSysSpawnModifier::m_usMinNumToSpawn));
    let max = u32::from(e.get(this, NiPSysSpawnModifier::m_usMaxNumToSpawn));
    let range = max.wrapping_sub(min) as i32;
    let scaled = x87_store32(x87_mul(f64::from(range), x87_m32(roll)));
    let scaled = x87_load32(scaled);
    let mut extra = crate::x87::fistp_chop(scaled) as u16 as u32;
    let fraction = e.call(CI_FMOD, &args![scaled, 1.0f64]).f64();
    let fraction = x87_load32(x87_store32(fraction));
    let half = x87_m32(e.global::<u32>(HALF_FLOAT));
    if fraction > half {
        extra += 1;
    }
    let mut count = min.wrapping_add(extra) & 0xffff;
    if count == 0 {
        count = 1;
    }
    let time = x87_quiet_bits(time);
    let age = x87_quiet_bits(age);
    for _ in 0..count {
        e.vcall(
            this.addr(),
            SPAWN_PARTICLE_SLOT,
            &args![time, age, index, system],
        );
    }
}

// Translated from 00c23ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::SpawnParticle` (Xbox PDB): spawns one particle from
/// particle `index` of `system` into the modifier's target system.
///
/// - The old particle's position (vertex) and velocity are read; when
///   `system` is not the target, both are turned by `system`'s world
///   rotation and then by the inverse of the target's world transform
///   (`NiTransform::Invert`, `004b4880`; positions through `00524c40`,
///   velocities inline).
/// - The target's data adds a particle (slot +0xA8); 0xFFFF (`010be248`)
///   ends it.
/// - The speed is `|v| * (1 + speedChaos * rand()/32767)`; a random
///   direction is built from two table-indexed angles (`dirChaos *
///   rand()/32767 * π` and `2π * rand()/32767`), then turned by the
///   rotation taking the z axis to the old velocity, built from the skew
///   matrix of the velocity (Rodrigues' form through `00a85520`, `0043f8d0`
///   and the matrix sum/difference of this unit) when the cross product's
///   check value is above 0.0001 or below -0.0001, the flip matrix
///   (`00476980`) when the velocity points straight down, the identity
///   otherwise; the result times the speed is the new velocity.
/// - Age `time - age`, life span `lifeSpan + (rand()/32767 - 0.5) *
///   lifeSpanVar`, generation + 1, last update `time - age`; position,
///   colour, radius, size, rotation angle and speed and axis copied where
///   both systems have them; then `InitializeNewParticle` (`00c1aee0`).
///
/// The locals live in a frame laid out as the exe's (offsets from the stack
/// pointer after the prologue), since the matrix helpers take their
/// addresses. x87 in the exe's order and operand order.
pub fn ni_p_sys_spawn_modifier_spawn_particle(
    e: &mut Engine,
    this: Ptr<NiPSysSpawnModifier>,
    time: u32,
    age: u32,
    index: u32,
    system: Ptr,
) {
    let old = index & 0xffff;
    e.with_stack(0x1a4, |e, frame| {
        let f = frame.addr();
        let ld = |e: &Engine, off: u32| x87_load32(e.mem.u32(f + off));
        let m32 = |e: &Engine, off: u32| x87_m32(e.mem.u32(f + off));
        let put = |e: &mut Engine, off: u32, value: f64| e.mem.set_u32(f + off, x87_store32(value));
        let copy = |e: &mut Engine, to: u32, from: u32, words: u32| {
            for i in 0..words {
                let value = e.mem.u32(from + 4 * i);
                e.mem.set_u32(to + 4 * i, value);
            }
        };
        let data = e.mem.u32(system.addr() + SYSTEM_DATA);
        let old_colors = e.mem.u32(data + 0x28);
        let old_radii = e.mem.u32(data + 0x4c);
        let old_sizes = e.mem.u32(data + 0x54);
        let old_angles = e.mem.u32(data + 0x5c);
        let old_axes = e.mem.u32(data + 0x60);
        let old_speeds = e.mem.u32(data + 0x74);
        let old_vertices = e.mem.u32(data + 0x20);
        let old_info = e.mem.u32(data + 0x70).wrapping_add(old * 0x1c);
        let old_vertex = old_vertices.wrapping_add(old * 12);
        copy(e, f + 0x3c, old_vertex, 3);
        let target = e.mem.u32(this.addr() + MODIFIER_TARGET);
        copy(e, f + 0x24, old_info, 3);
        if system.addr() != target {
            copy(e, f + 0xd0, system.addr() + SYSTEM_WORLD, 13);
            let turned = e
                .call(MATRIX_TIMES_POINT, &args![f + 0xd0, f + 0x4c, f + 0x3c])
                .u32();
            copy(e, f + 0x3c, turned, 3);
            let (v0, v1, v2) = (ld(e, 0x24), ld(e, 0x28), ld(e, 0x2c));
            let r = |e: &Engine, i: u32| ld(e, 0xd0 + 4 * i);
            copy(e, f + 0x128, target + SYSTEM_WORLD, 13);
            let x = x87_add(
                x87_mul(r(e, 2), v2),
                x87_add(x87_mul(r(e, 0), v0), x87_mul(r(e, 1), v1)),
            );
            put(e, 0x18, x);
            let y = x87_add(
                x87_add(x87_mul(r(e, 4), v1), x87_mul(r(e, 3), v0)),
                x87_mul(r(e, 5), v2),
            );
            put(e, 0x1c, y);
            let z = x87_add(
                x87_add(x87_mul(v0, r(e, 6)), x87_mul(v1, r(e, 7))),
                x87_mul(v2, m32(e, 0xf0)),
            );
            put(e, 0x20, z);
            e.call(TRANSFORM_INVERT, &args![f + 0x128, f + 0xd0]);
            let turned = e
                .call(MATRIX_TIMES_POINT, &args![f + 0xd0, f + 0x4c, f + 0x3c])
                .u32();
            let (vx, vy, vz) = (ld(e, 0x18), ld(e, 0x1c), ld(e, 0x20));
            let x = x87_add(
                x87_mul(r(e, 2), vz),
                x87_add(x87_mul(r(e, 1), vy), x87_mul(vx, m32(e, 0xd0))),
            );
            copy(e, f + 0x3c, turned, 3);
            put(e, 0x18, x);
            copy(e, f + 0x24, f + 0x18, 1);
            let y = x87_add(
                x87_add(x87_mul(r(e, 3), vx), x87_mul(r(e, 4), vy)),
                x87_mul(r(e, 5), vz),
            );
            put(e, 0x1c, y);
            copy(e, f + 0x28, f + 0x1c, 1);
            let z = x87_add(
                x87_mul(vz, r(e, 8)),
                x87_add(x87_mul(vy, r(e, 7)), x87_mul(vx, r(e, 6))),
            );
            put(e, 0x20, z);
            copy(e, f + 0x2c, f + 0x20, 1);
        }
        let target = e.mem.u32(this.addr() + MODIFIER_TARGET);
        let new_data = e.mem.u32(target + SYSTEM_DATA);
        let new_index = e.vcall(new_data, ADD_PARTICLE_SLOT, &args![]).u32() & 0xffff;
        if new_index == u32::from(e.global::<u16>(NO_PARTICLE)) {
            return;
        }
        put(e, 0x34, x87_sub(x87_load32(time), x87_m32(age)));
        let new_info = e.mem.u32(new_data + 0x70).wrapping_add(new_index * 0x1c);
        let new_vertices = e.mem.u32(new_data + 0x20);
        let new_colors = e.mem.u32(new_data + 0x28);
        let new_radii = e.mem.u32(new_data + 0x4c);
        let new_sizes = e.mem.u32(new_data + 0x54);
        let new_angles = e.mem.u32(new_data + 0x5c);
        let new_axes = e.mem.u32(new_data + 0x60);
        let new_speeds = e.mem.u32(new_data + 0x74);
        let (v0, v1, v2) = (ld(e, 0x24), ld(e, 0x28), ld(e, 0x2c));
        let planar = x87_add(x87_mul(v1, v1), x87_mul(v0, v0));
        e.mem.set_u64(f + 0x7c, planar.to_bits());
        put(e, 0x30, x87_add(planar, x87_mul(v2, v2)));
        let root = e.call(SQRT, &args![ld(e, 0x30)]).f64();
        put(e, 0x30, root);
        let speed_roll = random_unit(e);
        let speed_chaos = x87_load32(e.get(this, NiPSysSpawnModifier::m_fSpawnSpeedChaos));
        let chaos = x87_store32(x87_mul(speed_chaos, x87_m32(speed_roll)));
        let one: f64 = e.global(ONE_DOUBLE);
        put(
            e,
            0x90,
            x87_mul(x87_add(x87_load32(chaos), one), m32(e, 0x30)),
        );
        let dir_roll = random_unit(e);
        let dir_chaos = x87_load32(e.get(this, NiPSysSpawnModifier::m_fSpawnDirChaos));
        let pi: f64 = e.global(PI_DOUBLE);
        put(e, 0x48, x87_mul(x87_mul(dir_chaos, x87_m32(dir_roll)), pi));
        let turn_roll = random_unit(e);
        let two_pi = x87_load32(e.global::<u32>(TWO_PI_FLOAT));
        let table_size: f64 = e.global(TABLE_SIZE_DOUBLE);
        let scale = x87_div(table_size, two_pi);
        let first = e.call(FTOL, &args![x87_mul(ld(e, 0x48), scale)]).u32() & 0x1ff;
        let a_first = x87_quiet_bits(e.global::<u32>(TABLE_A + 4 * first));
        e.mem.set_u32(f + 0x14, a_first);
        put(e, 0x48, x87_mul(two_pi, x87_load32(turn_roll)));
        let second = e.call(FTOL, &args![x87_mul(scale, m32(e, 0x48))]).u32() & 0x1ff;
        let b_second = x87_load32(e.global::<u32>(TABLE_B + 4 * second));
        let a1 = ld(e, 0x14);
        put(e, 0x18, x87_mul(b_second, a1));
        put(
            e,
            0x1c,
            x87_mul(a1, x87_m32(e.global::<u32>(TABLE_A + 4 * second))),
        );
        let b_first = x87_quiet_bits(e.global::<u32>(TABLE_B + 4 * first));
        e.mem.set_u32(f + 0x20, b_first);
        copy(e, f + 0x104, IDENTITY_MATRIX, 9);
        // The skew matrix of the old velocity at +0x58 (rows (0, 0, -x),
        // (0, 0, -y), (x, y, 0)), stored in the exe's order.
        put(e, 0x14, -v0);
        let minus_x = ld(e, 0x14);
        put(e, 0x4c, minus_x);
        put(e, 0x50, -v1);
        for off in [0x58, 0x64, 0x5c, 0x68] {
            put(e, off, 0.0);
        }
        put(e, 0x70, v0);
        put(e, 0x74, v1);
        copy(e, f + 0x60, f + 0x4c, 1);
        copy(e, f + 0x6c, f + 0x50, 1);
        put(e, 0x78, 0.0);
        put(e, 0x50, minus_x);
        // The cross product of the direction and the old velocity.
        let (dx, dy, dz) = (ld(e, 0x18), ld(e, 0x1c), ld(e, 0x20));
        put(e, 0x24, x87_sub(x87_mul(dy, v2), x87_mul(dz, v1)));
        put(e, 0x28, x87_sub(x87_mul(dz, v0), x87_mul(dx, v2)));
        put(e, 0x2c, x87_sub(x87_mul(dx, v1), x87_mul(v0, dy)));
        let zero: f64 = e.global(ZERO_DOUBLE);
        let dot = x87_add(
            x87_add(x87_mul(v1, ld(e, 0x24)), x87_mul(ld(e, 0x28), m32(e, 0x50))),
            x87_mul(ld(e, 0x2c), zero),
        );
        put(e, 0x14, dot);
        let dot = ld(e, 0x14);
        let epsilon = x87_load32(e.global::<u32>(EPSILON_FLOAT));
        let rotation = if epsilon < dot || -dot > epsilon {
            let len = ld(e, 0x30);
            put(e, 0x14, x87_div(1.0, len));
            let inverse = e.mem.u32(f + 0x14);
            let planar = f64::from_bits(e.mem.u64(f + 0x7c));
            put(e, 0x14, x87_div(x87_sub(len, v2), planar));
            let factor = e.mem.u32(f + 0x14);
            if epsilon < dot {
                let scaled = e
                    .call(MATRIX_TIMES_SCALAR, &args![f + 0x58, f + 0x180, factor])
                    .ptr::<()>();
                let sum = ni_matrix3_add(e, Ptr::new(f + 0x104), Ptr::new(f + 0x15c), scaled);
                let product = e
                    .call(MATRIX_TIMES_MATRIX, &args![f + 0x58, f + 0x128, sum])
                    .u32();
                let scaled = e
                    .call(MATRIX_TIMES_SCALAR, &args![product, f + 0x58, inverse])
                    .ptr::<()>();
                ni_matrix3_add(e, Ptr::new(f + 0x104), Ptr::new(f + 0xd0), scaled).addr()
            } else {
                let scaled = e
                    .call(MATRIX_TIMES_SCALAR, &args![f + 0x58, f + 0x128, factor])
                    .ptr::<()>();
                let difference =
                    ni_matrix3_subtract(e, Ptr::new(f + 0x104), Ptr::new(f + 0x15c), scaled);
                let product = e
                    .call(MATRIX_TIMES_MATRIX, &args![f + 0x58, f + 0x180, difference])
                    .u32();
                let scaled = e
                    .call(MATRIX_TIMES_SCALAR, &args![product, f + 0xd0, inverse])
                    .ptr::<()>();
                ni_matrix3_subtract(e, Ptr::new(f + 0x104), Ptr::new(f + 0x58), scaled).addr()
            }
        } else if v2 < 0.0 {
            let minus_one = x87_store32(x87_load32(e.global::<u32>(MINUS_ONE_FLOAT)));
            e.mem.set_u32(f + 0x4c, 0);
            e.mem.set_u32(f + 0x50, 0);
            e.mem.set_u32(f + 0x54, minus_one);
            e.mem.set_u32(f + 0x28, minus_one);
            e.mem.set_u32(f + 0x7c, minus_one);
            for off in [0x24, 0x2c, 0x80, 0x84] {
                e.mem.set_u32(f + off, 0);
            }
            e.call(
                MATRIX_FROM_POINTS,
                &args![f + 0xd0, f + 0x7c, f + 0x24, f + 0x4c],
            )
            .u32()
        } else {
            0
        };
        if rotation != 0 {
            copy(e, f + 0x104, rotation, 9);
        }
        // The rotation times the new speed, into +0x58.
        let speed = ld(e, 0x90);
        for i in 0..8 {
            put(e, 0x58 + 4 * i, x87_mul(ld(e, 0x104 + 4 * i), speed));
        }
        put(e, 0x78, x87_mul(speed, m32(e, 0x124)));
        let s = |e: &Engine, i: u32| ld(e, 0x58 + 4 * i);
        let x = x87_add(
            x87_mul(s(e, 2), dz),
            x87_add(x87_mul(s(e, 0), dx), x87_mul(s(e, 1), dy)),
        );
        put(e, 0x18, x);
        copy(e, new_info, f + 0x18, 1);
        let y = x87_add(
            x87_add(x87_mul(s(e, 4), dy), x87_mul(s(e, 3), dx)),
            x87_mul(s(e, 5), dz),
        );
        put(e, 0x1c, y);
        copy(e, new_info + 4, f + 0x1c, 1);
        let z = x87_add(
            x87_mul(dz, s(e, 8)),
            x87_add(x87_mul(dx, s(e, 6)), x87_mul(dy, s(e, 7))),
        );
        put(e, 0x20, z);
        copy(e, new_info + 8, f + 0x20, 1);
        let new_age = x87_quiet_bits(e.mem.u32(f + 0x34));
        e.mem.set_u32(new_info + 0xc, new_age);
        let life_roll = random_unit(e);
        e.mem.set_u32(f + 0x34, life_roll);
        let half: f64 = e.global(HALF_DOUBLE);
        let life_span = x87_add(
            x87_mul(
                x87_sub(x87_load32(life_roll), half),
                x87_m32(e.get(this, NiPSysSpawnModifier::m_fLifeSpanVar)),
            ),
            x87_m32(e.get(this, NiPSysSpawnModifier::m_fLifeSpan)),
        );
        e.mem.set_u32(new_info + 0x10, x87_store32(life_span));
        let generation = e.mem.u16(old_info + 0x18).wrapping_add(1);
        e.mem.set_u16(new_info + 0x18, generation);
        let row = new_index * 12;
        copy(e, new_vertices.wrapping_add(row), f + 0x3c, 3);
        if old_colors != 0 && new_colors != 0 {
            copy(
                e,
                new_colors.wrapping_add(new_index << 4),
                old_colors.wrapping_add(old << 4),
                4,
            );
        }
        let copy_float = |e: &mut Engine, to: u32, from: u32| {
            let value = x87_quiet_bits(e.mem.u32(from.wrapping_add(old * 4)));
            e.mem.set_u32(to.wrapping_add(new_index * 4), value);
        };
        if old_radii != 0 && new_radii != 0 {
            copy_float(e, new_radii, old_radii);
        }
        if old_sizes != 0 && new_sizes != 0 {
            copy_float(e, new_sizes, old_sizes);
        }
        if old_angles != 0 && new_angles != 0 {
            copy_float(e, new_angles, old_angles);
            copy_float(e, new_speeds, old_speeds);
        }
        if old_axes != 0 && new_axes != 0 {
            copy(
                e,
                new_axes.wrapping_add(row),
                old_axes.wrapping_add(old * 12),
                3,
            );
        }
        let last_update = x87_sub(x87_load32(time), x87_m32(e.mem.u32(new_info + 0xc)));
        e.mem.set_u32(new_info + 0x14, x87_store32(last_update));
        let target = e.mem.u32(this.addr() + MODIFIER_TARGET);
        e.call(INITIALIZE_NEW_PARTICLE, &args![target, new_index]);
    });
}

// Translated from 00c24920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::SaveBinary` (Xbox PDB): the modifier part
/// (`00c31950`), then the generations (2 bytes), percentage (4), minimum and
/// maximum counts (2 each), both chaos values, the life span and its
/// variation (4 each), through the stream's output; the count word is the
/// argument slot.
pub fn ni_p_sys_spawn_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysSpawnModifier>,
    stream: Ptr,
) {
    e.call(MODIFIER_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        for (off, size) in [
            (NiPSysSpawnModifier::m_usNumSpawnGenerations.off, 2u32),
            (NiPSysSpawnModifier::m_fPercentageSpawned.off, 4),
            (NiPSysSpawnModifier::m_usMinNumToSpawn.off, 2),
            (NiPSysSpawnModifier::m_usMaxNumToSpawn.off, 2),
            (NiPSysSpawnModifier::m_fSpawnSpeedChaos.off, 4),
            (NiPSysSpawnModifier::m_fSpawnDirChaos.off, 4),
            (NiPSysSpawnModifier::m_fLifeSpan.off, 4),
            (NiPSysSpawnModifier::m_fLifeSpanVar.off, 4),
        ] {
            let output = e.mem.u32(stream.addr() + STREAM_OUTPUT);
            let write = e.mem.u32(output + 0xc);
            e.mem.set_u32(count.addr(), size);
            let put = e
                .call(write, &args![output, this.addr() + off, size, count, 1u32])
                .u32();
            let position = e.mem.u32(output + 4);
            e.mem.set_u32(output + 4, position.wrapping_add(put));
        }
    });
}

// Translated from 00c24a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::GetViewerStrings` (Xbox PDB): the modifier strings
/// (`00c319e0`), then the class name and every field (counts zero-extended
/// through the `unsigned short` overload, floats pushed by `FLD`/`FSTP`,
/// quieting a signalling NaN), each appended to the array.
pub fn ni_p_sys_spawn_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysSpawnModifier>,
    strings: Ptr,
) {
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(SPAWN_MODIFIER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let count = |e: &mut Engine, name: u32, field| {
        let value = u32::from(e.get(this, field));
        e.call(NI_GET_VIEWER_STRING_U16, &args![name, value]).u32()
    };
    let float = |e: &mut Engine, name: u32, field| {
        let value = x87_quiet_bits(e.get(this, field));
        e.call(NI_GET_VIEWER_STRING_FLOAT, &args![name, value])
            .u32()
    };
    let text = count(
        e,
        NAME_GENERATIONS,
        NiPSysSpawnModifier::m_usNumSpawnGenerations,
    );
    push_viewer_string(e, strings, text);
    let text = float(
        e,
        NAME_PERCENTAGE,
        NiPSysSpawnModifier::m_fPercentageSpawned,
    );
    push_viewer_string(e, strings, text);
    let text = count(e, NAME_MIN, NiPSysSpawnModifier::m_usMinNumToSpawn);
    push_viewer_string(e, strings, text);
    let text = count(e, NAME_MAX, NiPSysSpawnModifier::m_usMaxNumToSpawn);
    push_viewer_string(e, strings, text);
    for (name, field) in [
        (NAME_SPEED_CHAOS, NiPSysSpawnModifier::m_fSpawnSpeedChaos),
        (NAME_DIR_CHAOS, NiPSysSpawnModifier::m_fSpawnDirChaos),
        (NAME_LIFE_SPAN, NiPSysSpawnModifier::m_fLifeSpan),
        (
            NAME_LIFE_SPAN_VARIATION,
            NiPSysSpawnModifier::m_fLifeSpanVar,
        ),
    ] {
        let text = float(e, name, field);
        push_viewer_string(e, strings, text);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_3b70,
            ni_p_sys_spawn_modifier_is_equal(
                Ptr<NiPSysSpawnModifier>,
                Ptr<NiPSysSpawnModifier>,
            ) -> bool
        ),
        entry!(0x00c2_3c00, ni_matrix3_add(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00c2_3c70, ni_matrix3_subtract(Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00c2_3db0,
            ni_p_sys_spawn_modifier_spawn_particles(Ptr<NiPSysSpawnModifier>, u32, u32, u32, Ptr)
        ),
        entry!(
            0x00c2_3ee0,
            ni_p_sys_spawn_modifier_spawn_particle(Ptr<NiPSysSpawnModifier>, u32, u32, u32, Ptr)
        ),
        entry!(
            0x00c2_4920,
            ni_p_sys_spawn_modifier_save_binary(Ptr<NiPSysSpawnModifier>, Ptr)
        ),
        entry!(
            0x00c2_4a50,
            ni_p_sys_spawn_modifier_get_viewer_strings(Ptr<NiPSysSpawnModifier>, Ptr)
        ),
        entry!(
            0x00c2_3d10,
            ni_p_sys_spawn_modifier_create_clone(
                Ptr<NiPSysSpawnModifier>,
                Ptr,
            ) -> Ptr<NiPSysSpawnModifier>
        ),
        entry!(
            0x00c2_47f0,
            ni_p_sys_spawn_modifier_load_binary(Ptr<NiPSysSpawnModifier>, Ptr)
        ),
        entry!(0x00c23ce0, ni_p_sys_spawn_modifier_get_r_t_t_i() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_spawn_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_3ce0, &args![]).u32(), 0x0120_2550);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    #[test]
    fn create_clone_constructs_then_copies_every_field() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiPSysSpawnModifier>();
        e.set(this, NiPSysSpawnModifier::m_usNumSpawnGenerations, 3);
        e.set(this, NiPSysSpawnModifier::m_fPercentageSpawned, 0x7f80_0001);
        e.set(this, NiPSysSpawnModifier::m_usMaxNumToSpawn, 9);
        e.set(this, NiPSysSpawnModifier::m_fLifeSpanVar, 2.0f32.to_bits());
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c2_3d10, &args![this, 0x4000u32])
            .ptr::<NiPSysSpawnModifier>();
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(
            |c| c.0 == MODIFIER_COPY_MEMBERS && c.1 == vec![this.addr(), clone.addr(), 0x4000]
        ));
        assert_eq!(e.mem.u32(clone.addr()), VTABLE);
        assert_eq!(
            e.get(clone, NiPSysSpawnModifier::m_usNumSpawnGenerations),
            3
        );
        assert_eq!(
            e.get(clone, NiPSysSpawnModifier::m_fPercentageSpawned),
            0x7fc0_0001
        );
        assert_eq!(e.get(clone, NiPSysSpawnModifier::m_usMinNumToSpawn), 0);
        assert_eq!(e.get(clone, NiPSysSpawnModifier::m_usMaxNumToSpawn), 9);
        assert_eq!(
            e.get(clone, NiPSysSpawnModifier::m_fLifeSpanVar),
            2.0f32.to_bits()
        );
    }

    #[test]
    fn create_clone_defaults_before_the_copy() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        // The copy double checks the defaults the constructor left.
        e.register(MODIFIER_COPY_MEMBERS, |e, a| {
            let clone = a[1];
            assert_eq!(e.mem.u16(clone + 0x18), 1);
            assert_eq!(e.mem.f32(clone + 0x1c), 1.0);
            assert_eq!(e.mem.u16(clone + 0x20), 1);
            assert_eq!(e.mem.u16(clone + 0x22), 1);
            Ret::default()
        });
        let this = e.new_object::<NiPSysSpawnModifier>();
        e.call(0x00c2_3d10, &args![this, 0x4000u32]);
    }

    #[test]
    fn load_binary_reads_every_field() {
        let mut e = Engine::new();
        e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        let mut bytes = vec![2u8, 0];
        bytes.extend_from_slice(&0.5f32.to_le_bytes());
        bytes.extend_from_slice(&[3, 0, 4, 0]);
        for v in [1.0f32, 2.0, 3.0, 4.0] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        let stream = test_stream::new(&mut e, 0x1402_0007, &bytes);
        let this = e.new_object::<NiPSysSpawnModifier>();
        e.call(0x00c2_47f0, &args![this, stream]);
        assert_eq!(e.get(this, NiPSysSpawnModifier::m_usNumSpawnGenerations), 2);
        assert_eq!(e.get(this, NiPSysSpawnModifier::m_usMinNumToSpawn), 3);
        assert_eq!(e.get(this, NiPSysSpawnModifier::m_usMaxNumToSpawn), 4);
        assert_eq!(
            e.get(this, NiPSysSpawnModifier::m_fLifeSpanVar),
            4.0f32.to_bits()
        );
        assert_eq!(test_stream::position(&e, stream), bytes.len() as u32);
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;
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

    fn modifier(e: &mut Engine) -> Ptr<NiPSysSpawnModifier> {
        let m = Ptr::new(e.mem.alloc(0x34));
        e.set(m, NiPSysSpawnModifier::m_usNumSpawnGenerations, 2);
        e.set(
            m,
            NiPSysSpawnModifier::m_fPercentageSpawned,
            0.5f32.to_bits(),
        );
        e.set(m, NiPSysSpawnModifier::m_usMinNumToSpawn, 1);
        e.set(m, NiPSysSpawnModifier::m_usMaxNumToSpawn, 3);
        e.set(m, NiPSysSpawnModifier::m_fSpawnSpeedChaos, 0);
        e.set(m, NiPSysSpawnModifier::m_fSpawnDirChaos, 0);
        e.set(m, NiPSysSpawnModifier::m_fLifeSpan, 10.0f32.to_bits());
        e.set(m, NiPSysSpawnModifier::m_fLifeSpanVar, 2.0f32.to_bits());
        m
    }

    #[test]
    fn is_equal_compares_every_field() {
        let mut e = Engine::new();
        let this = modifier(&mut e);
        let other = modifier(&mut e);
        e.register(MODIFIER_IS_EQUAL, |_, _| word(1));
        assert!(e.call(0x00c2_3b70, &args![this, other]).bool());
        e.set(other, NiPSysSpawnModifier::m_usMaxNumToSpawn, 4);
        assert!(!e.call(0x00c2_3b70, &args![this, other]).bool());
        e.set(other, NiPSysSpawnModifier::m_usMaxNumToSpawn, 3);
        e.set(other, NiPSysSpawnModifier::m_fLifeSpanVar, 0x7fc0_0000);
        e.set(this, NiPSysSpawnModifier::m_fLifeSpanVar, 0x7fc0_0000);
        assert!(!e.call(0x00c2_3b70, &args![this, other]).bool());
        e.register(MODIFIER_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00c2_3b70, &args![this, this]).bool());
    }

    #[test]
    fn matrix_add_and_subtract_copy_then_combine() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x24);
        let b = e.mem.alloc(0x24);
        let out = e.mem.alloc(0x24);
        for i in 0..9 {
            e.mem.set_f32(a + 4 * i, i as f32);
            e.mem.set_f32(b + 4 * i, 0.5);
        }
        e.mem.set_u32(b + 8, 0x7f80_0001);
        assert_eq!(e.call(0x00c2_3c00, &args![a, out, b]).u32(), out);
        assert_eq!(e.mem.f32(out), 0.5);
        assert_eq!(e.mem.f32(out + 0x20), 8.5);
        // FLD of the signalling NaN quiets it.
        assert_eq!(e.mem.u32(out + 8), 0x7fc0_0001);
        assert_eq!(e.call(0x00c2_3c70, &args![a, out, b]).u32(), out);
        assert_eq!(e.mem.f32(out + 4), 0.5);
        assert_eq!(e.mem.f32(out + 0x20), 7.5);
        assert_eq!(e.mem.u32(out + 8), 0x7fc0_0001);
        // The result may be the other operand: it is read after the copy.
        assert_eq!(e.call(0x00c2_3c00, &args![a, b, b]).u32(), b);
        assert_eq!(e.mem.f32(b + 4), 2.0);
    }

    fn system_with(e: &mut Engine, generation: u16) -> u32 {
        let system = e.mem.alloc(0x110);
        let data = e.mem.alloc(0x7c);
        let infos = e.mem.alloc(0x1c * 4);
        e.mem.set_u32(system + SYSTEM_DATA, data);
        e.mem.set_u32(data + 0x70, infos);
        e.mem.set_u16(infos + 0x1c + 0x18, generation);
        system
    }

    #[test]
    fn spawn_particles_spawns_a_random_count_of_particles() {
        let mut e = Engine::new();
        e.map(RAND_MAX_DOUBLE, 8);
        e.set_global(RAND_MAX_DOUBLE, 32767.0f64);
        e.map(HALF_FLOAT, 4);
        e.set_global(HALF_FLOAT, 0.5f32);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SPAWN_PARTICLE_SLOT, 0x0100_0050);
        e.register(0x0100_0050, |_, _| Ret::default());
        e.register(CI_FMOD, |_, a| Ret {
            st0: f64_arg(a, 0) % f64_arg(a, 2),
            ..Ret::default()
        });
        let this = modifier(&mut e);
        e.mem.set_u32(this.addr(), vtable);
        let system = system_with(&mut e, 1);
        // Generation 1 is below 2. Rolls: 0 passes the percentage; then
        // 32767 gives 2.0 (1 + 2 = 3 particles), 16384 a little over 1.0
        // (fraction below 0.5: 1 + 1), 26000 about 1.587 (fraction above
        // 0.5: 1 + 1 + 1); 32767 first fails the percentage.
        for (rolls, expected) in [
            (vec![0, 32767], 3usize),
            (vec![0, 16384], 2),
            (vec![0, 26000], 3),
            (vec![32767], 0),
        ] {
            let left = Rc::new(RefCell::new(rolls));
            let next = left.clone();
            e.register_double(RAND, move |_, _| word(next.borrow_mut().remove(0)));
            e.call_log = Some(vec![]);
            e.call(
                0x00c2_3db0,
                &args![this, 0x7f80_0001u32, 2.0f32, 0x0005_0001u32, system],
            );
            let spawned = calls_to(&e, 0x0100_0050);
            assert_eq!(spawned.len(), expected);
            for call in spawned {
                assert_eq!(
                    call,
                    vec![
                        this.addr(),
                        0x7fc0_0001,
                        2.0f32.to_bits(),
                        0x0005_0001,
                        system
                    ]
                );
            }
            assert!(left.borrow().is_empty());
        }
        // A generation at the limit spawns nothing and rolls nothing.
        let old = system_with(&mut e, 2);
        e.register(RAND, |_, _| panic!("no roll"));
        e.call(0x00c2_3db0, &args![this, 0u32, 0u32, 1u32, old]);
    }

    const ADD_PARTICLE: u32 = 0x0100_0060;

    fn spawn_globals(e: &mut Engine, a: f32, b: f32) {
        for (address, value) in [
            (RAND_MAX_DOUBLE, 32767.0f64),
            (ONE_DOUBLE, 1.0),
            (PI_DOUBLE, std::f64::consts::PI),
            (TABLE_SIZE_DOUBLE, 512.0),
            (HALF_DOUBLE, 0.5),
            (ZERO_DOUBLE, 0.0),
        ] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        for (address, value) in [
            (TWO_PI_FLOAT, 2.0 * std::f32::consts::PI),
            (EPSILON_FLOAT, 0.0001),
            (MINUS_ONE_FLOAT, -1.0),
        ] {
            e.map(address, 4);
            e.set_global(address, value);
        }
        e.map(NO_PARTICLE, 2);
        e.set_global(NO_PARTICLE, 0xffffu16);
        e.map(IDENTITY_MATRIX, 0x24);
        for i in 0..9u32 {
            let one = if i % 4 == 0 { 1.0f32 } else { 0.0 };
            e.set_global(IDENTITY_MATRIX + 4 * i, one);
        }
        e.map(TABLE_A, 0x800);
        e.map(TABLE_B, 0x800);
        e.set_global(TABLE_A, a);
        e.set_global(TABLE_B, b);
        e.register(RAND, |_, _| word(0));
        e.register(FTOL, |_, a| word(f64_arg(a, 0) as i32 as u32));
        e.register(SQRT, |_, a| Ret {
            st0: f64_arg(a, 0).sqrt(),
            ..Ret::default()
        });
        e.register(INITIALIZE_NEW_PARTICLE, |_, _| Ret::default());
        e.register(ADD_PARTICLE, |_, _| word(0x7777_0001));
        // Doubles: a scaled matrix is the matrix, a product is the right
        // operand, and the built matrix has the three points as rows.
        e.register(MATRIX_TIMES_SCALAR, |e, a| {
            let bytes = e.mem.bytes(a[0], 0x24);
            e.mem.write(a[1], &bytes);
            word(a[1])
        });
        e.register(MATRIX_TIMES_MATRIX, |e, a| {
            let bytes = e.mem.bytes(a[2], 0x24);
            e.mem.write(a[1], &bytes);
            word(a[1])
        });
        e.register(MATRIX_FROM_POINTS, |e, a| {
            for row in 0..3 {
                let bytes = e.mem.bytes(a[1 + row], 12);
                e.mem.write(a[0] + 12 * row as u32, &bytes);
            }
            word(a[0])
        });
    }

    /// A system that is its own target, with two particles; answers (this,
    /// system, data).
    fn spawn_setup(e: &mut Engine, velocity: [f32; 3]) -> (Ptr<NiPSysSpawnModifier>, u32, u32) {
        let this = modifier(e);
        let system = e.mem.alloc(0x110);
        let data = e.mem.alloc(0x7c);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(data, vtable);
        e.mem.set_u32(vtable + ADD_PARTICLE_SLOT, ADD_PARTICLE);
        e.mem.set_u32(system + SYSTEM_DATA, data);
        e.mem.set_u32(this.addr() + MODIFIER_TARGET, system);
        let infos = e.mem.alloc(0x1c * 2);
        e.mem.set_u32(data + 0x70, infos);
        for (i, v) in velocity.iter().enumerate() {
            e.mem.set_f32(infos + 4 * i as u32, *v);
        }
        e.mem.set_u16(infos + 0x18, 6);
        let vertices = e.mem.alloc(24);
        e.mem.set_f32(vertices, 7.0);
        e.mem.set_f32(vertices + 8, 9.0);
        e.mem.set_u32(data + 0x20, vertices);
        let colors = e.mem.alloc(32);
        e.mem.set_u32(colors, 0x1111_2222);
        e.mem.set_u32(colors + 12, 0x3333_4444);
        e.mem.set_u32(data + 0x28, colors);
        (this, system, data)
    }

    #[test]
    fn spawn_particle_turns_the_direction_by_the_rotation_to_the_velocity() {
        // (table A, table B, old velocity, new velocity)
        let cases = [
            (0.0f32, 1.0f32, [0.0f32, 0.0, 1.0], [0.0f32, 0.0, 1.0]),
            (0.0, 1.0, [0.0, 0.0, -1.0], [0.0, 0.0, -1.0]),
            (0.0, 1.0, [1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]),
            (0.0, -1.0, [1.0, 0.0, 0.0], [1.0, 0.0, -2.0]),
        ];
        for (a, b, velocity, expected) in cases {
            let mut e = Engine::new();
            spawn_globals(&mut e, a, b);
            let (this, system, data) = spawn_setup(&mut e, velocity);
            e.call_log = Some(vec![]);
            e.call(
                0x00c2_3ee0,
                &args![this, 5.0f32, 1.0f32, 0xabcd_0000u32, system],
            );
            let info = e.mem.u32(data + 0x70) + 0x1c;
            for (i, v) in expected.iter().enumerate() {
                assert_eq!(e.mem.f32(info + 4 * i as u32), *v, "{velocity:?} {i}");
            }
            assert_eq!(e.mem.f32(info + 0xc), 4.0);
            assert_eq!(e.mem.f32(info + 0x10), 9.0);
            assert_eq!(e.mem.f32(info + 0x14), 1.0);
            assert_eq!(e.mem.u16(info + 0x18), 7);
            let vertices = e.mem.u32(data + 0x20);
            assert_eq!(e.mem.f32(vertices + 12), 7.0);
            assert_eq!(e.mem.f32(vertices + 20), 9.0);
            let colors = e.mem.u32(data + 0x28);
            assert_eq!(e.mem.u32(colors + 16), 0x1111_2222);
            assert_eq!(e.mem.u32(colors + 28), 0x3333_4444);
            assert_eq!(calls_to(&e, INITIALIZE_NEW_PARTICLE), vec![vec![system, 1]]);
            let built = calls_to(&e, MATRIX_FROM_POINTS).len();
            let scaled = calls_to(&e, MATRIX_TIMES_SCALAR).len();
            match velocity[2] as i32 {
                1 => assert_eq!((built, scaled), (0, 0)),
                -1 => assert_eq!((built, scaled), (1, 0)),
                _ => assert_eq!((built, scaled), (0, 2)),
            }
        }
    }

    #[test]
    fn spawn_particle_stops_when_the_data_is_full_and_turns_between_systems() {
        let mut e = Engine::new();
        spawn_globals(&mut e, 0.0, 1.0);
        let (this, system, _) = spawn_setup(&mut e, [0.0, 0.0, 1.0]);
        e.register(ADD_PARTICLE, |_, _| word(0x1234_ffff));
        e.call_log = Some(vec![]);
        e.call(0x00c2_3ee0, &args![this, 5.0f32, 1.0f32, 0u32, system]);
        assert!(calls_to(&e, INITIALIZE_NEW_PARTICLE).is_empty());
        // Another system: positions and velocities go through both
        // transforms (identity rotations here).
        e.register(ADD_PARTICLE, |_, _| word(1));
        let other = e.mem.alloc(0x110);
        let other_data = e.mem.alloc(0x7c);
        e.mem.set_u32(other + SYSTEM_DATA, other_data);
        let infos = e.mem.alloc(0x1c);
        e.mem.set_f32(infos + 8, 1.0);
        e.mem.set_u32(other_data + 0x70, infos);
        let vertices = e.mem.alloc(12);
        e.mem.set_f32(vertices, 3.0);
        e.mem.set_u32(other_data + 0x20, vertices);
        for transform in [other + SYSTEM_WORLD, system + SYSTEM_WORLD] {
            e.mem.set_f32(transform, 1.0);
            e.mem.set_f32(transform + 0x10, 1.0);
            e.mem.set_f32(transform + 0x20, 1.0);
        }
        e.register(MATRIX_TIMES_POINT, |e, a| {
            let bytes = e.mem.bytes(a[2], 12);
            e.mem.write(a[1], &bytes);
            word(a[1])
        });
        e.register(TRANSFORM_INVERT, |e, a| {
            let bytes = e.mem.bytes(a[0], 52);
            e.mem.write(a[1], &bytes);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c2_3ee0, &args![this, 5.0f32, 1.0f32, 0u32, other]);
        assert_eq!(calls_to(&e, MATRIX_TIMES_POINT).len(), 2);
        assert_eq!(calls_to(&e, TRANSFORM_INVERT).len(), 1);
        let data = e.mem.u32(system + SYSTEM_DATA);
        let info = e.mem.u32(data + 0x70) + 0x1c;
        assert_eq!(e.mem.f32(info + 8), 1.0);
        let new_vertices = e.mem.u32(data + 0x20);
        assert_eq!(e.mem.f32(new_vertices + 12), 3.0);
    }

    #[test]
    fn save_binary_writes_every_field() {
        const WRITE: u32 = 0x0100_0040;
        let mut e = Engine::new();
        e.register(MODIFIER_SAVE_BINARY, |_, _| Ret::default());
        let sizes = Rc::new(RefCell::new(vec![]));
        let log = sizes.clone();
        e.register_double(WRITE, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            log.borrow_mut().push((a[1], a[2]));
            word(a[2])
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = modifier(&mut e);
        e.call(0x00c2_4920, &args![this, stream]);
        let t = this.addr();
        assert_eq!(
            *sizes.borrow(),
            vec![
                (t + 0x18, 2),
                (t + 0x1c, 4),
                (t + 0x20, 2),
                (t + 0x22, 2),
                (t + 0x24, 4),
                (t + 0x28, 4),
                (t + 0x2c, 4),
                (t + 0x30, 4)
            ]
        );
        assert_eq!(e.mem.u32(output + 4), 26);
    }

    #[test]
    fn get_viewer_strings_appends_the_name_and_eight_fields() {
        let mut e = Engine::new();
        e.map(SPAWN_MODIFIER_RTTI, 4);
        e.set_global(SPAWN_MODIFIER_RTTI, 0x0555_0000u32);
        e.register(MODIFIER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0000));
        e.register(NI_GET_VIEWER_STRING_U16, |_, a| word(0x6000_0000 | a[1]));
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
        let this = modifier(&mut e);
        e.set(this, NiPSysSpawnModifier::m_fSpawnDirChaos, 0x7f80_0001);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_4a50, &args![this, strings]);
        assert_eq!(
            *stored.borrow(),
            vec![
                0x7000_0000,
                0x6000_0002,
                0.5f32.to_bits(),
                0x6000_0001,
                0x6000_0003,
                0,
                0x7fc0_0001,
                10.0f32.to_bits(),
                2.0f32.to_bits(),
            ]
        );
        let names: Vec<u32> = calls_to(&e, NI_GET_VIEWER_STRING_U16)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(names, [NAME_GENERATIONS, NAME_MIN, NAME_MAX]);
    }
}
